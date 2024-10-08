use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

pub struct MoguraPlugins;

impl Plugin for MoguraPlugins {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_systems(Startup, setup_light)
            .add_systems(Startup, setup_material)
            .add_systems(Update, update_camera);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(15.0, 5.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn update_camera(
    mut mouse_motion: EventReader<MouseMotion>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut camera: Query<&mut Transform, With<Camera>>,
) {
    if mouse_button.pressed(MouseButton::Left) {
        let mut camera = camera.single_mut();
        for motion in mouse_motion.read() {
            dbg!(&motion.delta.x, &motion.delta.y);
            dbg!(&camera);
        }
    }
}

fn setup_light(mut commands: Commands) {
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_translation(Vec3::ONE).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn setup_material(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(20.0, 20.0)),
            material: materials.add(Color::srgb(0.3, 0.5, 0.3)),
            ..default()
        },
        Ground,
    ));

    commands.spawn((
        PbrBundle {
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
        },
        SimpleSphere,
    ));

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Sphere::default()),
            material: materials.add(StandardMaterial {
                base_color: Color::srgba(1.0, 2.0, 3.0, 1.0),
                ..default()
            }),
            transform: Transform::from_xyz(-1.0, 1.0, -1.0),
            ..default()
        },
        SimpleSphere,
    ));
}

#[derive(Component)]
struct Ground;

#[derive(Component)]
struct SimpleSphere;
