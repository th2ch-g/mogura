#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone)]
pub struct EguiGUI {
    pub settings: std::rc::Rc<std::cell::RefCell<crate::settings::Settings>>,
    pdbcontent_sandbox: std::sync::Arc<std::sync::Mutex<Option<String>>>,
}

const LEFT_SIDE_PANEL_DEFAULT_WIDTH: f32 = 200.0;

impl EguiGUI {
    pub fn new(settings: std::rc::Rc<std::cell::RefCell<crate::settings::Settings>>) -> Self {
        let pdbcontent = settings.borrow().pdbcontent.clone();
        Self {
            settings,
            pdbcontent_sandbox: std::sync::Arc::new(std::sync::Mutex::new(pdbcontent)),
        }
    }
    pub fn run_gui(&mut self, ctx: &egui::Context) {
        if ctx.input(|i| i.viewport().close_requested()) && !self.settings.borrow().allowed_to_close
        {
            ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
            self.settings.borrow_mut().show_close_dialog = true;
        }

        if self.settings.borrow().show_close_dialog {
            egui::Window::new("Do you want to close?")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        if ui.button("No").clicked() {
                            self.settings.borrow_mut().show_close_dialog = false;
                            self.settings.borrow_mut().allowed_to_close = false;
                        }

                        if ui.button("Yes").clicked() {
                            self.settings.borrow_mut().show_close_dialog = false;
                            self.settings.borrow_mut().allowed_to_close = true;
                            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                });
        }

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                egui::widgets::global_dark_light_mode_switch(ui);
                ui.separator();
                if ui.button("File").on_hover_text("Load PDB File").clicked() {
                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        if let Some(path) = rfd::FileDialog::new()
                            .add_filter("pdb", &["pdb"])
                            .pick_file()
                        {
                            self.settings.borrow_mut().pdbcontent = Some(
                                std::fs::read_to_string(path.display().to_string()).unwrap()
                            );
                            self.settings.borrow_mut().renew_render = true;
                        }
                    }

                    #[cfg(target_arch = "wasm32")]
                    {
                        let pdbcontent_sandbox_clone = self.pdbcontent_sandbox.clone();

                        let task = rfd::AsyncFileDialog::new()
                            .add_filter("pdb", &["pdb"])
                            .pick_file();

                        pollster::block_on(async {
                            wasm_bindgen_futures::spawn_local(async move {
                                let path = task.await;
                                if let Some(path) = path {
                                    // rfd in wasm, need to use read instead of file path
                                    // let mut pdbfile_lock = pdbfile_sandbox_clone.lock().unwrap();
                                    // *pdbfile_lock = Some(path.file_name().to_owned());
                                    let file_content = path.read().await;
                                    match String::from_utf8(file_content) {
                                        Ok(s) => {
                                            let mut pdbcontent_lock = pdbcontent_sandbox_clone.lock().unwrap();
                                            *pdbcontent_lock = Some(s);
                                        },
                                        Err(e) => { panic!("Could not open the file: {:?}", e); }
                                    }
                                }
                            });
                        });
                    }
                }

                #[cfg(target_arch = "wasm32")]
                {
                    let pdbcontent2: Option<String> = self.pdbcontent_sandbox.lock().unwrap().clone();

                    if pdbcontent2 != self.settings.borrow().pdbcontent {
                        self.settings.borrow_mut().pdbcontent = pdbcontent2;
                        self.settings.borrow_mut().renew_render = true;
                    }
                }

                ui.separator();
                if ui
                    .button("Download")
                    .on_hover_text("Download from RCSB PDB")
                    .clicked()
                {
                    self.settings.borrow_mut().show_download_dialog = true;
                }
                if self.settings.borrow().show_download_dialog {
                    let mut show_download_dialog = self.settings.borrow().show_download_dialog;
                    egui::Window::new("Input PDB ID to download")
                        .collapsible(false)
                        .resizable(false)
                        .open(&mut show_download_dialog)
                        .show(ctx, |ui| {
                            let _response = egui::TextEdit::singleline(
                                &mut self.settings.borrow_mut().download_pdbid,
                            )
                            .hint_text("PDB ID here")
                            .show(ui);
                            if ui.button("Start to download").clicked() {
                                let pdbid = self.settings.borrow().download_pdbid.clone();

                                #[cfg(not(target_arch = "wasm32"))]
                                {
                                    match download_pdbcontent_from_pdbid(&pdbid) {
                                        Ok(pdbcontent) => {
                                            self.settings.borrow_mut().renew_render = true;
                                            self.settings.borrow_mut().pdbcontent = Some(pdbcontent);
                                        }
                                        Err(s) => {
                                            eprintln!("Error occurred: {}", s);
                                        }
                                    }
                                }

                                #[cfg(target_arch = "wasm32")]
                                {
                                    let pdbcontent_sandbox_clone = self.pdbcontent_sandbox.clone();
                                    pollster::block_on(async {
                                        wasm_bindgen_futures::spawn_local(async move {
                                            let mut opts = web_sys::RequestInit::new();
                                            opts.method("GET");
                                            opts.mode(web_sys::RequestMode::Cors);
                                            let url = format!("https://files.rcsb.org/view/{}.pdb", pdbid);
                                            let request = web_sys::Request::new_with_str_and_init(&url, &opts).unwrap();
                                            let window = gloo::utils::window();
                                            let resp_value = wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&request)).await.unwrap();
                                            let resp: web_sys::Response = resp_value.dyn_into().unwrap();
                                            let text = wasm_bindgen_futures::JsFuture::from(resp.text().unwrap()).await.unwrap();
                                            let mut pdbcontent_lock = pdbcontent_sandbox_clone.lock().unwrap();
                                            *pdbcontent_lock = Some(text.as_string().unwrap());
                                        });
                                    });
                                    let pdbcontent2: Option<String> = self.pdbcontent_sandbox.lock().unwrap().clone();

                                    if pdbcontent2 != self.settings.borrow().pdbcontent {
                                        self.settings.borrow_mut().pdbcontent = pdbcontent2;
                                        self.settings.borrow_mut().renew_render = true;
                                    }
                                }
                            }
                        });
                    self.settings.borrow_mut().show_download_dialog = show_download_dialog;
                }
                ui.separator();
                if ui.button("Settings").clicked() {
                    self.settings.borrow_mut().show_settings_window ^= true;
                }
                if self.settings.borrow().show_settings_window {
                    let mut show_settings_window = self.settings.borrow().show_settings_window;
                    egui::Window::new("Settings")
                        .collapsible(true)
                        .resizable(true)
                        .open(&mut show_settings_window)
                        .default_width(1000.0)
                        .default_height(1000.0)
                        .show(ctx, |ui| {
                            // ui.collapsing("Move Speed", |ui| {});

                            ui.collapsing("Camera Speed", |ui| {
                                ui.add(
                                    egui::Slider::new(
                                        &mut self.settings.borrow_mut().camera_speed,
                                        1..=10,
                                    )
                                    .text("age"),
                                );
                                ui.add_space(5.0);
                            });

                            ui.collapsing("For Debug", |ui| {
                                let mut vis_axis = self.settings.borrow().vis_axis;
                                let mut vis_center = self.settings.borrow().vis_center;
                                ui.checkbox(&mut vis_axis, "Visualize XYZ axis");
                                ui.checkbox(&mut vis_center, "Visualize Center");
                                if vis_axis != self.settings.borrow().vis_axis {
                                    self.settings.borrow_mut().vis_axis = vis_axis;
                                }
                                if vis_center != self.settings.borrow().vis_center {
                                    self.settings.borrow_mut().vis_center = vis_center;
                                }
                            });

                            ui.collapsing("About Backend", |ui| {
                                ui.label(format!("{:?}", self.settings.borrow().backend_info));
                            });
                        });
                    self.settings.borrow_mut().show_settings_window = show_settings_window;
                }
                ui.separator();
            });
        });

        egui::SidePanel::left("left_panel")
            .resizable(false)
            .default_width(LEFT_SIDE_PANEL_DEFAULT_WIDTH)
            .show(ctx, |ui| {
                ui.label("CameraMode Selection");
                ui.radio_value(
                    &mut self.settings.borrow_mut().camera_mode,
                    crate::camera::CameraMode::Normal,
                    "Normal",
                )
                .on_hover_text("Clear view of surroundings");
                ui.radio_value(
                    &mut self.settings.borrow_mut().camera_mode,
                    crate::camera::CameraMode::Rotation,
                    "Rotation",
                )
                .on_hover_text("Rotate with center of molecule as center");
                ui.radio_value(
                    &mut self.settings.borrow_mut().camera_mode,
                    crate::camera::CameraMode::Translation,
                    "Translation",
                )
                .on_hover_text("Move horizontally easily");
                ui.separator();

                ui.label("ProjectionMode Selection");
                ui.radio_value(
                    &mut self.settings.borrow_mut().projection_mode,
                    crate::camera::ProjectionMode::Perspective,
                    "Perspective",
                )
                .on_hover_text("Perspective Projection");
                ui.radio_value(
                    &mut self.settings.borrow_mut().projection_mode,
                    crate::camera::ProjectionMode::Orthographic,
                    "Orthographic",
                )
                .on_hover_text("Orthographic Projection");
                ui.separator();

                ui.label("MoveMode Selection");
                // ui.radio_value(
                //     &mut self.settings.borrow_mut().move_mode,
                //     crate::settings::MoveMode::MoveWithNNP,
                //     "Move with NNP",
                // )
                // .on_hover_text("Move groups using Neural Network Potential");
                ui.radio_value(
                    &mut self.settings.borrow_mut().move_mode,
                    // crate::settings::MoveMode::MoveWithoutNNP,
                    crate::settings::MoveMode::Move,
                    "Move",
                )
                .on_hover_text("Move selected groups");
                ui.radio_value(
                    &mut self.settings.borrow_mut().move_mode,
                    crate::settings::MoveMode::Off,
                    "Off",
                )
                .on_hover_text("Turn off MoveMode");

                ui.separator();

                ui.label("Group to select");
                ui.radio_value(
                    &mut self.settings.borrow_mut().group_to_select,
                    crate::settings::GroupToSelect::Atoms,
                    "Atoms",
                )
                .on_hover_text("Select atom level as groups");
                ui.radio_value(
                    &mut self.settings.borrow_mut().group_to_select,
                    crate::settings::GroupToSelect::Residues,
                    "Residues",
                )
                .on_hover_text("Select residue level as groups");
                ui.radio_value(
                    &mut self.settings.borrow_mut().group_to_select,
                    crate::settings::GroupToSelect::Molecules,
                    "Molecules",
                )
                .on_hover_text("Select molecule level as groups");

                ui.separator();

                ui.collapsing("Drawing Method", |ui| {
                    let mut drawing_method = self.settings.borrow().drawing_method;
                    ui.radio_value(
                        &mut drawing_method,
                        crate::settings::DrawingMethod::Lines,
                        "Lines",
                    );
                    ui.radio_value(
                        &mut drawing_method,
                        crate::settings::DrawingMethod::VDW,
                        "VDW",
                    );
                    if drawing_method != self.settings.borrow().drawing_method {
                        self.settings.borrow_mut().renew_render = true;
                        self.settings.borrow_mut().drawing_method = drawing_method;
                    }
                });
            });
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn download_pdbcontent_from_pdbid(pdbid: &str) -> anyhow::Result<String, anyhow::Error> {
    let response = reqwest::blocking::Client::new()
        .get(format!("https://files.rcsb.org/view/{}.pdb", pdbid))
        .send()?;
    let status_code = response.status().as_u16();
    let content = response.text()?;

    if status_code == 200 {
        // let pdbfile = format!("{}.{}.pdb", env!("CARGO_PKG_NAME"), pdbid);
        // let mut file = std::fs::File::create(&pdbfile)?;
        // file.write_all(content.as_bytes())?;
        Ok(content)
    } else {
        Err(anyhow::anyhow!("Failed to download PDB file for {}", pdbid))
    }
}
