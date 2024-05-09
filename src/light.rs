#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LightUniform {
    specular_color: [f32; 4],
    ambient_intensity: f32,
    diffuse_intensity: f32,
    specular_intensity: f32,
    specular_shininess: f32,
    // is_two_side: i32,
}

impl Default for LightUniform {
    fn default() -> Self {
        Self {
            specular_color: [0.0, 0.0, 0.0, 0.0],
            ambient_intensity: 0.2,
            diffuse_intensity: 0.6,
            specular_intensity: 0.4,
            specular_shininess: 32.0,
            // is_two_side: 1,
        }
    }
}
