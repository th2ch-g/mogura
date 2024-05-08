// use cgmath::InnerSpace;
// use cgmath::SquareMatrix;
use cgmath::prelude::*;

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);

const SAFE_FRAC_PI_2: f32 = std::f32::consts::FRAC_PI_2 - 0.0001;

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum ProjectionMode {
    Perspective,
    Orthographic,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CameraMode {
    Normal,
    Rotation,
    Translation,
}

// #[derive(Debug, Clone, Copy, PartialEq)]
// pub enum MoveMode {
//     MoveWithNNP,
//     MoveWithoutNNP,
//     Off,
// }
//
// #[derive(Debug, Clone, Copy, PartialEq)]
// pub enum GroupToSelect {
//     Atoms,
//     Residues,
//     Molecules,
// }

#[derive(Debug)]
pub struct Camera {
    pub position: cgmath::Point3<f32>,
    pub yaw: cgmath::Rad<f32>,
    pub pitch: cgmath::Rad<f32>,
}

impl Camera {
    pub fn new<
        V: Into<cgmath::Point3<f32>>,
        Y: Into<cgmath::Rad<f32>>,
        P: Into<cgmath::Rad<f32>>,
    >(
        position: V,
        yaw: Y,
        pitch: P,
    ) -> Self {
        Self {
            position: position.into(),
            yaw: yaw.into(),
            pitch: pitch.into(),
        }
    }

    pub fn calc_matrix(&self) -> cgmath::Matrix4<f32> {
        let (sin_pitch, cos_pitch) = self.pitch.0.sin_cos();
        let (sin_yaw, cos_yaw) = self.yaw.0.sin_cos();

        // contradict with CameraSphere
        cgmath::Matrix4::look_to_rh(
            self.position,
            // current coordinate
            cgmath::Vector3::new(cos_pitch * cos_yaw, sin_pitch, cos_pitch * sin_yaw).normalize(),
            // my coordinate
            // cgmath::Vector3::new(cos_pitch * sin_yaw, sin_pitch, cos_pitch * cos_yaw).normalize(),
            cgmath::Vector3::unit_y(),
        )
    }

    pub fn gazer(&self) -> ([f32; 3], [f32; 3]) {
        let (sin_pitch, cos_pitch) = self.pitch.0.sin_cos();
        let (sin_yaw, cos_yaw) = self.yaw.0.sin_cos();
        // my coordinate
        // (self.position.into(), [cos_pitch * sin_yaw, sin_pitch, cos_pitch * cos_yaw])
        // current coordinate
        (
            self.position.into(),
            [cos_pitch * cos_yaw, sin_pitch, cos_pitch * sin_yaw],
        )
    }
}

#[derive(Debug)]
pub struct Projection {
    aspect: f32,
    fovy: cgmath::Rad<f32>,
    znear: f32,
    zfar: f32,
    width: f32,
    height: f32,
    settings: std::rc::Rc<std::cell::RefCell<crate::settings::Settings>>,
}

impl Projection {
    pub fn new<F: Into<cgmath::Rad<f32>>>(
        width: u32,
        height: u32,
        fovy: F,
        znear: f32,
        zfar: f32,
        settings: std::rc::Rc<std::cell::RefCell<crate::settings::Settings>>,
    ) -> Self {
        Self {
            aspect: width as f32 / height as f32,
            fovy: fovy.into(),
            znear,
            zfar,
            width: width as f32,
            height: height as f32,
            settings,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.aspect = width as f32 / height as f32;
        self.width = width as f32;
        self.height = height as f32;
    }

    pub fn mat(&self) -> cgmath::Matrix4<f32> {
        let settings = self.settings.borrow();
        // match ProjectionMode::Orthographic {
        match settings.projection_mode {
            ProjectionMode::Perspective => {
                cgmath::perspective(self.fovy, self.aspect, self.znear, self.zfar)
            }
            ProjectionMode::Orthographic => {
                cgmath::ortho(0.0, 1.0 * self.aspect, 0.0, 1.0, -1.0, 1.0)
            }
        }
    }

    pub fn calc_matrix(&self) -> cgmath::Matrix4<f32> {
        OPENGL_TO_WGPU_MATRIX * self.mat()
    }
}

#[derive(Debug)]
pub struct CameraController {
    // amount_left: f32,
    // amount_right: f32,
    // amount_forward: f32,
    // amount_backward: f32,
    // amount_up: f32,
    // amount_down: f32,
    pub rotate_horizontal: f32,
    pub rotate_vertical: f32,
    pub scroll: f32,
    // pub speed: f32,
    // pub sensitivity: f32,
    pub settings: std::rc::Rc<std::cell::RefCell<crate::settings::Settings>>,
    // mode: CameraMode,
    // pmode: CameraMode,
    pub sphere: CameraSphere,
    pub center: [f32; 3],
}

impl CameraController {
    pub fn new(
        settings: std::rc::Rc<std::cell::RefCell<crate::settings::Settings>>,
        // speed: f32,
        // sensitivity: f32,
        center: [f32; 3],
    ) -> Self {
        Self {
            // amount_left: 0.0,
            // amount_right: 0.0,
            // amount_forward: 0.0,
            // amount_backward: 0.0,
            // amount_up: 0.0,
            // amount_down: 0.0,
            rotate_horizontal: 0.0,
            rotate_vertical: 0.0,
            scroll: 0.0,
            // speed,
            // sensitivity,
            settings,
            // mode: CameraMode::Normal,
            // pmode: CameraMode::Normal,
            sphere: CameraSphere::default(),
            center,
        }
    }

    // pub fn process_keyboard(&mut self, key: winit::event::VirtualKeyCode, state: winit::event::ElementState) -> bool {
    pub fn process_keyboard(&mut self, event: &winit::event::KeyEvent) -> bool {
        // let amount = if state == ElementState::Pressed {
        //     1.0
        // } else {
        //     0.0
        // };

        // let is_pressed = event.state.is_pressed();

        if self.settings.borrow().show_download_dialog {
            false
        } else if let winit::keyboard::PhysicalKey::Code(keycode) = event.physical_key {
            match keycode {
                winit::keyboard::KeyCode::KeyN => {
                    self.settings.borrow_mut().camera_mode = CameraMode::Normal;
                    true
                }
                winit::keyboard::KeyCode::KeyR => {
                    self.settings.borrow_mut().camera_mode = CameraMode::Rotation;
                    true
                }
                winit::keyboard::KeyCode::KeyT => {
                    self.settings.borrow_mut().camera_mode = CameraMode::Translation;
                    true
                }
                _ => false,
            }
        } else {
            false
        }

        // match key {
        //     // winit::event::VirtualKeyCode::N => {
        //     //     self.mode = CameraMode::Normal;
        //     //     dbg!(self.mode);
        //     //     true
        //     // }
        //     // winit::event::VirtualKeyCode::R => {
        //     //     self.mode = CameraMode::Rotation;
        //     //     dbg!(self.mode);
        //     //     true
        //     // }
        //     // winit::event::VirtualKeyCode::T => {
        //     //     self.mode = CameraMode::Translation;
        //     //     dbg!(self.mode);
        //     //     true
        //     // }
        //     // _ => false,
        // }
        // false

        // match key {
        //     VirtualKeyCode::W | VirtualKeyCode::Up => {
        //         self.amount_forward = amount;
        //         true
        //     }
        //     VirtualKeyCode::S | VirtualKeyCode::Down => {
        //         self.amount_backward = amount;
        //         true
        //     }
        //     VirtualKeyCode::A | VirtualKeyCode::Left => {
        //         self.amount_left = amount;
        //         true
        //     }
        //     VirtualKeyCode::D | VirtualKeyCode::Right => {
        //         self.amount_right = amount;
        //         true
        //     }
        //     VirtualKeyCode::Space => {
        //         self.amount_up = amount;
        //         true
        //     }
        //     // VirtualKeyCode::LShift => {
        //     //     self.amount_down = amount;
        //     //     true
        //     // }
        //     _ => false,
        // }
    }

    pub fn process_mouse(&mut self, mouse_dx: f64, mouse_dy: f64) {
        self.rotate_horizontal = mouse_dx as f32;
        self.rotate_vertical = mouse_dy as f32;
    }

    pub fn process_scroll(&mut self, delta: &winit::event::MouseScrollDelta) {
        self.scroll = match delta {
            winit::event::MouseScrollDelta::LineDelta(_, scroll) => -scroll * 0.5,
            winit::event::MouseScrollDelta::PixelDelta(winit::dpi::PhysicalPosition {
                y: scroll,
                ..
            }) => -*scroll as f32,
        };
    }

    // pub fn process_event(&mut self, event: &winit::event) -> bool {
    //     // match event {
    //     //     winit::event::MouseScrollDelta => {
    //     //
    //     //         true
    //     //     }
    //     //     winit::event::KeyEvent => {
    //     //         let is_pressed = event.state.is_pressed();
    //     //         if let winit::keyboard::PhysicalKey::Code(keycode) = event.physical_key {
    //     //             match keycode {
    //     //                 winit::keyboard::KeyCode::KeyN => {
    //     //                     self.mode = CameraMode::Normal;
    //     //                     dbg!(self.mode);
    //     //                     true
    //     //                 }
    //     //                 winit::keyboard::KeyCode::KeyR => {
    //     //                     self.mode = CameraMode::Rotation;
    //     //                     dbg!(self.mode);
    //     //                     true
    //     //                 }
    //     //                 winit::keyboard::KeyCode::KeyT = > {
    //     //                     self.mode = CameraMode::Translation;
    //     //                     dbg!(self.mode);
    //     //                     true
    //     //                 }
    //     //             }
    //     //         }
    //     //     }
    //     // }
    //     // false
    // }

    pub fn mouse_gazer(
        &self,
        camera: &Camera,
        proj: &Projection,
        mouse_position: &[f32; 2],
        window_size: &winit::dpi::PhysicalSize<u32>,
    ) -> ([f32; 3], [f32; 3]) {
        let (_camera_gazer_p, camera_gazer_q) = camera.gazer();

        // normalize mouse_position
        // let norm_mouse_position = cgmath::Vector4::new(
        //     (mouse_position[0] / window_size.width as f32) * 2.0 - 1.0,
        //     1.0 - (mouse_position[1] / window_size.height as f32) * 2.0,
        //     0.0,
        //     1.0
        // );

        // scale with window size (seems to be ok if perspective)
        let norm_mouse_position = cgmath::Vector4::new(
            window_size.width as f32 / 2.0
                * ((mouse_position[0] / window_size.width as f32) * 2.0 - 1.0),
            window_size.height as f32 / 2.0
                * (1.0 - (mouse_position[1] / window_size.height as f32) * 2.0),
            // 0.0,
            -1.0,
            1.0,
        );

        // same with camera position
        // let norm_mouse_position = cgmath::Vector4::new(
        //     0.0,
        //     0.0,
        //     0.0,
        //     1.0
        // );

        // let viewport_mat = cgmath::Matrix4::new(
        //     window_size.width as f32 / 2.0, 0.0, 0.0, 0.0,
        //     0.0, window_size.height as f32 / - 2.0, 0.0, 0.0,
        //     0.0, 0.0, 1.0, 0.0,
        //     window_size.width as f32 / 2.0, window_size.height as f32 / 2.0, 0.0, 1.0,
        // );

        // norm_mouse_position to world coordinate
        let world_mouse_position = {
            let view_proj = OPENGL_TO_WGPU_MATRIX * proj.calc_matrix() * camera.calc_matrix();
            // let view_proj = viewport_mat * OPENGL_TO_WGPU_MATRIX * proj.calc_matrix() * camera.calc_matrix();
            let view_proj_inv = view_proj.invert().unwrap();
            view_proj_inv * norm_mouse_position
        };

        dbg!(&world_mouse_position);
        let world_mouse_position_r = [
            world_mouse_position.x / world_mouse_position.w,
            world_mouse_position.y / world_mouse_position.w,
            world_mouse_position.z / world_mouse_position.w,
        ];

        dbg!(&world_mouse_position_r);
        dbg!(&camera.position);

        (world_mouse_position_r, camera_gazer_q)

        // let nvec = self.sphere.nvec();
        // let mvec = self.sphere.mvec();
        // let SCALE_FACTOR_X: f32 = 2.0 / window_size.width as f32;
        // let SCALE_FACTOR_Y: f32 = 2.0 / window_size.height as f32;
        // let m_pos_r = [SCALE_FACTOR_X * (mouse_position[0] - window_size.width as f32 / 2.0), SCALE_FACTOR_Y * (-mouse_position[1] + window_size.height as f32 / 2.0)];
        // dbg!(&m_pos_r);
        // let mvec_r = [mvec[0] * m_pos_r[0], mvec[1] * m_pos_r[0], mvec[2] * m_pos_r[0]];
        // let nvec_r = [nvec[0] * m_pos_r[1], nvec[1] * m_pos_r[1], nvec[2] * m_pos_r[1]];
        // ([camera_gazer_p[0] + nvec_r[0] + mvec_r[0], camera_gazer_p[1] + nvec_r[1] + mvec_r[1], camera_gazer_p[2] + nvec_r[2] + mvec_r[2]],
        //  camera_gazer_q)
    }

    pub fn update_sphere(&mut self, position: [f32; 3]) {
        self.sphere.renew(position, self.center);
    }

    pub fn update_camera(&mut self, camera: &mut Camera, dt: std::time::Duration) {
        let dt = dt.as_secs_f32();

        let mut settings = self.settings.borrow_mut();
        match settings.camera_mode {
            CameraMode::Normal => {
                // Move forward/backward and left/right;
                // tmp coordinate
                let (yaw_sin, yaw_cos) = camera.yaw.0.sin_cos();
                // let forward = cgmath::Vector3::new(yaw_cos, 0.0, yaw_sin).normalize();
                // let right = cgmath::Vector3::new(-yaw_sin, 0.0, yaw_cos).normalize();
                // camera.position +=
                //     forward * (self.amount_forward - self.amount_backward) * self.speed * dt;
                // camera.position += right * (self.amount_right - self.amount_left) * self.speed * dt;

                let (pitch_sin, pitch_cos) = camera.pitch.0.sin_cos();
                let scrollward =
                    cgmath::Vector3::new(pitch_cos * yaw_cos, pitch_sin, pitch_cos * yaw_sin)
                        .normalize();
                camera.position += scrollward * self.scroll * settings.camera_speed as f32 * dt / 10.0;
                // camera.position += scrollward * self.scroll * self.speed * self.sensitivity * dt;
                // camera.position.y += (self.amount_up - self.amount_down) * self.speed * dt;

                camera.yaw += cgmath::Rad(self.rotate_horizontal) * settings.camera_speed as f32 * dt / 10.0;
                // camera.yaw += cgmath::Rad(self.rotate_horizontal) * self.settings.sensitivity * dt;

                camera.pitch += cgmath::Rad(-self.rotate_vertical) * settings.camera_speed as f32 * dt / 10.0;
                // camera.pitch += cgmath::Rad(-self.rotate_vertical) * self.sensitivity * dt;

                if camera.pitch < -cgmath::Rad(SAFE_FRAC_PI_2) {
                    camera.pitch = -cgmath::Rad(SAFE_FRAC_PI_2);
                } else if camera.pitch > cgmath::Rad(SAFE_FRAC_PI_2) {
                    camera.pitch = cgmath::Rad(SAFE_FRAC_PI_2);
                }
                settings.camera_pmode = CameraMode::Normal;
                self.scroll = 0.0;
                self.rotate_horizontal = 0.0;
                self.rotate_vertical = 0.0;
            }
            CameraMode::Rotation => {
                // self.sphere.renew(camera.position.into(), self.center);
                // self.sphere.theta += self.rotate_vertical / 1000.0;
                // self.sphere.phi += self.rotate_horizontal / 1000.0;
                // self.sphere.radius += self.scroll / 100.0;
                // if self.sphere.radius <= 0.001 {
                //     self.sphere.radius = 0.001;
                // }
                // let xyz = self.sphere.xyz(self.center);
                // let sxyz = self.sphere.sxyz();
                //
                // camera.position = cgmath::Point3::new(xyz[0], xyz[1], xyz[2]);
                // // dbg!(&sxyz[1].asin());
                // camera.pitch =
                //     cgmath::Rad::from(cgmath::Deg(-sxyz[1].asin() * 180.0 / std::f32::consts::PI));
                // // dbg!((sxyz[2]/sxyz[0]).atan());
                // camera.yaw = cgmath::Rad::from(cgmath::Deg(
                //     -(sxyz[2] / sxyz[0]).atan() * 180.0 / std::f32::consts::PI,
                // ));

                self.sphere.renew(camera.position.into(), self.center);
                self.sphere.radius += self.scroll * settings.camera_speed as f32 / 10.0;
                if self.sphere.radius <= 0.001 {
                    self.sphere.radius = 0.001;
                }
                // let xyz = self.sphere.xyz(self.center);
                let sxyz = self.sphere.sxyz();
                let nvec = self.sphere.nvec();
                let mvec = self.sphere.mvec();

                let q_theta =
                    crate::quaternion::Quaternion::new_rotater(mvec, -self.rotate_vertical * settings.camera_speed as f32 / 1000.0);
                let q_phi = crate::quaternion::Quaternion::new_rotater(
                    nvec,
                    -self.rotate_horizontal * settings.camera_speed as f32 / 1000.0,
                );
                let q_camera = crate::quaternion::Quaternion::new(sxyz[0], sxyz[1], sxyz[2], 0.0);
                let q_camera_r = q_theta * q_phi * q_camera * q_phi.inverse() * q_theta.inverse();
                let camera_position: [f32; 3] = q_camera_r.into();
                let camera_theta = camera_position[1].asin();
                let camera_phi = camera_position[0].atan2(camera_position[2]);
                let camera_position = [
                    self.center[0] + self.sphere.radius * camera_theta.cos() * camera_phi.sin(),
                    self.center[1] + self.sphere.radius * camera_theta.sin(),
                    self.center[2] + self.sphere.radius * camera_theta.cos() * camera_phi.cos(),
                ];
                camera.position =
                    cgmath::Point3::new(camera_position[0], camera_position[1], camera_position[2]);
                camera.pitch =
                    cgmath::Rad::from(cgmath::Deg(-camera_theta * 180.0 / std::f32::consts::PI));
                // for tmp coordinate
                camera.yaw = cgmath::Rad::from(cgmath::Deg(
                    -90.0 - camera_phi * 180.0 / std::f32::consts::PI,
                ));

                settings.camera_pmode = CameraMode::Rotation;
                self.scroll = 0.0;
                self.rotate_horizontal = 0.0;
                self.rotate_vertical = 0.0;
            }
            CameraMode::Translation => {
                match settings.camera_pmode {
                    CameraMode::Translation => {}
                    _ => {
                        self.sphere.renew(camera.position.into(), self.center);
                        // self.update_sphere(camera.position.into());
                    }
                }
                // let e_theta = self.sphere.e_theta();
                // let e_phi = self.sphere.e_phi();
                // camera.position[0] += self.rotate_vertical / 10.0 * e_theta[0]
                //     + self.rotate_horizontal / 10.0 * e_phi[0];
                // camera.position[1] += self.rotate_vertical / 10.0 * e_theta[1]
                //     + self.rotate_horizontal / 10.0 * e_phi[1];
                // camera.position[2] += self.rotate_vertical / 10.0 * e_theta[2]
                //     + self.rotate_horizontal / 10.0 * e_phi[2];
                // camera.position[0] += -self.scroll * self.sphere.ix / 1000.0;
                // camera.position[1] += -self.scroll * self.sphere.iy / 1000.0;
                // camera.position[2] += -self.scroll * self.sphere.iz / 1000.0;

                let nvec = self.sphere.nvec();
                let mvec = self.sphere.mvec();
                let sxyz = self.sphere.sxyz();
                let delta_x = (self.rotate_vertical * nvec[0] - self.rotate_horizontal * mvec[0]) * settings.camera_speed as f32 / 10.0;
                let delta_y = (self.rotate_vertical * nvec[1] - self.rotate_horizontal * mvec[1]) * settings.camera_speed as f32 / 10.0;
                let delta_z = (self.rotate_vertical * nvec[2] - self.rotate_horizontal * mvec[2]) * settings.camera_speed as f32 / 10.0;
                camera.position[0] += delta_x;
                camera.position[1] += delta_y;
                camera.position[2] += delta_z;
                self.center[0] += delta_x;
                self.center[1] += delta_y;
                self.center[2] += delta_z;
                camera.position[0] += -self.scroll * sxyz[0] * settings.camera_speed as f32 / 10.0;
                camera.position[1] += -self.scroll * sxyz[1] * settings.camera_speed as f32 / 10.0;
                camera.position[2] += -self.scroll * sxyz[2] * settings.camera_speed as f32 / 10.0;
                settings.camera_pmode = CameraMode::Translation;
                self.scroll = 0.0;
                self.rotate_horizontal = 0.0;
                self.rotate_vertical = 0.0;
            }
        }
        // println!("theta: {:?}, phi: {:?}", self.sphere.theta, self.sphere.phi);
        // println!("pitch: {:?}, yaw: {:?}", camera.pitch, camera.yaw);
    }
}

#[derive(Debug, Clone)]
pub struct CameraSphere {
    // ix: f32,
    // iy: f32,
    // iz: f32,
    radius: f32,
    theta: f32,
    phi: f32,
}

impl CameraSphere {
    pub fn renew(&mut self, xyz: [f32; 3], center: [f32; 3]) {
        // get (ix, iy, iz) (the center is center)
        let (ix, iy, iz) = (xyz[0] - center[0], xyz[1] - center[1], xyz[2] - center[2]);

        // get \phi and \theta from base point
        let radius = (ix.powi(2) + iy.powi(2) + iz.powi(2)).sqrt();
        // let theta = (iz / radius).acos();
        // let phi = (iy / ix).atan();
        // let phi = (ix / iz).atan();
        let phi = ix.atan2(iz);
        let theta = (iy / radius).asin();

        // self.ix = ix;
        // self.iy = iy;
        // self.iz = iz;
        self.radius = radius;
        self.theta = theta;
        self.phi = phi;
    }

    // pub fn e_theta(&self) -> [f32; 3] {
    //     [
    //         self.theta.cos() * self.phi.cos(),
    //         self.theta.cos() * self.phi.sin(),
    //         -self.theta.sin(),
    //     ]
    // }
    //
    // pub fn e_phi(&self) -> [f32; 3] {
    //     [-self.phi.sin(), self.phi.cos(), 0.0]
    // }

    // real coordinate (x, y, z)
    pub fn xyz(&self, center: [f32; 3]) -> [f32; 3] {
        // let x = center[0] + self.radius * self.theta.sin() * self.phi.cos();
        // let y = center[1] + self.radius * self.theta.sin() * self.phi.sin();
        // let z = center[2] + self.radius * self.theta.cos();
        let x = center[0] + self.radius * self.theta.cos() * self.phi.sin();
        let y = center[1] + self.radius * self.theta.sin();
        let z = center[2] + self.radius * self.theta.cos() * self.phi.cos();
        [x, y, z]
    }

    // ideal coordinate (ix, iy, iz)
    pub fn sxyz(&self) -> [f32; 3] {
        // let x = self.theta.sin() * self.phi.cos();
        // let y = self.theta.sin() * self.phi.sin();
        // let z = self.theta.cos();
        let x = self.theta.cos() * self.phi.sin();
        let y = self.theta.sin();
        let z = self.theta.cos() * self.phi.cos();
        [x, y, z]
    }

    // theta(pitch) direction
    pub fn nvec(&self) -> [f32; 3] {
        [
            -self.theta.sin() * self.phi.sin(),
            self.theta.cos(),
            -self.theta.sin() * self.phi.cos(),
        ]
    }

    // phi(yaw) direction
    pub fn mvec(&self) -> [f32; 3] {
        [self.phi.cos(), 0.0, -self.phi.sin()]
    }
}

impl Default for CameraSphere {
    fn default() -> Self {
        Self {
            // ix: 0.0,
            // iy: 0.0,
            // iz: 0.0,
            radius: 0.0,
            theta: 0.0,
            phi: 0.0,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_position: [f32; 4],
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        Self {
            view_position: [0.0; 4],
            view_proj: cgmath::Matrix4::identity().into(),
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera, projection: &Projection) {
        self.view_position = camera.position.to_homogeneous().into();
        self.view_proj = (projection.calc_matrix() * camera.calc_matrix()).into();
    }
}

impl Default for CameraUniform {
    fn default() -> Self {
        Self::new()
    }
}
