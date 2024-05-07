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
