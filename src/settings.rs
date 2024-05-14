#[derive(Debug, Clone)]
pub struct Settings {
    pub renew_render: bool,
    // pub pdbfile: Option<String>,
    pub pdbcontent: Option<String>,
    pub camera_mode: crate::camera::CameraMode,
    pub camera_pmode: crate::camera::CameraMode,
    pub projection_mode: crate::camera::ProjectionMode,
    pub move_mode: MoveMode,
    pub group_to_select: GroupToSelect,
    pub selected_group: SelectedGroup,
    pub show_close_dialog: bool,
    pub allowed_to_close: bool,
    pub show_download_dialog: bool,
    pub download_pdbid: String,
    pub show_settings_window: bool,
    pub backend_info: wgpu::AdapterInfo,
    pub camera_speed: u32,
    pub vis_axis: bool,
    pub vis_center: bool,
    pub drawing_method: DrawingMethod,
}

impl Settings {
    pub fn new(pdbcontent: Option<String>, backend_info: wgpu::AdapterInfo) -> Self {
        Self {
            renew_render: false,
            // pdbfile,
            pdbcontent,
            camera_mode: crate::camera::CameraMode::Rotation,
            camera_pmode: crate::camera::CameraMode::Rotation,
            // projection_mode: crate::camera::ProjectionMode::Perspective,
            projection_mode: crate::camera::ProjectionMode::Orthographic,
            move_mode: MoveMode::Off,
            group_to_select: GroupToSelect::Atoms,
            selected_group: SelectedGroup::default(),
            show_close_dialog: false,
            allowed_to_close: false,
            show_download_dialog: false,
            download_pdbid: "".to_string(),
            show_settings_window: false,
            backend_info,
            camera_speed: 2,
            vis_axis: true,
            vis_center: true,
            drawing_method: DrawingMethod::Lines,
            // drawing_method: DrawingMethod::VDW,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DrawingMethod {
    Lines,
    VDW,
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
