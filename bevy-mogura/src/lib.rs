use bevy::prelude::*;
use bevy_trackball::prelude::*;
// use structure::LineMaterial;

mod camera;
mod light;
mod structure;

pub mod prelude {
    pub use crate::MoguraPlugins;
}

#[derive(Clone, Resource)]
pub struct MoguraPlugins {
    pub input_structure: Option<String>,
}

impl Default for MoguraPlugins {
    fn default() -> Self {
        Self {
            input_structure: None,
        }
    }
}

impl Plugin for MoguraPlugins {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.clone())
            // .add_plugins(MaterialPlugin::<LineMaterial>::default())
            .add_plugins(TrackballPlugin)
            .add_systems(Startup, light::setup_light)
            .add_systems(Startup, structure::setup_structure)
            .add_systems(Startup, setup_test)
            .add_systems(Startup, camera::setup_camera);
    }
}

fn setup_test(
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
