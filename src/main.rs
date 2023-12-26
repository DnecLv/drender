// use drenderer::run;
mod engine;
mod utils;
use engine::Engine;
fn main() {
    
    // pollster::block_on(run());
    let (engine,event_loop) = Engine::new();
    engine.run(event_loop);
}