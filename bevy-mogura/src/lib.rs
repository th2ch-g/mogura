use bevy::prelude::*;
use bevy_trackball::prelude::*;
use structure::LineMaterial;
use mogura_io::prelude::*;

mod camera;
mod gui;
mod light;
mod structure;

pub mod prelude {
    pub use crate::MoguraPlugins;
}

#[derive(Clone, Resource)]
pub struct MoguraPlugins {
    pub input_structure_file: Option<String>,
}

impl Default for MoguraPlugins {
    fn default() -> Self {
        Self {
            input_structure_file: None,
        }
    }
}

impl Plugin for MoguraPlugins {
    fn build(&self, app: &mut App) {
        let mogura_state = MoguraState::new(self.input_structure_file.clone());

        app.insert_resource(mogura_state)
            .init_resource::<gui::OccupiedScreenSpace>()
            .add_plugins(MaterialPlugin::<LineMaterial>::default())
            .add_plugins(TrackballPlugin)
            .add_plugins(bevy_egui::EguiPlugin)
            .add_systems(Startup, light::setup_light)
            .add_systems(Startup, dbg::setup_test)
            .add_systems(Startup, camera::setup_camera)
            .add_systems(
                PreUpdate,
                gui::absorb_egui_inputs
                    .after(bevy_egui::systems::process_input_system)
                    .before(bevy_egui::EguiSet::BeginPass),
            )
            .add_systems(Update, structure::update_structure)
            .add_systems(Update, gui::poll_rfd)
            .add_systems(Update, gui::poll_downloadpdb)
            .add_systems(Update, gui::update_gui);
    }
}

#[derive(Resource)]
pub struct MoguraState {
    pub structure_file: Option<String>,
    pub structure_data: Option<Box<dyn StructureData>>,
    pub drawing_method: structure::DrawingMethod,
    pub redraw: bool,
}

impl MoguraState {
    pub fn new(structure_file: Option<String>) -> Self {
        Self {
            structure_data: if let Some(ref file) = structure_file {
                Some(structure_loader(&file))
            } else {
                None
            },
            structure_file,
            drawing_method: structure::DrawingMethod::Licorise,
            redraw: true,
        }
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
