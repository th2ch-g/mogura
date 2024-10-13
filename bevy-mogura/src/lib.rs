use bevy::prelude::*;

mod camera;
mod light;
mod structure;

pub mod prelude {
    pub use crate::MoguraPlugins;
}

pub struct MoguraPlugins;

impl Plugin for MoguraPlugins {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, camera::setup_camera)
            .add_systems(Startup, light::setup_light)
            .add_systems(Startup, structure::setup_structure)
            .add_systems(Startup, setup_material)
            .add_systems(Update, camera::update_camera);
    }
}

fn setup_material(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(20.0, 20.0)),
        material: materials.add(Color::srgb(0.3, 0.5, 0.3)),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(Sphere::default()),
        material: materials.add(StandardMaterial {
            base_color: Srgba::hex("126212CC").unwrap().into(),
            reflectance: 1.0,
            perceptual_roughness: 0.0,
            metallic: 0.5,
            alpha_mode: AlphaMode::Blend,
            ..default()
        }),
        transform: Transform::from_xyz(0.0, 1.0, 0.0),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(Sphere::default()),
        material: materials.add(StandardMaterial {
            base_color: Color::srgba(1.0, 2.0, 3.0, 1.0),
            ..default()
        }),
        transform: Transform::from_xyz(-1.0, 1.0, -1.0),
        ..default()
    });
}
