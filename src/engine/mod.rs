use raw_window_handle::HasRawDisplayHandle;
use std::iter;
use winit::{
    event::*,
    event_loop::{self, ControlFlow, EventLoop, EventLoopBuilder},
    window,
    window::{WindowBuilder, WindowId},
};

mod state;
use state::State;
mod renderer;
use renderer::Renderer;
mod plugin;
use plugin::Plugin;
mod egui_layer;
use egui_layer::EguiLayer;
pub struct Engine {
    state: State,
    // scene: Scene,
    renderer: Renderer,
    egui_layer: EguiLayer,
    plugins: Vec<Box<dyn Plugin>>,
}

#[allow(unused)]
#[derive(Debug)]
pub struct CustomJsTriggerEvent {
    ty: &'static str,
    value: String,
}

impl Engine {
    pub fn new() -> (Engine, EventLoop<CustomJsTriggerEvent>) {
        env_logger::init();
        // let event_loop = EventLoop::new();
        let event_loop = EventLoopBuilder::<CustomJsTriggerEvent>::with_user_event().build();
        // let window = window::WindowBuilder::new().build(&event_loop).unwrap();
        let window = WindowBuilder::new()
            .with_inner_size(winit::dpi::PhysicalSize::new(1024, 768))
            .with_title("D-Render")
            .build(&event_loop)
            .unwrap();

        let state = pollster::block_on(State::new(window));
        (
            Self {
                renderer: Renderer::new(&state),
                egui_layer: EguiLayer::new(&state, &event_loop),
                state: state,
                plugins: vec![],
            },
            event_loop,
        )
    }
    #[allow(dead_code)]
    pub fn add_plugin(&mut self) {}

    pub fn current_window_id(&self) -> WindowId {
        self.state.window.id()
    }
    fn ui_event(&mut self, event: &winit::event::WindowEvent<'_>) {
        self.egui_layer.ui_event(event);
    }
    pub fn run(mut self, event_loop: EventLoop<CustomJsTriggerEvent>) {
        event_loop.run(move |event, _, control_flow| {
            let window = &self.state.window;

            match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == self.current_window_id() =>
                // if !state.input(event)
                {
                    self.ui_event(event);
                    match event {
                        WindowEvent::CloseRequested
                        | WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    state: ElementState::Pressed,
                                    virtual_keycode: Some(VirtualKeyCode::Escape),
                                    ..
                                },
                            ..
                        } => *control_flow = ControlFlow::Exit,
                        WindowEvent::Resized(physical_size) => {
                            self.state.resize(*physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            self.state.resize(**new_inner_size);
                        }
                        _ => {}
                    }
                }
                Event::RedrawRequested(window_id) if window_id == self.current_window_id() => {
                    // self.render();
                    // state.update();
                    match self.render() {
                        Ok(_) => {}
                        // 当展示平面的上下文丢失，就需重新配置
                        Err(wgpu::SurfaceError::Lost) => self.state.resize(self.state.size),
                        // 系统内存不足时，程序应该退出。
                        Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                        // 所有其他错误（过期、超时等）应在下一帧解决
                        Err(e) => eprintln!("{:?}", e),
                    }
                }
                Event::MainEventsCleared => {
                    // 除非我们手动请求，RedrawRequested 将只会触发一次。
                    window.request_redraw();
                }
                _ => {}
            }
        });
    }
    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        // self.renderer.render(&self.state)
        let output = self.state.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder =
            self.state
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });
        self.renderer.render(&self.state, &mut encoder,&view);
        let ebuffer = self.egui_layer.render(&self.state, &mut encoder,&view);
        // drop(render_pass);
        if let Some(egui_cmd_bufs) = ebuffer {
            self.state.queue.submit(
                egui_cmd_bufs
                    .into_iter()
                    .chain(iter::once(encoder.finish())),
            );
        } else {
            self.state.queue.submit(iter::once(encoder.finish()));
        }
        output.present();
        Ok(())
    }
}
