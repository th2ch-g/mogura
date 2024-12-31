use crate::structure::StructureParams;
use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
    window::PrimaryWindow,
};

#[derive(Component, Debug)]
pub struct CameraParams {
    pub target: Vec3,
    pub mode: CameraMode,
    pub sensitivity: f32,
    pub pre_mouse: Option<Vec2>,
}

impl Default for CameraParams {
    fn default() -> Self {
        Self {
            target: Vec3::ZERO,
            mode: CameraMode::Rotation,
            sensitivity: 0.03,
            pre_mouse: None,
        }
    }
}

#[derive(Debug)]
enum CameraMode {
    Rotation,
    Translation,
}

pub fn set_look_at_center(
    mut camera: Query<(&mut Transform, &mut CameraParams), With<Camera>>,
    mut structure_params: Query<&mut StructureParams>,
) {
    let structure_params = structure_params.single();
    let (mut transform, mut camera_params) = camera.single_mut();
    if let Some(structure_data) = &structure_params.structure_data {
        let center = structure_data.center();
        transform.translation = Vec3::new(center[0], center[1] * 2., center[2]);
        camera_params.target = Vec3::new(center[0], center[1], center[2]);
        transform.look_at(camera_params.target, Vec3::Y);
    }
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
            ..default()
        },
    ));
}

pub fn update_camera_mode(
    keys: Res<ButtonInput<KeyCode>>,
    mut camera: Query<&mut CameraParams, With<Camera>>,
) {
    let mut camera_params = camera.single_mut();
    if keys.just_pressed(KeyCode::KeyR) {
        camera_params.mode = CameraMode::Rotation;
    }
    if keys.just_pressed(KeyCode::KeyT) {
        camera_params.mode = CameraMode::Translation;
    }
}

pub fn update_camera_scroll(
    mut camera: Query<(&mut Transform, &mut CameraParams), With<Camera>>,
    mut mouse_scroll: EventReader<MouseWheel>,
) {
    let (mut transform, mut camera_params) = camera.single_mut();
    let mut radius = (transform.translation - camera_params.target).length();
    for scroll in mouse_scroll.read() {
        let dir = (transform.translation - camera_params.target).normalize();
        radius += scroll.y * camera_params.sensitivity;
        radius = radius.max(0.1);
        transform.translation = camera_params.target + radius * dir;
        transform.look_at(camera_params.target, Vec3::Y);
    }
}

pub fn update_camera_pos(
    mut mouse_motion: EventReader<MouseMotion>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    window: Query<&Window, With<PrimaryWindow>>,
    mut camera: Query<(&mut Transform, &mut CameraParams), With<Camera>>,
) {
    if mouse_button.pressed(MouseButton::Left) {
        let (mut transform, mut camera_params) = camera.single_mut();
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
                _ => {}
            }
        }

        match camera_params.mode {
            CameraMode::Rotation => {
                // Shoemake Virtual Trackball
                // https://doi.org/10.1109/TVCG.2004.1260772
                let mapped_shoemake = |x: f32, y: f32, r: f32| -> Vec3 {
                    let x2_y2 = x.powi(2) + y.powi(2);
                    let r2 = r.powi(2);
                    if x2_y2 <= r2 {
                        Vec3::new(x, y, (r2 - x2_y2).sqrt())
                    } else {
                        let a = r / x2_y2.sqrt();
                        Vec3::new(x * a, y * a, 0.)
                    }
                };

                if let Some(position) = window.single().cursor_position() {
                    // window position
                    let width = window.single().width();
                    let height = window.single().height();
                    let win_x = position.x / (width / 2.) - 1.;
                    let win_y = position.y / (height / 2.) - 1.;

                    // rotater
                    // let mut radius = (transform.translation - camera_params.target).length();
                    let pre_mouse = match camera_params.pre_mouse {
                        Some(p) => p,
                        None => Vec2::new(win_x, win_y),
                    };
                    let p = mapped_shoemake(pre_mouse.x, pre_mouse.y, 1.).normalize();
                    let q = mapped_shoemake(win_x, win_y, 1.).normalize();
                    let theta = p.dot(q).clamp(-1., 1.).acos() / 2.;
                    let (theta_sin, theta_cos) = theta.sin_cos();
                    let n = {
                        let tmp = p.cross(q);
                        if tmp.length() != 0. {
                            tmp.normalize()
                        } else {
                            tmp
                        }
                    };
                    let theta_sin_n = theta_sin * n;
                    let rotater =
                        Quat::from_xyzw(theta_sin_n.x, theta_sin_n.y, theta_sin_n.z, theta_cos);

                    let offset = transform.translation - camera_params.target;
                    let rotated_offset = rotater * offset;
                    transform.translation = camera_params.target + rotated_offset;
                    transform.look_at(camera_params.target, Vec3::Y);
                    camera_params.pre_mouse = Some(Vec2::new(win_x, win_y));
                }
            }
            _ => {}
        }
    }

    #[inline]
    fn get_unit_vectors_from_quat(quat: Quat) -> (Vec3, Vec3, Vec3) {
        let u_x = quat * Vec3::X;
        let u_y = quat * Vec3::Y;
        let u_z = quat * Vec3::Z;
        (u_x.normalize(), u_y.normalize(), u_z.normalize())
    }
}
