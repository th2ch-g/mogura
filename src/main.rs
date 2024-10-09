use mogura_core::*;
use pollster::FutureExt as _;
mod arg;

fn main() {
    let cli = arg::arg();

    #[cfg(target_arch = "wasm32")]
    {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init_with_level(log::Level::Warn).expect("Could't initialize logger");
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        env_logger::init();
    }

    let event_loop = winit::event_loop::EventLoop::new().unwrap();
    let title = env!("CARGO_PKG_NAME");
    let window = winit::window::WindowBuilder::new()
        .with_title(title)
        .with_resizable(true)
        .build(&event_loop)
        .unwrap();

    #[cfg(target_arch = "wasm32")]
    {
        use winit::platform::web::WindowExtWebSys;
        let web_window = web_sys::window().unwrap();
        let document = web_window.document().unwrap();
        let body = document.body().unwrap();
        let canvas = web_sys::Element::from(window.canvas().unwrap());
        body.append_child(&canvas).expect("Could not append");
    }

    let mut models: Vec<Box<dyn rasterizer::model::Model>> = vec![
        // Box::new(rasterizer::model::shape::triangle::Triangle::default()),
        // Box::new(rasterizer::model::shape::sphere::Sphere::default()),
        // Box::new(rasterizer::model::shape::cylinder::Cylinder::default()),
    ];

    if let Some(structure_file) = &cli.structure_file {
        models.push(Box::new(rasterizer::model::mol_drawer::vdw::VDW::new(structure_file)))
    }

    let mut pipeline = Pipeline::new(PipelineDescirptor { window, models }).block_on();

    pipeline.run(event_loop);
}
