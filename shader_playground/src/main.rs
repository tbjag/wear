use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::{Window,WindowBuilder};

struct State<'a> {
    window: &'a Window,
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
}

impl<'a> State<'a> {
    async fn new(window: &'a Window) -> State<'a> {
        let size = window.inner_size();
        let instance = wgpu::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });
    let surface = instance.create_surface(window).unwrap();
    let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        compatible_surface: Some(&surface),
        force_fallback_adapter: false,
    }).await.unwrap();

    let (device, queue) = adapter.request_device(
        &wgpu::DeviceDescriptor{
            label: Some("Main Device"),
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::default(),
            memory_hints: Default::default()
        },
        None,
    ).await.unwrap();



fn main() {
    env_logger::init();

    let event_loop = EventLoop::new().unwrap();

    let window = WindowBuilder::new()
        .with_title("Shader Playground")
        .build(&event_loop)
        .unwrap();
    
    event_loop
        .run(move | event, control_flow | {
            match event {
                Event::WindowEvent { ref event, window_id }
                    if window_id == window.id() => {
                        match event {
                            WindowEvent::CloseRequested => control_flow.exit(),
                            WindowEvent::RedrawRequested => {
                                window.pre_present_notify();
                            }
                            _ => {}
                        }
                    }
                    Event::AboutToWait => {
                        window.request_redraw();
                    }
                    _ => {}
            }
        }).unwrap();
}

