#[derive(Debug, Clone)]
pub struct Settings {
    pub pdbfile: String,
    pub camera_mode: crate::camera::CameraMode,
    pub camera_pmode: crate::camera::CameraMode,
    pub projection_mode: crate::camera::ProjectionMode,
    pub move_mode: MoveMode,
    pub group_to_select: GroupToSelect,
    pub selected_group: SelectedGroup,
    pub show_close_dialog: bool,
    pub allowed_to_close: bool,
}

impl Settings {
    pub fn new(pdbfile: String) -> Self {
        Self {
            pdbfile,
            camera_mode: crate::camera::CameraMode::Rotation,
            camera_pmode: crate::camera::CameraMode::Rotation,
            // projection_mode: crate::camera::ProjectionMode::Perspective,
            projection_mode: crate::camera::ProjectionMode::Orthographic,
            move_mode: MoveMode::Off,
            group_to_select: GroupToSelect::Atoms,
            selected_group: SelectedGroup::default(),
            show_close_dialog: false,
            allowed_to_close: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MoveMode {
    // MoveWithNNP,
    // MoveWithoutNNP,
    Move,
    Off,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GroupToSelect {
    Atoms,
    Residues,
    Molecules,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct SelectedGroup {
    pub atoms: std::collections::HashSet<usize>,
}

impl SelectedGroup {
    pub fn reset(&mut self) {
        self.atoms = std::collections::HashSet::new();
    }
}

// impl Default for SelectedGroup {
//     fn default() -> Self {
//         Self {
//             atoms: std::collections::HashSet::new(),
//         }
//     }
// }
