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
    let target = Vec3::ZERO;
    let up = Vec3::Y;
    let init_pos = [0., 30., 0.];
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(init_pos[0], init_pos[1], init_pos[2])
                .looking_at(target, up),
            ..default()
        },
        CameraParams {
            target,
            ..Default::default()
        },
    ));
}

fn update_camera(
    mut mouse_motion: EventReader<MouseMotion>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut camera: Query<(&mut Transform, &mut CameraParams), With<Camera>>,
) {
    if mouse_button.pressed(MouseButton::Left) {
        let (mut transform, mut camera_params) = camera.single_mut();
        dbg!(&transform);
        for motion in mouse_motion.read() {
            match camera_params.mode {
                CameraMode::Translation => {
                    let (u_x, u_y, u_z) = get_unit_vectors_from_quat(transform.rotation);
                    let delta_x = u_y * motion.delta.x * camera_params.sensitivity;
                    let delta_y = u_z * motion.delta.y * camera_params.sensitivity;
                    transform.translation += delta_x;
                    transform.translation += delta_y;
                    camera_params.target += delta_x;
                    camera_params.target += delta_y;
                }
                CameraMode::Rotation => {
                    let yaw = motion.delta.y * camera_params.sensitivity;
                    let pitch = motion.delta.x * camera_params.sensitivity;
                    let roll = 0.;
                    let rotation_quat = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
                    transform.rotation = rotation_quat * transform.rotation;
                    let radius = (transform.translation - camera_params.target).length();
                    let (dir, _, _) = get_unit_vectors_from_quat(transform.rotation);
                    transform.translation = camera_params.target - dir * radius;
                }
            }
        }
    }

    fn get_unit_vectors_from_quat(quat: Quat) -> (Vec3, Vec3, Vec3) {
        let x = quat.x;
        let y = quat.y;
        let z = quat.z;
        let w = quat.w;
        let u_x = Vec3::new(
            1.0 - 2.0 * (y * y + z * z),
            2.0 * (x * y - w * z),
            2.0 * (x * z + w * y),
        );
        let u_y = Vec3::new(
            2.0 * (x * y + w * z),
            1.0 - 2.0 * (x * x + z * z),
            2.0 * (y * z - w * x),
        );
        let u_z = Vec3::new(
            2.0 * (x * z - w * y),
            2.0 * (y * z + w * x),
            1.0 - 2.0 * (x * x + y * y),
        );
        (u_x.normalize(), u_y.normalize(), u_z.normalize())
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

#[derive(Component, Debug)]
struct CameraParams {
    pub target: Vec3,
    pub mode: CameraMode,
    pub sensitivity: f32,
}

impl Default for CameraParams {
    fn default() -> Self {
        Self {
            target: Vec3::ZERO,
            // mode: CameraMode::Translation,
            mode: CameraMode::Rotation,
            sensitivity: 0.01,
            // sensitivity: 0.1,
        }
    }
}

#[derive(Debug)]
enum CameraMode {
    Rotation,
    Translation,
}
