use wgpu::util::DeviceExt;

#[derive(Debug)]
pub struct UnitRender {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_indices: u32,
}

impl UnitRender {
    pub fn new(device: &wgpu::Device, vertices: &Vec<Vertex>, indices: &Vec<Index>) -> Self {
        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(vertices),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            },
        );
        let index_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(indices),
                usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
            }
        );
        Self {
            vertex_buffer,
            index_buffer,
            num_indices: indices.len() as u32,
        }
    }
}


#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub color: [f32; 3],
}

impl Vertex {
    const ATTRIBUTES: [wgpu::VertexAttribute; 3] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3, 2 => Float32x3];
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTES,
        }
    }
    // purple -> blue ??
    // pub fn desc() -> wgpu::VertexBufferLayout<'static> {
    //     wgpu::VertexBufferLayout {
    //         array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
    //         step_mode: wgpu::VertexStepMode::Vertex,
    //         attributes: &[
    //             wgpu::VertexAttribute {
    //                 offset: 0,
    //                 shader_location: 0,
    //                 format: wgpu::VertexFormat::Float32x3,
    //             },
    //             wgpu::VertexAttribute {
    //                 offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
    //                 shader_location: 1,
    //                 format: wgpu::VertexFormat::Float32x3,
    //             },
    //             wgpu::VertexAttribute {
    //                 offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
    //                 shader_location: 2,
    //                 format: wgpu::VertexFormat::Float32x3,
    //             },
    //         ],
    //     }
    // }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Index {
    pub ids: u16,
}

#[derive(Copy, Clone, Debug)]
pub enum ModelType {
    Pentagon,
    Sphere,
    Line,
}

// For test case
#[derive(Clone, Debug)]
pub struct Pentagon {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<Index>,
}

impl Default for Pentagon {
    fn default() -> Self {
        Self {
            vertices: vec![
                Vertex {
                    position: [-0.0868241, 0.49240386, 0.0],
                    normal: [0.0, 0.0, 1.0],
                    color: [0.5, 0.0, 0.5],
                }, // A
                Vertex {
                    position: [-0.49513406, 0.06958647, 0.0],
                    normal: [0.0, 0.0, 1.0],
                    color: [0.5, 0.0, 0.5],
                }, // B
                Vertex {
                    position: [-0.21918549, -0.44939706, 0.0],
                    normal: [0.0, 0.0, 1.0],
                    color: [0.5, 0.0, 0.5],
                }, // C
                Vertex {
                    position: [0.35966998, -0.3473291, 0.0],
                    normal: [0.0, 0.0, 1.0],
                    color: [0.5, 0.0, 0.5],
                }, // D
                Vertex {
                    position: [0.44147372, 0.2347359, 0.0],
                    normal: [0.0, 0.0, 1.0],
                    color: [0.5, 0.0, 0.5],
                }, // E
            ],
            indices: vec![
                Index { ids: 0 },
                Index { ids: 1 },
                Index { ids: 4 },
                Index { ids: 1 },
                Index { ids: 2 },
                Index { ids: 4 },
                Index { ids: 2 },
                Index { ids: 3 },
                Index { ids: 4 },
            ],
        }
    }
}

// cannot visualize (primitive topology? light? depth?)
// #[derive(Clone, Debug)]
// pub struct Point {
//     pub color: [f32; 3],
//     pub vertices: Vec<Vertex>,
//     pub indices: Vec<Index>,
// }
//
// impl Point {
//     pub fn new(color: [f32; 3], point: Vec<[f32; 3]>) -> Self {
//         let mut vertices = Vec::new();
//         let mut indices = Vec::new();
//         for i in 0..point.len() {
//             vertices.push(
//                 Vertex {
//                     position: point[i],
//                     normal: point[i],
//                     color: color,
//                 }
//             );
//             indices.push(
//                 Index {
//                     ids: i as u16,
//                 }
//             );
//         }
//         Self {
//             color,
//             vertices,
//             indices,
//         }
//     }
// }

#[derive(Clone, Debug)]
pub struct Line {
    pub color: [f32; 3],
    pub vertices: Vec<Vertex>,
    pub indices: Vec<Index>,
}

impl Line {
    pub fn new(color: [f32; 3], point: Vec<[f32; 3]>) -> Self {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        for (i, p) in point.iter().enumerate() {
            vertices.push(Vertex {
                position: *p,
                normal: *p,
                color,
            });
            indices.push(Index { ids: i as u16 });
        }
        Self {
            color,
            vertices,
            indices,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Sphere {
    pub radius: f32,
    pub center: [f32; 3],
    pub color: [f32; 3],
    pub accuracy: u32,
    pub vertices: Vec<Vertex>,
    pub indices: Vec<Index>,
}

impl Sphere {
    pub fn new(radius: f32, center: [f32; 3], color: [f32; 3], accuracy: u32) -> Self {
        // decompose sphere to vertices
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        // x = r * sin \theta/N * cos \phi/N + center[0]
        // y = r * sin \theta/N * sin \phi/N + center[1]
        // z = r * cos \theta/N + center[2]
        // where 0 <= \theta <= \pi, 0 <= \phi <= 2 \pi

        fn gen_xyz(radius: f32, n_accuracy: f32, m_accuracy: f32) -> (f32, f32, f32) {
            let theta_n_a = std::f32::consts::PI * n_accuracy;
            let phi_m_a = 2.0 * std::f32::consts::PI * m_accuracy;
            let theta_n_a_cos = theta_n_a.cos();
            let theta_n_a_sin = theta_n_a.sin();
            let phi_m_a_cos = phi_m_a.cos();
            let phi_m_a_sin = phi_m_a.sin();
            let x = radius * theta_n_a_sin * phi_m_a_cos;
            let y = radius * theta_n_a_sin * phi_m_a_sin;
            let z = radius * theta_n_a_cos;
            (x, y, z)
        }

        fn gen_vertex(
            radius: f32,
            n_accuracy: f32,
            m_accuracy: f32,
            center: [f32; 3],
            color: [f32; 3],
        ) -> Vertex {
            let (x, y, z, n_x, n_y, n_z) = {
                let (x, y, z) = gen_xyz(radius, n_accuracy, m_accuracy);
                (
                    x + center[0],
                    y + center[1],
                    z + center[2],
                    x - center[0],
                    y - center[1],
                    z - center[2],
                )
            };
            Vertex {
                position: [x, y, z],
                normal: [n_x, n_y, n_z],
                color,
            }
        }

        for n in 0..accuracy {
            for m in 0..accuracy {
                let n2 = n + 1;
                let m2 = m + 1;
                vertices.push(gen_vertex(
                    radius,
                    n as f32 / accuracy as f32,
                    m as f32 / accuracy as f32,
                    center,
                    color,
                ));
                vertices.push(gen_vertex(
                    radius,
                    n2 as f32 / accuracy as f32,
                    m as f32 / accuracy as f32,
                    center,
                    color,
                ));
                vertices.push(gen_vertex(
                    radius,
                    n as f32 / accuracy as f32,
                    m2 as f32 / accuracy as f32,
                    center,
                    color,
                ));
                vertices.push(gen_vertex(
                    radius,
                    n2 as f32 / accuracy as f32,
                    m2 as f32 / accuracy as f32,
                    center,
                    color,
                ));

                let csum = 4 * (accuracy * n + m);

                // triangle-1
                indices.push(Index { ids: csum as u16 });
                indices.push(Index {
                    ids: (1 + csum) as u16,
                });
                indices.push(Index {
                    ids: (2 + csum) as u16,
                });
                // triangle-2
                indices.push(Index {
                    ids: (3 + csum) as u16,
                });
                indices.push(Index {
                    ids: (2 + csum) as u16,
                });
                indices.push(Index {
                    ids: (1 + csum) as u16,
                });
            }
        }

        Self {
            radius,
            center,
            color,
            accuracy,
            vertices,
            indices,
        }
    }
}
