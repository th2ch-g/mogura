#[derive(Clone, Debug, Copy, PartialEq)]
pub struct Quaternion {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl Quaternion {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }
    // pub fn to3d(&self) -> [f32; 3] {
    //     [self.x, self.y, self.z]
    // }
    pub fn norm(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt()
    }
    pub fn conjugate(&self) -> Quaternion {
        Quaternion {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: self.w,
        }
    }
    pub fn inverse(&self) -> Quaternion {
        let norm = self.norm();
        Quaternion {
            x: -self.x / norm,
            y: -self.y / norm,
            z: -self.z / norm,
            w: self.w / norm,
        }
    }
    pub fn new_rotater(axis: [f32; 3], angle: f32) -> Self {
        Self {
            x: (angle / 2.0).sin() * axis[0],
            y: (angle / 2.0).sin() * axis[1],
            z: (angle / 2.0).sin() * axis[2],
            w: (angle / 2.0).cos(),
        }
    }
}

// impl Into<[f32; 3]> for Quaternion {
//     fn into(self) -> [f32; 3] {
//         [self.x, self.y, self.z]
//     }
// }

impl From<Quaternion> for [f32; 3] {
    fn from(val: Quaternion) -> Self {
        [val.x, val.y, val.z]
    }
}

impl std::ops::Add for Quaternion {
    type Output = Quaternion;
    fn add(self, other: Quaternion) -> Quaternion {
        Quaternion {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl std::ops::Sub for Quaternion {
    type Output = Quaternion;
    fn sub(self, other: Quaternion) -> Quaternion {
        Quaternion {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl std::ops::Mul for Quaternion {
    type Output = Quaternion;
    fn mul(self, other: Quaternion) -> Quaternion {
        Quaternion {
            x: self.w * other.x + self.x * other.w + self.y * other.z - self.z * other.y,
            y: self.w * other.y + self.y * other.w + self.z * other.x - self.x * other.z,
            z: self.w * other.z + self.z * other.w + self.x * other.y - self.y * other.x,
            w: self.w * other.w - self.x * other.x - self.y * other.y - self.z * other.z,
        }
    }
}
