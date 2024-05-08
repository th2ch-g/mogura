use std::io::Write;

#[derive(Debug, Clone)]
pub struct EguiGUI {
    pub settings: std::rc::Rc<std::cell::RefCell<crate::settings::Settings>>,
}

const LEFT_SIDE_PANEL_DEFAULT_WIDTH: f32 = 150.0;

impl EguiGUI {
    pub fn new(settings: std::rc::Rc<std::cell::RefCell<crate::settings::Settings>>) -> Self {
        Self { settings }
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
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("pdb", &["pdb"])
                        .pick_file()
                    {
                        self.settings.borrow_mut().pdbfile = Some(path.display().to_string());
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
                    let mut show_download_dialog = self.settings.borrow().show_download_dialog.clone();
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
                                match download_pdbfile_from_pdbid(&pdbid) {
                                    Ok(pdbfile) => {
                                        self.settings.borrow_mut().renew_render = true;
                                        self.settings.borrow_mut().pdbfile = Some(pdbfile);
                                        self.settings.borrow_mut().show_download_dialog = false;
                                    }
                                    Err(s) => {
                                        eprintln!("Error occurred: {}", s);
                                    }
                                }
                            }
                        });
                    self.settings.borrow_mut().show_download_dialog = show_download_dialog;
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

                ui.menu_button("Drawing Method", |ui| {
                    ui.label("Choose!");
                });
            });
    }
}

fn download_pdbfile_from_pdbid(pdbid: &str) -> anyhow::Result<String, anyhow::Error> {
    let response = reqwest::blocking::Client::new()
        .get(format!("https://files.rcsb.org/view/{}.pdb", pdbid))
        .send()?;
    let status_code = response.status().as_u16();
    let content = response.text()?;

    if status_code == 200 {
        let pdbfile = format!("{}.{}.pdb", env!("CARGO_PKG_NAME"), pdbid);
        let mut file = std::fs::File::create(&pdbfile)?;
        file.write_all(content.as_bytes())?;
        Ok(pdbfile)
    } else {
        Err(anyhow::anyhow!("Failed to download PDB file for {}", pdbid))
    }
}
