use crate::structure::*;
use bevy::prelude::*;
use mogura_io::prelude::*;

mod camera;
mod gui;
mod light;
mod structure;
mod trajectory;

pub mod prelude {
    pub use crate::MoguraPlugins;
}

#[derive(Clone)]
pub struct MoguraPlugins {
    pub input_structure_file: Option<String>,
    pub input_trajectory_file: Option<String>,
}

impl Default for MoguraPlugins {
    fn default() -> Self {
        Self {
            input_structure_file: None,
            input_trajectory_file: None,
        }
    }
}

impl Plugin for MoguraPlugins {
    fn build(&self, app: &mut App) {
        let mogura_state = MoguraState::new(
            self.input_structure_file.clone(),
            self.input_trajectory_file.clone(),
        );

        bevy::asset::load_internal_asset!(
            app,
            structure::SHADER_HANDLE,
            "line_material.wgsl",
            Shader::from_wgsl
        );

        app.insert_resource(mogura_state)
            // .add_systems(Startup, dbg::setup_test)
            .add_plugins(camera::MoguraCameraPlugins)
            .add_plugins(gui::MoguraGuiPlugins)
            .add_plugins(light::MoguraLightPlugins)
            .add_plugins(structure::MoguraStructurePlugins)
            .add_plugins(trajectory::MoguraTrajectoryPlugins);
    }
}

#[derive(Resource)]
pub struct MoguraState {
    pub structure_file: Option<String>,
    pub structure_data: Option<Box<dyn StructureData>>,
    pub trajectory_file: Option<String>,
    pub trajectory_data: Option<Box<dyn TrajectoryData>>,
    pub drawing_method: structure::DrawingMethod,
    pub redraw: bool,
    pub update_trajectory: bool,
    pub update_tmp_trajectory: bool,
    pub loop_trajectory: bool,
    pub current_frame_id: usize,
    pub atom_selection: String,
    pub selected_atoms: std::collections::HashSet<usize>,
    pub selected_bonds: std::collections::HashSet<(usize, usize)>,
}

impl MoguraState {
    pub fn new(structure_file: Option<String>, trajectory_file: Option<String>) -> Self {
        let structure_data = if let Some(ref file) = structure_file {
            Some(structure_loader(&file))
        } else {
            None
        };
        let trajectory_data = if let Some(ref str_file) = structure_file {
            if let Some(ref traj_file) = trajectory_file {
                Some(trajectory_loader(&str_file, &traj_file))
            } else {
                None
            }
        } else {
            None
        };
        Self {
            structure_data,
            structure_file,
            trajectory_data,
            trajectory_file,
            drawing_method: structure::DrawingMethod::Licorise,
            redraw: true,
            update_trajectory: false,
            update_tmp_trajectory: false,
            loop_trajectory: false,
            current_frame_id: 0,
            atom_selection: "all".to_string(),
            selected_atoms: std::collections::HashSet::new(),
            selected_bonds: std::collections::HashSet::new(),
        }
    }

    pub fn n_frame(&self) -> Option<usize> {
        self.trajectory_data
            .as_ref()
            .and_then(|td| Some(td.n_frame()))
    }

    pub fn next_frame_id(&mut self) {
        let n_frame = if let Some(n_frame) = self.n_frame() {
            n_frame
        } else {
            return;
        };
        self.current_frame_id += 1;
        if self.current_frame_id >= n_frame {
            self.current_frame_id = 0;
            self.update_trajectory = false;
        } else {
            self.update_trajectory = true;
        }
    }

    pub fn loop_frame_id(&mut self) {
        let n_frame = if let Some(n_frame) = self.n_frame() {
            n_frame
        } else {
            return;
        };
        self.current_frame_id += 1;
        if self.current_frame_id >= n_frame {
            self.current_frame_id = 0;
        }
    }

    pub fn apply_selection(&mut self) -> Result<(), String> {
        let selection = mogura_asl::parse_selection(&self.atom_selection)?;
        let (selected_atoms, selected_bonds) = {
            let atoms = self.structure_data.as_ref().unwrap().atoms();
            let bonds = self.structure_data.as_ref().unwrap().bonds_indirected();
            let (selected_atoms, selected_bonds) = selection.select_atoms_bonds(&atoms, &bonds);
            (selected_atoms, selected_bonds)
        };
        self.selected_atoms = selected_atoms;
        self.selected_bonds = selected_bonds;
        Ok(())
    }
}

mod dbg {
    use bevy::prelude::*;
    pub fn setup_test(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        // circular base
        commands.spawn((
            Mesh3d(meshes.add(Circle::new(4.0))),
            MeshMaterial3d(materials.add(Color::WHITE)),
            Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ));
        // cube
        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
            MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
            Transform::from_xyz(0.0, 0.5, 0.0),
        ));
    }
}
