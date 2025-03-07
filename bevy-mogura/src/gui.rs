use crate::*;
use bevy_trackball::prelude::*;

#[derive(Clone)]
pub struct MoguraGuiPlugins;

impl Plugin for MoguraGuiPlugins {
    fn build(&self, app: &mut App) {
        app.init_resource::<OccupiedScreenSpace>()
            .add_plugins(bevy_egui::EguiPlugin)
            .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
            .add_systems(
                PreUpdate,
                (absorb_egui_inputs)
                    .after(bevy_egui::systems::process_input_system)
                    .before(bevy_egui::EguiSet::BeginPass),
            )
            .add_systems(Update, poll_rfd_structure)
            .add_systems(Update, poll_rfd_trajectory)
            .add_systems(Update, poll_downloadpdb)
            .add_systems(Update, update_gui);
    }
}

#[derive(Default, Resource)]
pub struct OccupiedScreenSpace {
    left: f32,
    top: f32,
    right: f32,
    #[allow(unused)]
    bottom: f32,
}

// path: String, content: String
#[derive(Component)]
pub struct SelectedStructureFile(bevy::tasks::Task<Option<(String, String)>>);

// path: String
#[derive(Component)]
pub struct SelectedTrajectoryFile(bevy::tasks::Task<Option<String>>);

fn poll_rfd_trajectory(
    mut commands: Commands,
    mut tasks: Query<(Entity, &mut SelectedTrajectoryFile)>,
    mut mogura_state: ResMut<MoguraState>,
) {
    for (entity, mut selected_file) in tasks.iter_mut() {
        if let Some(result) = bevy::tasks::futures_lite::future::block_on(
            bevy::tasks::futures_lite::future::poll_once(&mut selected_file.0),
        ) {
            commands.entity(entity).despawn_recursive();

            let path = if let Some(result) = result {
                result
            } else {
                return;
            };

            #[cfg(not(target_arch = "wasm32"))]
            {
                if mogura_state.structure_file.is_none() {
                    return;
                }
                let trajectory_data =
                    trajectory_loader(mogura_state.structure_file.as_ref().unwrap(), &path);
                match trajectory_data {
                    Ok(trajectory_data) => {
                        mogura_state.trajectory_data = Some(trajectory_data);
                        mogura_state.logs.push("Trajectory file loaded".to_string());
                    }
                    Err(e) => {
                        mogura_state.logs.push(e.to_string());
                    }
                }
            }

            #[cfg(target_arch = "wasm32")]
            {
                let _extension = std::path::Path::new(&path)
                    .extension()
                    .unwrap()
                    .to_str()
                    .unwrap();
            }

            mogura_state.trajectory_file = Some(path);
        }
    }
}

fn poll_rfd_structure(
    mut commands: Commands,
    mut tasks: Query<(Entity, &mut SelectedStructureFile)>,
    mut mogura_state: ResMut<MoguraState>,
    mut mogura_selections: ResMut<MoguraSelections>,
) {
    for (entity, mut selected_file) in tasks.iter_mut() {
        if let Some(result) = bevy::tasks::futures_lite::future::block_on(
            bevy::tasks::futures_lite::future::poll_once(&mut selected_file.0),
        ) {
            commands.entity(entity).despawn_recursive();

            #[allow(unused_variables)]
            let (path, content) = if let Some(result) = result {
                result
            } else {
                return;
            };

            #[cfg(not(target_arch = "wasm32"))]
            {
                match structure_loader(&path) {
                    Ok(structure_data) => {
                        mogura_state.structure_data = Some(structure_data);
                        mogura_state
                            .logs
                            .push("Structure file loaded from path".to_string());
                    }
                    Err(e) => {
                        mogura_state.logs.push(e.to_string());
                    }
                }
            }

            #[cfg(target_arch = "wasm32")]
            {
                let extension = std::path::Path::new(&path)
                    .extension()
                    .unwrap()
                    .to_str()
                    .unwrap();

                match structure_loader_from_content(&content, &extension) {
                    Ok(structure_data) => {
                        mogura_state.structure_data = Some(structure_data);
                        mogura_state
                            .logs
                            .push("Structure file loaded from content".to_string());
                    }
                    Err(e) => {
                        mogura_state.logs.push(e.to_string());
                    }
                }
            }

            mogura_state.structure_file = Some(path);
            mogura_state.init_look_at = true;
            if mogura_selections.0.is_empty() {
                mogura_selections.0.push(EachSelection::default());
            }
            mogura_selections.0[0].redraw = true;
        }
    }
}

fn poll_downloadpdb(
    mut commands: Commands,
    mut tasks: Query<(Entity, &mut DownloadPDB)>,
    mut mogura_state: ResMut<MoguraState>,
    mut mogura_selections: ResMut<MoguraSelections>,
) {
    for (entity, mut downloadded_pdb) in tasks.iter_mut() {
        if let Some(result) = bevy::tasks::futures_lite::future::block_on(
            bevy::tasks::futures_lite::future::poll_once(&mut downloadded_pdb.0),
        ) {
            commands.entity(entity).despawn_recursive();

            if let Ok(structure_data) = result {
                if mogura_selections.0.is_empty() {
                    mogura_selections.0.push(EachSelection::default());
                }
                mogura_selections.0[0].redraw = true;
                mogura_state.structure_data = Some(Box::new(structure_data));
                mogura_state.structure_file = None;
                mogura_state.init_look_at = true;
                mogura_state
                    .logs
                    .push("Structure file downloaded".to_string());
            } else {
                mogura_state.logs.push("Structure file download failed\nPDB ID may not correct. Or some PDB files have dirty format so pdbtbx could not parse".to_string());
            }
        }
    }
}

#[derive(Component)]
pub struct DownloadPDB(bevy::tasks::Task<Result<PDBData, anyhow::Error>>);

#[allow(clippy::too_many_arguments)]
fn update_gui(
    mut commands: Commands,
    mut contexts: bevy_egui::EguiContexts,
    mut occupied_screen_space: ResMut<OccupiedScreenSpace>,
    mut mogura_state: ResMut<MoguraState>,
    mut mogura_selections: ResMut<MoguraSelections>,
    mut target_pdbid: Local<String>,
    mut trackball_camera: Query<&mut TrackballCamera, With<Camera>>,
    mut open_help_window: Local<bool>,
    diagnostics: Res<bevy::diagnostic::DiagnosticsStore>,
) {
    let ctx = contexts.ctx_mut();
    let task_pool = bevy::tasks::AsyncComputeTaskPool::get();

    occupied_screen_space.left = egui::SidePanel::left("left")
        .resizable(true)
        .show(ctx, |ui| {
            ui.label("Controlpanel");
            ui.separator();
            egui::ScrollArea::vertical().show(ui, |ui| {
                let _response = egui::TextEdit::singleline(&mut *target_pdbid)
                    .hint_text("PDB ID here. e.g. 8GNG")
                    .show(ui);

                if ui.button("Start to download").clicked() {
                    let target_pdbid_clone = target_pdbid.clone();
                    let task = task_pool
                        .spawn(async move { PDBData::download(&target_pdbid_clone).await });
                    commands.spawn(DownloadPDB(task));
                }

                ui.separator();

                if let Some(top_file) = &mogura_state.structure_file {
                    ui.label(format!("File: {}", top_file));
                } else {
                    ui.label("File: None");
                }

                ui.horizontal(|ui| {
                    if ui.button("Select Structure File").clicked() {
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            let task = task_pool.spawn(async move {
                                if let Some(path) = rfd::FileDialog::new().pick_file() {
                                    let content =
                                        std::fs::read_to_string(path.display().to_string())
                                            .unwrap();
                                    Some((path.display().to_string(), content))
                                } else {
                                    None
                                }
                            });
                            commands.spawn(SelectedStructureFile(task));
                        }
                        #[cfg(target_arch = "wasm32")]
                        {
                            let task = task_pool.spawn(async move {
                                let path = rfd::AsyncFileDialog::new().pick_file().await;
                                if let Some(path) = path {
                                    let content = path.read().await;
                                    let content_str = String::from_utf8(content).unwrap();
                                    Some((path.file_name(), content_str))
                                } else {
                                    None
                                }
                            });
                            commands.spawn(SelectedStructureFile(task));
                        }
                    }

                    if ui.button("Clear").clicked() {
                        mogura_state.structure_file = None;
                        mogura_state.structure_data = None;
                        mogura_state.logs.push("Structure file cleared".to_string());
                    }
                });

                ui.separator();

                if let Some(traj_file) = &mogura_state.trajectory_file {
                    ui.label(format!("File: {}", traj_file));
                } else {
                    ui.label("File: None");
                }

                ui.horizontal(|ui| {
                    if ui.button("Select Trajectory File").clicked() {
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            let task = task_pool.spawn(async move {
                                rfd::FileDialog::new()
                                    .pick_file()
                                    .map(|path| path.display().to_string())
                            });
                            commands.spawn(SelectedTrajectoryFile(task));
                        }
                        #[cfg(target_arch = "wasm32")]
                        {
                            // let task = task_pool.spawn(async move {
                            //     let path = rfd::AsyncFileDialog::new().pick_file().await;
                            //     if let Some(path) = path {
                            //         let content = path.read().await;
                            //         let content_str = String::from_utf8(content).unwrap();
                            //         Some((path.file_name(), content_str))
                            //     } else {
                            //         None
                            //     }
                            // });
                            // commands.spawn(SelectedTrajectoryFile(task));
                        }
                    }

                    if ui.button("Clear").clicked() {
                        mogura_state.trajectory_file = None;
                        mogura_state.trajectory_data = None;
                        mogura_state
                            .logs
                            .push("Trajectory file cleared".to_string());
                    }
                });
                ui.separator();

                ui.label("Looking at Center of Structure");
                if ui.button("Look").clicked() {
                    if let Some(structure_data) = &mogura_state.structure_data {
                        let center = structure_data.center();
                        let center_vec = Vec3::new(center[0], center[1], center[2]);
                        let mut trackball_camera = trackball_camera.single_mut();
                        trackball_camera.frame.set_target(center_vec.into());
                    }
                }

                ui.separator();

                ui.label("Trajectory Control");
                ui.horizontal(|ui| {
                    if ui.button("Start").clicked() {
                        mogura_state.update_trajectory = true;
                    }

                    if ui.button("Stop").clicked() {
                        mogura_state.update_trajectory = false;
                        mogura_state.update_tmp_trajectory = false;
                        mogura_state.loop_trajectory = false;
                    }

                    if ui.button("Loop").clicked() {
                        mogura_state.loop_trajectory = !mogura_state.loop_trajectory;
                    }
                });
                if let Some(n_frame) = mogura_state.n_frame() {
                    ui.add(
                        egui::Slider::new(&mut mogura_state.current_frame_id, 0..=n_frame - 1)
                            .text(format!(" / {} frame", n_frame - 1)),
                    );
                    mogura_state.update_tmp_trajectory = true;
                } else {
                    ui.add(egui::Slider::new(&mut 0, 0..=0).text(format!(" / {} frame", 0)));
                }

                ui.separator();

                if let Some(value) = diagnostics
                    .get(&bevy::diagnostic::FrameTimeDiagnosticsPlugin::FPS)
                    .and_then(|fps| fps.smoothed())
                {
                    ui.label(format!("FPS: {:.2}", value));
                } else {
                    ui.label("FPS: None");
                }

                ui.separator();

                ui.label("Help");
                if ui.button("Help").clicked() {
                    *open_help_window = !*open_help_window;
                }

                egui::Window::new("Help Window")
                    .open(&mut open_help_window)
                    .vscroll(true)
                    .hscroll(true)
                    .resizable(true)
                    .title_bar(true)
                    .collapsible(true)
                    .show(ctx, |ui| {
                        use egui::special_emojis::GITHUB;
                        ui.hyperlink_to(format!("{GITHUB} Github"), "https://github.com/mogura-rs");
                    });
            });

            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .width();

    occupied_screen_space.right = egui::SidePanel::right("right")
        .resizable(true)
        .show(ctx, |ui| {
            ui.label("Selection panel");
            ui.separator();
            egui::ScrollArea::vertical().show(ui, |ui| {
                if let Some(structure_data) = &mogura_state.structure_data {
                    for selection in mogura_selections.0.iter_mut() {
                        let _response = egui::TextEdit::singleline(&mut selection.atom_selection)
                            .hint_text("protein")
                            .show(ui);
                        if ui.button("Apply").clicked() {
                            let selection_result = selection.apply_selection(structure_data);
                            match selection_result {
                                Ok(_) => {
                                    selection.redraw = true;
                                }
                                Err(e) => {
                                    ui.label(format!("Error: {}", e));
                                }
                            }
                        }

                        ui.label("Select Drawing Method");
                        let pre_drawing_method = selection.drawing_method;
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            // NOTE
                            // webgpu cannot draw line as PolygonMode::Line
                            // if "webgpu" in bevy's features
                            // but webgl2 can draw
                            ui.radio_value(
                                &mut selection.drawing_method,
                                DrawingMethod::Line,
                                "Line",
                            );
                        }
                        ui.radio_value(
                            &mut selection.drawing_method,
                            DrawingMethod::BallAndStick,
                            "BallAndStick",
                        );
                        ui.radio_value(&mut selection.drawing_method, DrawingMethod::Ball, "Ball");
                        ui.radio_value(
                            &mut selection.drawing_method,
                            DrawingMethod::Stick,
                            "Stick",
                        );
                        ui.radio_value(&mut selection.drawing_method, DrawingMethod::Tube, "Tube");
                        // ui.radio_value(&mut mogura_state.drawingMethod, DrawingMethod::NewCartoon, "NewCartoon");
                        // ui.radio_value(&mut mogura_state.drawingMethod, DrawingMethod::NewCartoon, "NewCartoon");
                        if pre_drawing_method != selection.drawing_method {
                            selection.redraw = true;
                        }

                        if ui.button("Clear").clicked() {
                            selection.delete = true;
                        }

                        ui.separator();
                    }

                    if ui.button("+ Add New Selection").clicked() {
                        mogura_selections.0.push(EachSelection::default());
                    }
                }
            });

            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .width();

    occupied_screen_space.top = egui::TopBottomPanel::top("top")
        // .resizable(true)
        .show(ctx, |ui| {
            ui.label("Visualization panel");
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .height();

    occupied_screen_space.bottom = egui::TopBottomPanel::bottom("bottom")
        .max_height(90.0)
        .show(ctx, |ui| {
            ui.label("Log panel (the higher the newest)");
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.separator();
                for log in mogura_state.logs.iter().rev() {
                    ui.label(log);
                }
            });
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .height();
}

// https://github.com/vladbat00/bevy_egui/blob/main/examples/side_panel.rs
// pub fn update_camera_window_transform(
//     occupied_screen_space: Res<OccupiedScreenSpace>,
//     original_camera_transform: Res<OriginalCameraTransform>,
//     windows: Query<&Window, With<bevy::window::PrimaryWindow>>,
//     mut camera_query: Query<(&Projection, &mut Transform)>,
// ) {
//     let (camera_projection, mut transform) = match camera_query.get_single_mut() {
//         Ok((Projection::Perspective(projection), transform)) => (projection, transform),
//         _ => unreachable!(),
//     };
//
//     let distance_to_target = (CAMERA_TARGET - original_camera_transform.translation).length();
//     let frustum_height = 2.0 * distance_to_target * (camera_projection.fov * 0.5).tan();
//     let frustum_width = frustum_height * camera_projection.aspect_ratio;
//
//     let window = windows.single();
//
//     let left_taken = occupied_screen_space.left / window.width();
//     let right_taken = occupied_screen_space.right / window.width();
//     let top_taken = occupied_screen_space.top / window.height();
//     let bottom_taken = occupied_screen_space.bottom / window.height();
//     transform.translation = original_camera_transform.translation
//         + transform.rotation.mul_vec3(Vec3::new(
//             (right_taken - left_taken) * frustum_width * 0.5,
//             (top_taken - bottom_taken) * frustum_height * 0.5,
//             0.0,
//         ));
// }

// ref: https://github.com/vladbat00/bevy_egui/issues/47
fn absorb_egui_inputs(
    mut contexts: bevy_egui::EguiContexts,
    mut mouse: ResMut<ButtonInput<MouseButton>>,
    mut mouse_wheel: ResMut<Events<bevy::input::mouse::MouseWheel>>,
    mut keyboard: ResMut<ButtonInput<KeyCode>>,
) {
    let ctx = contexts.ctx_mut();
    if !(ctx.wants_pointer_input() || ctx.is_pointer_over_area()) {
        return;
    }

    let modifiers = [
        KeyCode::SuperLeft,
        KeyCode::SuperRight,
        KeyCode::ControlLeft,
        KeyCode::ControlRight,
        KeyCode::AltLeft,
        KeyCode::AltRight,
        KeyCode::ShiftLeft,
        KeyCode::ShiftRight,
    ];

    let pressed = modifiers.map(|key| keyboard.pressed(key).then_some(key));

    mouse.reset_all();
    mouse_wheel.clear();
    keyboard.reset_all();

    for key in pressed.into_iter().flatten() {
        keyboard.pressed(key);
    }
}
