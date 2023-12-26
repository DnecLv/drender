use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub fn run() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    // ? run函数接受一个闭包，该匿名函数接受三个参数，根据event的类型产生结果
    // pub fn run<F>(self, event_handler: F) -> !
    // where
    //     F: 'static + FnMut(Event<'_, T>, &EventLoopWindowTarget<T>, &mut ControlFlow),
    // {
    //     self.event_loop.run(event_handler)
    // }

    event_loop.run(move |event, _, control_flow| match event {
        // 模式匹配到Event::WindowEvent{}，然后event和window_id都被拿出来了，如果window_id相等，继续匹配，如果是CloseRequest or 按下了Esc 退出
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => match event {
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
            _ => {}
        },
        _ => {}
    });
}

fn main() {
    run();
}