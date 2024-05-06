pub mod arg;
pub mod bond;
pub mod camera;
pub mod egui_gui;
pub mod egui_renderer;
pub mod light;
pub mod model;
pub mod molecules;
pub mod pdb;
pub mod quaternion;
pub mod settings;

use wgpu::util::DeviceExt;

// State for wgpu
pub struct State {
    settings: std::rc::Rc<std::cell::RefCell<settings::Settings>>,
    pdbsystem: pdb::PDBSystem,
    window: winit::window::Window,
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    render_pipeline: wgpu::RenderPipeline,
    camera: camera::Camera,
    projection: camera::Projection,
    vertex_buffers: Vec<wgpu::Buffer>,
    index_buffers: Vec<wgpu::Buffer>,
    num_indices: Vec<u32>,
    camera_controller: camera::CameraController,
    camera_uniform: camera::CameraUniform,
    camera_buffer: wgpu::Buffer,
    light_buffer: wgpu::Buffer,
    light_uniform: light::LightUniform,
    // camera_bind_group: wgpu::BindGroup,
    uniform_bind_group: wgpu::BindGroup,
    size: winit::dpi::PhysicalSize<u32>,
    mouse_pressed: bool,
    is_touched: bool,
    mouse_position: [f32; 2],
    egui_renderer: egui_renderer::EguiRenderer,
    egui_gui: egui_gui::EguiGUI,
}

impl State {
    pub async fn new(window: winit::window::Window, pdbfile: String) -> Self {
        let settings = std::rc::Rc::new(std::cell::RefCell::new(settings::Settings::new(pdbfile)));
        let (input_pdb, _errors) = pdbtbx::open(
            &settings.borrow().pdbfile,
            pdbtbx::StrictnessLevel::Loose, // Strict, Medium, Loose
        )
        .unwrap();

        let mut pdbsystem = pdb::PDBSystem::from(&input_pdb);
        pdbsystem.update_bonds_all();
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let surface = unsafe { instance.create_surface(&window).unwrap() };

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        // print backends info
        dbg!(&adapter.get_info());

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    // from wgpu
                    // WebGL doesn't support all of wgpu's features, so if
                    // we're building for the web we'll have to disable some.
                    limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        // from wgpu
        // Shader code in this tutorial assumes an Srgb surface texture. Using a different
        // one will result all the colors comming out darker. If you want to support non
        // Srgb surfaces, you'll need to account for that when drawing to the frame.
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };

        surface.configure(&device, &config);

        let camera = camera::Camera::new([0.0, 200.0, 0.0], cgmath::Deg(-90.0), cgmath::Deg(-90.0));
        // let camera = camera::Camera::new((0.5, 5.0, 10.0), cgmath::Deg(-90.0), cgmath::Deg(-20.0));
        let projection = camera::Projection::new(
            config.width,
            config.height,
            cgmath::Deg(45.0),
            0.1,
            100.0,
            settings.clone(),
        );
        let camera_controller =
            camera::CameraController::new(settings.clone(), 4.0, 0.4, pdbsystem.center());
        // let camera_controller = camera::CameraController::new(4.0, 0.4);
        // let camera_controller = camera::CameraController::new(1.0, 0.1); // more sensitive

        let mut camera_uniform = camera::CameraUniform::new();
        camera_uniform.update_view_proj(&camera, &projection);

        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let light_uniform = light::LightUniform::new();
        let light_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Light Buffer"),
            contents: bytemuck::cast_slice(&[light_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let uniform_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                ],
                label: Some("uniform bind group layout"),
            });

        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &uniform_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: camera_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: light_buffer.as_entire_binding(),
                },
            ],
            label: Some("Uniform bind group"),
        });

        // let camera_bind_group_layout =
        //     device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        //         entries: &[wgpu::BindGroupLayoutEntry {
        //             binding: 0,
        //             visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
        //             ty: wgpu::BindingType::Buffer {
        //                 ty: wgpu::BufferBindingType::Uniform,
        //                 has_dynamic_offset: false,
        //                 min_binding_size: None,
        //             },
        //             count: None,
        //         }],
        //         label: Some("camera_bind_group_layout"),
        //     });
        //
        // let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        //     layout: &camera_bind_group_layout,
        //     entries: &[wgpu::BindGroupEntry {
        //         binding: 0,
        //         resource: camera_buffer.as_entire_binding(),
        //     }],
        //     label: Some("camera_bind_group"),
        // });
        //
        // let light_uniform = light::LightUniform::new();
        // let light_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        //     label: Some("Light Buffer"),
        //     contents: bytemuck::cast_slice(&[light_uniform]),
        //     usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        // });
        //
        // let light_bind_group_layout =
        //     device.create_bind_group_layout(&wgpu::BindGrouplayoutDescriptor {
        //         entries: &[wgpu::BindGroupLayoutEntry {
        //             binding: 0,
        //             visility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
        //             ty: wgpu::BindingType::Buffer {
        //                 ty: wgpu::BufferBindingType::Uniform,
        //                 has_dynamic_offset: false,
        //                 min_binding_size: None,
        //             },
        //             count: None,
        //         }],
        //         label: None,
        //     });
        //
        // let light_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        //     layout: &light_bind_group_layout,
        //     entries: &[wgpu::BindGroupEntry {
        //         binding: 0,
        //         resource: light_buffer.as_entire_binding(),
        //     }],
        //     label: None,
        // });

        let render_pipeline = {
            let render_pipeline_layout =
                device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Render Pipeline Layout"),
                    bind_group_layouts: &[
                        // &camera_bind_group_layout,
                        // &light_bind_group_layout,
                        &uniform_bind_group_layout,
                    ],
                    push_constant_ranges: &[],
                });

            let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("./shader/shader.wgsl").into()),
            });

            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Render Pipeline"),
                layout: Some(&render_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: &[model::Vertex::desc()],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
                    targets: &[Some(wgpu::ColorTargetState {
                        format: config.format,
                        blend: Some(wgpu::BlendState {
                            alpha: wgpu::BlendComponent::REPLACE,
                            color: wgpu::BlendComponent::REPLACE,
                        }),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                // sphere,
                // primitive: wgpu::PrimitiveState {
                //     topology: wgpu::PrimitiveTopology::TriangleList,
                //     strip_index_format: None,
                //     front_face: wgpu::FrontFace::Ccw,
                //     cull_mode: Some(wgpu::Face::Back),
                //     polygon_mode: wgpu::PolygonMode::Fill,
                //     unclipped_depth: false,
                //     conservative: false,
                // },
                // line
                primitive: wgpu::PrimitiveState {
                    // topology: wgpu::PrimitiveTopology::LineStrip,
                    topology: wgpu::PrimitiveTopology::LineList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: Some(wgpu::Face::Back),
                    polygon_mode: wgpu::PolygonMode::Fill,
                    unclipped_depth: false,
                    conservative: false,
                },
                // point
                // primitive: wgpu::PrimitiveState {
                //     topology: wgpu::PrimitiveTopology::PointList,
                //     strip_index_format: None,
                //     front_face: wgpu::FrontFace::Ccw,
                //     cull_mode: Some(wgpu::Face::Back),
                //     polygon_mode: wgpu::PolygonMode::Fill,
                //     unclipped_depth: false,
                //     conservative: false,
                // },
                // depth_stencil: None,
                depth_stencil: Some(wgpu::DepthStencilState {
                    format: wgpu::TextureFormat::Depth32Float,
                    depth_write_enabled: true,
                    depth_compare: wgpu::CompareFunction::LessEqual,
                    stencil: wgpu::StencilState::default(),
                    bias: wgpu::DepthBiasState::default(),
                }),
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                multiview: None,
            })
        };

        // let model = model::Pentagon::default();

        // let model1 = model::Sphere::new(0.5, [0.5, 0.5, 0.5], [0.0, 0.5, 1.0], 20);
        // let model2 = model::Sphere::new(0.5, [1.5, 1.5, 1.5], [0.0, 0.5, 1.0], 20);
        //
        // let vertex_buffer1 = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        //     label: Some("Vertex Buffer"),
        //     contents: bytemuck::cast_slice(&model1.vertices),
        //     usage: wgpu::BufferUsages::VERTEX,
        // });
        //
        // let index_buffer1 = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        //     label: Some("Index Buffer"),
        //     contents: bytemuck::cast_slice(&model1.indices),
        //     usage: wgpu::BufferUsages::INDEX,
        // });
        //
        // let vertex_buffer2 = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        //     label: Some("Vertex Buffer"),
        //     contents: bytemuck::cast_slice(&model2.vertices),
        //     usage: wgpu::BufferUsages::VERTEX,
        // });
        //
        // let index_buffer2 = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        //     label: Some("Index Buffer"),
        //     contents: bytemuck::cast_slice(&model2.indices),
        //     usage: wgpu::BufferUsages::INDEX,
        // });
        //
        // let models = vec![model1, model2];
        // let vertex_buffers = vec![vertex_buffer1, vertex_buffer2];
        // let index_buffers = vec![index_buffer1, index_buffer2];
        //
        // // let num_indices = model.indices.len() as u32;
        // let num_indices = models.iter().map(|m| m.indices.len() as u32).collect::<Vec<u32>>();

        // let water = molecules::SphereWater::default();
        // let mut vertex_buffers = Vec::new();
        // let mut index_buffers = Vec::new();
        // let (vertices, indices) = water.gen_vertex_index();
        // let num_indices = indices.iter().map(|i| i.len() as u32).collect::<Vec<u32>>();
        // for i in 0..3 {
        //     vertex_buffers.push(
        //         device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        //             label: Some("Vertex Buffer"),
        //             contents: bytemuck::cast_slice(&vertices[i]),
        //             usage: wgpu::BufferUsages::VERTEX,
        //         })
        //     );
        //     index_buffers.push(
        //         device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        //             label: Some("Index Buffer"),
        //             contents: bytemuck::cast_slice(&indices[i]),
        //             usage: wgpu::BufferUsages::INDEX,
        //         })
        //     );
        // }

        // line
        // let model = model::Line::new(
        //     // [0.0, 0.5, 1.0],
        //     [0.0, 0.0, 0.0],
        //     vec![
        //         [0.0, 0.0, 0.0],
        //         [1.0, 0.0, 1.0],
        //         [2.0, 1.0, 0.0],
        //     ],
        // );
        // let mut vertex_buffers = Vec::new();
        // let mut index_buffers = Vec::new();
        // vertex_buffers.push(device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        //     label: Some("Vertex Buffer"),
        //     contents: bytemuck::cast_slice(&model.vertices),
        //     usage: wgpu::BufferUsages::VERTEX,
        // }));
        // index_buffers.push(device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        //     label: Some("Index Buffer"),
        //     contents: bytemuck::cast_slice(&model.indices),
        //     usage: wgpu::BufferUsages::INDEX,
        // }));
        // let num_indices = vec![model.indices.len() as u32];

        // Linewater
        // let water = molecules::LineWater::default();
        // let (vertices, indices) = water.gen_vertex_index();
        // let mut vertex_buffers = Vec::new();
        // let mut index_buffers = Vec::new();
        // let num_indices = vec![indices.len() as u32];
        // vertex_buffers.push(
        //     device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        //         label: Some("Vertex Buffer"),
        //         contents: bytemuck::cast_slice(&vertices),
        //         usage: wgpu::BufferUsages::VERTEX,
        //     })
        // );
        // index_buffers.push(
        //     device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        //         label: Some("Index Buffer"),
        //         contents: bytemuck::cast_slice(&indices),
        //         usage: wgpu::BufferUsages::INDEX,
        //     })
        // );

        // line water from pdb
        // let pdbatoms = &pdbsystem
        //     .models[0]
        //     .chains[0]
        //     .residues[0]
        //     .atoms;
        // let water = molecules::LineWater {
        //     H1_O_H2: model::Line::new(
        //         [0.0, 0.5, 1.0],
        //         vec![
        //             [pdbatoms[0].x, pdbatoms[0].y, pdbatoms[0].z],
        //             [pdbatoms[1].x, pdbatoms[1].y, pdbatoms[1].z],
        //             [pdbatoms[2].x, pdbatoms[2].y, pdbatoms[2].z],
        //         ]
        //     )
        // };
        // let (vertices, indices) = water.gen_vertex_index();
        // let mut vertex_buffers = Vec::new();
        // let mut index_buffers = Vec::new();
        // let num_indices = vec![indices.len() as u32];
        // vertex_buffers.push(
        //     device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        //         label: Some("Vertex Buffer"),
        //         contents: bytemuck::cast_slice(&vertices),
        //         usage: wgpu::BufferUsages::VERTEX,
        //     })
        // );
        // index_buffers.push(
        //     device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        //         label: Some("Index Buffer"),
        //         contents: bytemuck::cast_slice(&indices),
        //         usage: wgpu::BufferUsages::INDEX,
        //     })
        // );

        // let line_model = pdbsystem.gen_line_model_test();
        // let mut vertex_buffers = Vec::new();
        // let mut index_buffers = Vec::new();
        // for i in 0..line_model.len() {
        //     vertex_buffers.push(
        //         device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        //             label: Some("Vertex Buffer"),
        //             contents: bytemuck::cast_slice(&line_model[i].vertices),
        //             usage: wgpu::BufferUsages::VERTEX,
        //         })
        //     );
        //     index_buffers.push(
        //         device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        //             label: Some("Index Buffer"),
        //             contents: bytemuck::cast_slice(&line_model[i].indices),
        //             usage: wgpu::BufferUsages::INDEX,
        //         })
        //     );
        // }
        // let num_indices = line_model.iter().map(|m| m.indices.len() as u32).collect::<Vec<u32>>();

        // let line_model = pdbsystem.gen_line_model();
        // let mut vertex_buffers = Vec::new();
        // let mut index_buffers = Vec::new();
        // for i in 0..line_model.len() {
        //     vertex_buffers.push(
        //         device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        //             label: Some("Vertex Buffer"),
        //             contents: bytemuck::cast_slice(&line_model[i].vertices),
        //             usage: wgpu::BufferUsages::VERTEX,
        //         }),
        //     );
        //     index_buffers.push(
        //         device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        //             label: Some("Index Buffer"),
        //             contents: bytemuck::cast_slice(&line_model[i].indices),
        //             usage: wgpu::BufferUsages::INDEX,
        //         }),
        //     );
        // }
        // let mut num_indices = line_model
        //     .iter()
        //     .map(|m| m.indices.len() as u32)
        //     .collect::<Vec<u32>>();

        // line model
        pdbsystem.set_line_model();
        let mut vertex_buffers = Vec::new();
        let mut index_buffers = Vec::new();
        vertex_buffers.push(
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&pdbsystem.vertices),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            }),
        );
        index_buffers.push(
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(&pdbsystem.indecies),
                usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
            }),
        );
        let mut num_indices = vec![pdbsystem.indecies.len() as u32];

        // center
        let center = model::Sphere::new(1.0, pdbsystem.center(), [1.0, 0.0, 0.0], 100);
        vertex_buffers.push(
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&center.vertices),
                usage: wgpu::BufferUsages::VERTEX,
            }),
        );
        index_buffers.push(
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(&center.indices),
                usage: wgpu::BufferUsages::INDEX,
            }),
        );
        num_indices.push(center.indices.len() as u32);

        // axises
        let xaxis = model::Line::new([1.0, 0.0, 0.0], vec![[0.0, 0.0, 0.0], [100.0, 0.0, 0.0]]);
        let yaxis = model::Line::new([0.0, 1.0, 0.0], vec![[0.0, 0.0, 0.0], [0.0, 100.0, 0.0]]);
        let zaxis = model::Line::new([0.0, 0.0, 1.0], vec![[0.0, 0.0, 0.0], [0.0, 0.0, 100.0]]);

        vertex_buffers.push(
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&xaxis.vertices),
                usage: wgpu::BufferUsages::VERTEX,
            }),
        );
        index_buffers.push(
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(&xaxis.indices),
                usage: wgpu::BufferUsages::INDEX,
            }),
        );
        vertex_buffers.push(
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&yaxis.vertices),
                usage: wgpu::BufferUsages::VERTEX,
            }),
        );
        index_buffers.push(
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(&yaxis.indices),
                usage: wgpu::BufferUsages::INDEX,
            }),
        );
        vertex_buffers.push(
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&zaxis.vertices),
                usage: wgpu::BufferUsages::VERTEX,
            }),
        );
        index_buffers.push(
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(&zaxis.indices),
                usage: wgpu::BufferUsages::INDEX,
            }),
        );
        num_indices.push(xaxis.indices.len() as u32);
        num_indices.push(yaxis.indices.len() as u32);
        num_indices.push(zaxis.indices.len() as u32);

        let egui_renderer =
            egui_renderer::EguiRenderer::new(&device, config.format, None, 1, &window);

        let egui_gui = egui_gui::EguiGUI::new(settings.clone());

        dbg!("state ok!");

        Self {
            settings,
            pdbsystem,
            window,
            surface,
            device,
            queue,
            config,
            render_pipeline,
            vertex_buffers,
            index_buffers,
            num_indices,
            camera,
            projection,
            camera_controller,
            camera_uniform,
            camera_buffer,
            light_uniform,
            light_buffer,
            // camera_bind_group,
            uniform_bind_group,
            size,
            mouse_pressed: false,
            mouse_position: [0.0, 0.0],
            is_touched: false,
            egui_renderer,
            egui_gui,
        }
    }

    pub fn window(&self) -> &winit::window::Window {
        &self.window
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.projection.resize(new_size.width, new_size.height);
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn input(&mut self, event: &winit::event::KeyEvent) -> bool {
        let is_pressed = event.state.is_pressed();
        self.mouse_pressed = is_pressed;
        if is_pressed {
            self.camera_controller.process_keyboard(event)
        } else {
            is_pressed
        }

        // match event {
        //     winit::event::WindowEvent::KeyboardInput {
        //         input:
        //             winit::event::WindowEvent::KeyboardInput {
        //                 virtual_keycode: Some(key),
        //                 state,
        //                 ..
        //             },
        //         ..
        //     } => self.camera_controller.process_keyboard(*key, *state),
        //     winit::event::WindowEvent::MouseWheel { delta, .. } => {
        //         self.camera_controller.process_scroll(delta);
        //         true
        //     }
        //     winit::event::WindowEvent::MouseInput {
        //         button: winit::event::MouseButton::Left,
        //         state,
        //         ..
        //     } => {
        //         self.mouse_pressed = *state == winit::event::ElementState::Pressed;
        //         true
        //     }
        //     _ => false,
        // }
    }

    pub fn update(&mut self, dt: std::time::Duration) {
        // if !self.is_touched {
        //     self.camera_controller.update_camera(&mut self.camera, dt);
        // }
        self.camera_controller.update_camera(&mut self.camera, dt);
        self.camera_uniform
            .update_view_proj(&self.camera, &self.projection);
        self.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[self.camera_uniform]),
        );
        self.queue.write_buffer(
            &self.light_buffer,
            0,
            bytemuck::cast_slice(&[self.light_uniform]),
        );
        // assume only line model
        match self.settings.borrow().move_mode {
            // crate::settings::MoveMode::MoveWithNNP | crate::settings::MoveMode::MoveWithoutNNP => {
            crate::settings::MoveMode::Move => {
                self.pdbsystem.set_line_model();
                self.queue.write_buffer(
                    &self.vertex_buffers[0],
                    0,
                    bytemuck::cast_slice(&self.pdbsystem.vertices),
                );
            }
            _ => {}
        }
    }

    pub fn check_is_touched(&mut self) {
        dbg!(&self.is_touched);
        let move_mode = self.settings.borrow().move_mode;
        let group_to_select = self.settings.borrow().group_to_select;
        self.settings.borrow_mut().selected_group.atoms = match move_mode {
            // settings::MoveMode::MoveWithNNP | settings::MoveMode::MoveWithoutNNP => {
            settings::MoveMode::Move => {
                let mouse_gazer = self.camera_controller.mouse_gazer(
                    &self.camera,
                    &self.projection,
                    &self.mouse_position,
                    &self.window().inner_size(),
                );
                self.pdbsystem.which_group_is_selected(
                    mouse_gazer,
                    &mut self.is_touched,
                    &group_to_select,
                )
            }
            settings::MoveMode::Off => {
                self.is_touched = false;
                std::collections::HashSet::new()
            }
        }
    }

    pub fn mover(&mut self, delta_x: f64, delta_y: f64) {
        let settings = self.settings.borrow();
        self.camera_controller
            .update_sphere(self.camera.position.into());
        let nvec = self.camera_controller.sphere.nvec();
        let mvec = self.camera_controller.sphere.mvec();
        match settings.move_mode {
            // crate::settings::MoveMode::MoveWithNNP => {
            //     self.pdbsystem.move_with_nnp(
            //         delta_x as f32,
            //         delta_y as f32,
            //         (nvec, mvec),
            //         &settings.selected_group,
            //     );
            // }
            // crate::settings::MoveMode::MoveWithoutNNP => {
            crate::settings::MoveMode::Move => {
                self.pdbsystem.move_without_nnp(
                    delta_x as f32,
                    delta_y as f32,
                    (nvec, mvec),
                    &settings.selected_group,
                );
            }
            crate::settings::MoveMode::Off => {
                panic!("mover should not be called at mover");
            }
        }
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        let depth_texture = self.device.create_texture(&wgpu::TextureDescriptor {
            size: wgpu::Extent3d {
                width: self.config.width,
                height: self.config.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            label: Some("Depth Texture"),
            view_formats: &[],
        });
        let depth_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 1.0,
                            g: 1.0,
                            b: 1.0,
                            a: 1.0,
                            // background: white
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &depth_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);
            for i in 0..self.vertex_buffers.len() {
                render_pass.set_vertex_buffer(0, self.vertex_buffers[i].slice(..));
                render_pass
                    .set_index_buffer(self.index_buffers[i].slice(..), wgpu::IndexFormat::Uint16);
                render_pass.draw_indexed(0..self.num_indices[i], 0, 0..1);
            }
            // render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            // render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            // render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
        }

        let screen_descriptor = egui_wgpu::renderer::ScreenDescriptor {
            size_in_pixels: [self.config.width, self.config.height],
            pixels_per_point: self.window().scale_factor() as f32,
        };

        self.egui_renderer.draw(
            &self.device,
            &self.queue,
            &mut encoder,
            &self.window,
            &view,
            screen_descriptor,
            |ui| self.egui_gui.run_gui(ui),
        );

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        Ok(())
    }
}

// run with pollster::block_on()
pub async fn mogura_run() {
    let cli = arg::arg();

    // let (input_pdb, _errors) = pdbtbx::open(
    //     &cli.pdbfile,
    //     pdbtbx::StrictnessLevel::Loose, // Strict, Medium, Loose
    // )
    // .unwrap();

    // let mut pdbsystem = pdb::PDBSystem::from(&input_pdb);
    // pdbsystem.update_bonds_all();
    // dbg!(&pdbsystem);

    env_logger::init();

    let event_loop = winit::event_loop::EventLoop::new().unwrap();
    let title = env!("CARGO_PKG_NAME");
    let window = winit::window::WindowBuilder::new()
        .with_title(title)
        .with_resizable(true)
        .build(&event_loop)
        .unwrap();

    // event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
    // event_loop.set_control_flow(winit::event_loop::ControlFlow::Wait);

    let mut windowstate = State::new(window, cli.pdbfile).await;
    let mut last_render_time = std::time::Instant::now();
    event_loop
        .run(move |event, elwt| match event {
            winit::event::Event::DeviceEvent {
                event: winit::event::DeviceEvent::MouseMotion { delta },
                ..
            } => {
                if windowstate.mouse_pressed {
                    // windowstate.check_is_touched();
                    if windowstate.is_touched {
                        windowstate.mover(delta.0, delta.1);
                    } else {
                        windowstate
                            .camera_controller
                            .process_mouse(delta.0, delta.1);
                    }
                }
                windowstate.window().request_redraw();
            }
            winit::event::Event::AboutToWait => {
                windowstate.window().request_redraw();
            }
            winit::event::Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == windowstate.window().id() => {
                match &event {
                    winit::event::WindowEvent::Resized(physical_size) => {
                        windowstate.resize(*physical_size);
                    }
                    winit::event::WindowEvent::CloseRequested => {
                        windowstate.settings.borrow_mut().show_close_dialog = true;
                        // elwt.exit();
                    }
                    winit::event::WindowEvent::ScaleFactorChanged { .. } => {
                        windowstate.resize(windowstate.window().inner_size());
                    }
                    winit::event::WindowEvent::MouseInput {
                        button: winit::event::MouseButton::Left,
                        state,
                        ..
                    } => {
                        windowstate.mouse_pressed = *state == winit::event::ElementState::Pressed;
                        dbg!(windowstate.mouse_pressed);
                        if windowstate.mouse_pressed {
                            windowstate.check_is_touched();
                        }
                    }
                    winit::event::WindowEvent::CursorMoved {
                        position: winit::dpi::PhysicalPosition { x, y },
                        ..
                    } => {
                        windowstate.mouse_position = [*x as f32, *y as f32];
                        // dbg!(&windowstate.mouse_position);
                        // dbg!(windowstate.window().inner_size());
                    }
                    winit::event::WindowEvent::MouseWheel { delta, .. } => {
                        windowstate.camera_controller.process_scroll(delta);
                    }
                    winit::event::WindowEvent::KeyboardInput { event, .. } => {
                        if windowstate.input(event) {
                            windowstate.window().request_redraw();
                            return;
                        }
                        // if event.state.is_pressed() {
                        //     match event.physical_key {
                        //         winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::Escape) => {
                        //             elwt.exit();
                        //         }
                        //         _ => {}
                        //     }
                        // }
                    }
                    winit::event::WindowEvent::RedrawRequested => {
                        let now = std::time::Instant::now();
                        let dt = now - last_render_time;
                        last_render_time = now;
                        windowstate.update(dt);
                        match windowstate.render() {
                            Ok(_) => {}
                            Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                                windowstate.resize(windowstate.size);
                            }
                            Err(wgpu::SurfaceError::OutOfMemory) => elwt.exit(),
                            Err(wgpu::SurfaceError::Timeout) => log::warn!("Surface timeout"),
                        }
                        windowstate.window().request_redraw();
                    }
                    _ => {}
                }
                windowstate
                    .egui_renderer
                    .handle_input(&windowstate.window, event);
                if windowstate.settings.borrow().allowed_to_close {
                    elwt.exit();
                }
            }
            _ => {} // *control_flow = winit::event_loop::ControlFlow::Poll;
                    //     winit::event::Event::MainEventsCleared => state.window().request_redraw(),
                    //     winit::event::Event::DeviceEvent {
                    //         event: winit::event::DeviceEvent::MouseMotion { delta },
                    //         ..
                    //     } => {
                    //         if state.mouse_pressed {
                    //             state.camera_controller.process_mouse(delta.0, delta.1)
                    //         }
                    //     }
                    //     winit::event::Event::WindowEvent {
                    //         ref event,
                    //         window_id,
                    //     } if window_id == state.window().id() && !state.input(event) => match event {
                    //         winit::event::WindowEvent::CloseRequested
                    //         | winit::event::WindowEvent::KeyboardInput {
                    //             input:
                    //                 winit::event::KeyboardInput {
                    //                     state: winit::event::ElementState::Pressed,
                    //                     virtual_keycode: Some(winit::event::VirtualKeyCode::Escape),
                    //                     ..
                    //                 },
                    //             ..
                    //         } => *control_flow = winit::event_loop::ControlFlow::Exit,
                    //         winit::event::WindowEvent::Resized(physical_size) => {
                    //             state.resize(*physical_size);
                    //         }
                    //         winit::event::WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    //             state.resize(**new_inner_size);
                    //         }
                    //         _ => {}
                    //     },
                    //     winit::event::Event::RedrawRequested(window_id) if window_id == state.window().id() => {
                    //         let now = std::time::Instant::now();
                    //         let dt = now - last_render_time;
                    //         last_render_time = now;
                    //         state.update(dt);
                    //         match state.render() {
                    //             Ok(_) => {}
                    //             Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                    //                 state.resize(state.size)
                    //             }
                    //             Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = winit::event_loop::ControlFlow::Exit,
                    //             Err(wgpu::SurfaceError::Timeout) => log::warn!("Surface timeout"),
                    //         }
                    //     }
                    //     _ => {}
                    // }
        })
        .unwrap();
}
