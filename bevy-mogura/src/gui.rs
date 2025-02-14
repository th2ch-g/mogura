use crate::camera;
use crate::*;
use crate::structure::*;
use bevy::prelude::*;
use mogura_io::prelude::*;
use bevy_trackball::prelude::*;

#[derive(Default, Resource)]
pub struct OccupiedScreenSpace {
    left: f32,
    top: f32,
    right: f32,
    bottom: f32,
}

#[derive(Component)]
pub struct SelectedFile(bevy::tasks::Task<Option<(String, String)>>);

pub fn poll_rfd(
    mut commands: Commands,
    mut tasks: Query<(Entity, &mut SelectedFile)>,
    mut mogura_state: ResMut<MoguraState>,
) {
    for (entity, mut selected_file) in tasks.iter_mut() {
        if let Some(result) = bevy::tasks::futures_lite::future::block_on(
            bevy::tasks::futures_lite::future::poll_once(&mut selected_file.0),
        ) {
            commands.entity(entity).despawn_recursive();

            let (path, content) = if let Some(result) = result {
                result
            } else {
                return;
            };

            #[cfg(not(target_arch = "wasm32"))]
            {
                mogura_state.structure_data = Some(structure_loader(&path));
                dbg!("ok");
            }

            #[cfg(target_arch = "wasm32")]
            {
                let extension = std::path::Path::new(&path)
                    .extension()
                    .unwrap()
                    .to_str()
                    .unwrap();
                mogura_state.structure_data =
                    Some(structure_loader_from_content(&content, &extension));
            }

            mogura_state.structure_file = Some(path);
            mogura_state.redraw = true;
        }
    }
}

pub fn poll_downloadpdb(
    mut commands: Commands,
    mut tasks: Query<(Entity, &mut DownloadPDB)>,
    mut mogura_state: ResMut<MoguraState>,
) {
    for (entity, mut downloadded_pdb) in tasks.iter_mut() {
        if let Some(result) = bevy::tasks::futures_lite::future::block_on(
            bevy::tasks::futures_lite::future::poll_once(&mut downloadded_pdb.0),
        ) {
            commands.entity(entity).despawn_recursive();

            if let Ok(structure_data) = result {
                mogura_state.redraw = true;
                mogura_state.structure_data = Some(Box::new(structure_data));
                mogura_state.structure_file = None;
            }
        }
    }
}

#[derive(Component)]
pub struct DownloadPDB(bevy::tasks::Task<Result<PDBData, anyhow::Error>>);

pub fn update_gui(
    mut commands: Commands,
    mut contexts: bevy_egui::EguiContexts,
    mut occupied_screen_space: ResMut<OccupiedScreenSpace>,
    mut mogura_state: ResMut<MoguraState>,
    mut target_pdbid: Local<String>,
    mut trackball_camera: Query<&mut TrackballCamera, With<Camera>>,
) {
    let ctx = contexts.ctx_mut();
    let task_pool = bevy::tasks::AsyncComputeTaskPool::get();

    occupied_screen_space.left = egui::SidePanel::left("left")
        .resizable(true)
        .show(ctx, |ui| {
            ui.label("left panel");

            ui.separator();

            let _response = egui::TextEdit::singleline(&mut *target_pdbid)
                .hint_text("PDB ID here. e.g. 8GNG")
                .show(ui);

            if ui.button("Start to download").clicked() {
                let target_pdbid_clone = target_pdbid.clone();
                let task =
                    task_pool.spawn(async move { PDBData::download(&target_pdbid_clone).await });
                commands.spawn(DownloadPDB(task));
            }

            ui.separator();

            if ui.button("Select local file").clicked() {
                #[cfg(not(target_arch = "wasm32"))]
                {
                    let task = task_pool.spawn(async move {
                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                            let content =
                                std::fs::read_to_string(path.display().to_string()).unwrap();
                            Some((path.display().to_string(), content))
                        } else {
                            None
                        }
                    });
                    commands.spawn(SelectedFile(task));
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
                    commands.spawn(SelectedFile(task));
                }
            }

            ui.separator();

            ui.label("Select Drawing Method");
            let pre_drawing_method = mogura_state.drawing_method;
            ui.radio_value(&mut mogura_state.drawing_method, DrawingMethod::VDW, "VDW");
            ui.radio_value(&mut mogura_state.drawing_method, DrawingMethod::Licorise, "Licorise");
            // ui.radio_value(&mut mogura_state.drawing_method, DrawingMethod::Cartoon, "Cartoon");
            // ui.radio_value(&mut mogura_state.drawingMethod, DrawingMethod::NewCartoon, "NewCartoon");
            ui.radio_value(&mut mogura_state.drawing_method, DrawingMethod::Bonds, "Bonds");
            if pre_drawing_method != mogura_state.drawing_method {
                mogura_state.redraw = true;
            }

            ui.separator();

            ui.label("Looking at Structure");
            if ui.button("Look").clicked() {
                match &mogura_state.structure_data {
                    Some(structure_data) => {
                        let center = structure_data.center();
                        let center_vec = Vec3::new(center[0], center[1], center[2]);
                        let mut trackball_camera = trackball_camera.single_mut();
                        trackball_camera.frame.set_target(center_vec.into());
                    },
                    None => ()
                }
            }

            ui.separator();

            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .width();

    occupied_screen_space.right = egui::SidePanel::right("right")
        .resizable(true)
        .show(ctx, |ui| {
            ui.label("right panel");
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .width();

    occupied_screen_space.top = egui::TopBottomPanel::top("top")
        .resizable(true)
        .show(ctx, |ui| {
            ui.label("top panel");
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .height();

    occupied_screen_space.bottom = egui::TopBottomPanel::bottom("bottom")
        .resizable(true)
        .show(ctx, |ui| {
            ui.label("bottom panel");
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
pub fn absorb_egui_inputs(
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
