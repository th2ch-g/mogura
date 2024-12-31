use bevy::prelude::*;

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
            .add_systems(Startup, light::setup_light)
            .add_systems(
                Startup,
                (
                    (camera::setup_camera, structure::setup_structure)
                        .before(camera::set_look_at_center),
                    camera::set_look_at_center,
                ),
            )
            .add_systems(Startup, setup_material)
            .add_systems(Update, camera::update_camera_pos)
            .add_systems(Update, camera::update_camera_scroll)
            .add_systems(Update, camera::update_camera_mode);
    }
}

// for tmp
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
