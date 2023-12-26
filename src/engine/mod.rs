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

pub struct Engine {
    state: State,
    // scene: Scene,
    renderer: Renderer,
    // egui_layer: EguiLayer,
    plugins: Vec<Box<dyn Plugin>>,
}
impl Engine {
    pub fn new() -> (Engine, EventLoop<()>) {
        env_logger::init();
        let event_loop = EventLoop::new();
        // let window = window::WindowBuilder::new().build(&event_loop).unwrap();
        let window = WindowBuilder::new()
            .with_inner_size(winit::dpi::PhysicalSize::new(512, 512))
            .with_title("D-Render")
            .build(&event_loop)
            .unwrap();

        let state = pollster::block_on(State::new(window));
        (
            Self {
                renderer: Renderer::new(&state),
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
    pub fn run(mut self, event_loop: EventLoop<()>) {
        event_loop.run(move |event, _, control_flow| {
            let window = &self.state.window;

            match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == self.current_window_id() =>
                // if !state.input(event)
                {
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
    fn render(&self) -> Result<(), wgpu::SurfaceError>{
        self.renderer.render(&self.state)
        // let frame = self.state.surface.get_current_frame().unwrap();
        // let view = frame
        //     .output
        //     .texture
        //     .create_view(&wgpu::TextureViewDescriptor::default());
        // let mut encoder = self
        //     .state
        //     .device
        //     .create_command_encoder(&wgpu::CommandEncoderDescriptor {
        //         label: Some("Render Encoder"),
        //     });
        // {
        //     let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        //         label: Some("Render Pass"),
        //         color_attachments: &[wgpu::RenderPassColorAttachment {
        //             view: &view,
        //             resolve_target: None,
        //             ops: wgpu::Operations {
        //                 load: wgpu::LoadOp::Clear(wgpu::Color {
        //                     r: 0.1,
        //                     g: 0.2,
        //                     b: 0.3,
        //                     a: 1.0,
        //                 }),
        //                 store: true,
        //             },
        //         }],
        //         depth_stencil_attachment: None,
        //     });
        // }
        // self.state.queue.submit(Some(encoder.finish()));
    }
}
