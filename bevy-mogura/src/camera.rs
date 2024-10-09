use bevy::{
    input::mouse::{
        MouseMotion,
        // MouseScrollUnit,
        MouseWheel,
    },
    prelude::*,
};

#[derive(Component, Debug)]
pub struct CameraParams {
    pub target: Vec3,
    pub mode: CameraMode,
    pub sensitivity: f32,
}

impl Default for CameraParams {
    fn default() -> Self {
        Self {
            target: Vec3::ZERO,
            mode: CameraMode::Rotation,
            sensitivity: 0.05,
        }
    }
}

#[derive(Debug)]
enum CameraMode {
    Rotation,
    Translation,
}

pub fn setup_camera(mut commands: Commands) {
    let target = Vec3::ZERO;
    let up = Vec3::Y;
    let init_pos = Vec3::new(0., 30., 0.);
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(init_pos).looking_at(target, up),
            camera: Camera {
                // background color
                clear_color: ClearColorConfig::Custom(Color::rgb(1., 1., 1.)),
                ..default()
            },
            ..default()
        },
        CameraParams {
            target,
            ..Default::default()
        },
    ));
}

pub fn update_camera(
    mut mouse_motion: EventReader<MouseMotion>,
    mut mouse_scroll: EventReader<MouseWheel>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut camera: Query<(&mut Transform, &mut CameraParams), With<Camera>>,
) {
    let (mut transform, mut camera_params) = camera.single_mut();
    if mouse_button.pressed(MouseButton::Left) {
        for motion in mouse_motion.read() {
            match camera_params.mode {
                CameraMode::Translation => {
                    let (u_x, u_y, _u_z) = get_unit_vectors_from_quat(transform.rotation);
                    let delta_x = -u_x * motion.delta.x * camera_params.sensitivity;
                    let delta_y = u_y * motion.delta.y * camera_params.sensitivity;
                    transform.translation += delta_x;
                    transform.translation += delta_y;
                    camera_params.target += delta_x;
                    camera_params.target += delta_y;
                }
                CameraMode::Rotation => {
                    let yaw = -motion.delta.x * camera_params.sensitivity;
                    let pitch = motion.delta.y * camera_params.sensitivity;
                    let roll = 0.;

                    let offset = transform.translation - camera_params.target;
                    let rotation_quat = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
                    let rotated_offset = rotation_quat * offset;

                    transform.translation = camera_params.target + rotated_offset;
                    transform.look_at(camera_params.target, Vec3::Y);
                }
            }
        }
    } else {
        for scroll in mouse_scroll.read() {
            let dir = (transform.translation - camera_params.target).normalize();
            let mut radius = (transform.translation - camera_params.target).length();
            radius += scroll.y * camera_params.sensitivity;
            radius = radius.max(0.1);
            transform.translation = camera_params.target + radius * dir;
            transform.look_at(camera_params.target, Vec3::Y);
            // match scroll.unit {
            //     MouseScrollUnit::Line => {
            //         dbg!("a", &scroll.y, &scroll.x);
            //     },
            //     MouseScrollUnit::Pixel => {
            //         dbg!("b", &scroll.y, &scroll.x);
            //     }
            // }
        }
    }

    if keys.just_pressed(KeyCode::KeyR) {
        camera_params.mode = CameraMode::Rotation;
    }
    if keys.just_pressed(KeyCode::KeyT) {
        camera_params.mode = CameraMode::Translation;
    }

    fn get_unit_vectors_from_quat(quat: Quat) -> (Vec3, Vec3, Vec3) {
        // let x = quat.x;
        // let y = quat.y;
        // let z = quat.z;
        // let w = quat.w;
        // let u_x = Vec3::new(
        //     1.0 - 2.0 * (y * y + z * z),
        //     2.0 * (x * y - w * z),
        //     2.0 * (x * z + w * y),
        // );
        // let u_y = Vec3::new(
        //     2.0 * (x * y + w * z),
        //     1.0 - 2.0 * (x * x + z * z),
        //     2.0 * (y * z - w * x),
        // );
        // let u_z = Vec3::new(
        //     2.0 * (x * z - w * y),
        //     2.0 * (y * z + w * x),
        //     1.0 - 2.0 * (x * x + y * y),
        // );
        let u_x = quat * Vec3::X;
        let u_y = quat * Vec3::Y;
        let u_z = quat * Vec3::Z;
        (u_x.normalize(), u_y.normalize(), u_z.normalize())
    }
}
