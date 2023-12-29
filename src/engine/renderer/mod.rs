use super::state::State;

// mod crate::engine::vertex;
pub mod workflow;
use workflow::RenderWorkFlow;
use super::vertex::{
    Vertex,
    VERTICES,
};

pub struct Renderer{
    process_list: Vec<Box<dyn RenderWorkFlow>>,
    // pub render_pipeline: wgpu::RenderPipeline,
    // pub vertex_buffer: wgpu::Buffer,
    // pub num_vertices: u32,
}
impl Renderer{
    pub fn new() -> Self{
        let process_list = Vec::new();
        Self{
            process_list,
        }
    }
    pub fn add_process<T: RenderWorkFlow + 'static>(&mut self, process: T){
        self.process_list.push(Box::new(process));
    }
    pub fn render(&mut self, state: &State, encoder: &mut wgpu::CommandEncoder,view: &wgpu::TextureView) {
        for process in self.process_list.iter_mut(){
            process.render(state, encoder, view);
        }
    }
}