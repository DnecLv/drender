use super::RenderWorkFlow;
use crate::engine::state::State;
use crate::engine::vertex::{Vertex, VERTICES};
use wgpu::util::DeviceExt;

pub struct DirectWorkFlow {
    pub render_pipeline: wgpu::RenderPipeline,
    pub vertex_buffer: wgpu::Buffer,
    pub num_vertices: u32,
}
impl DirectWorkFlow {
    pub fn new(state: &State) -> Self {
        let shader = state
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Shader"),
                source: wgpu::ShaderSource::Wgsl(
                    include_str!("../../../shader/shader.wgsl").into(),
                ),
            });
        let render_pipeline_layout =
            state
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Render Pipeline Layout"),
                    bind_group_layouts: &[],
                    push_constant_ranges: &[],
                });
        let render_pipeline =
            state
                .device
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("Render Pipeline"),
                    layout: Some(&render_pipeline_layout),
                    vertex: wgpu::VertexState {
                        module: &shader,
                        entry_point: "vs_main",     // 1.
                        buffers: &[Vertex::desc()], // 2.
                    },
                    fragment: Some(wgpu::FragmentState {
                        // 3.
                        module: &shader,
                        entry_point: "fs_main",
                        targets: &[Some(wgpu::ColorTargetState {
                            // 4.
                            format: state.config.format,
                            blend: Some(wgpu::BlendState::REPLACE),
                            write_mask: wgpu::ColorWrites::ALL,
                        })],
                    }),
                    primitive: wgpu::PrimitiveState {
                        topology: wgpu::PrimitiveTopology::TriangleList, // 1.
                        strip_index_format: None,
                        front_face: wgpu::FrontFace::Ccw, // 2.
                        cull_mode: Some(wgpu::Face::Back),
                        // 将此设置为 Fill 以外的任何值都要需要开启 Feature::NON_FILL_POLYGON_MODE
                        polygon_mode: wgpu::PolygonMode::Fill,
                        // 需要开启 Features::DEPTH_CLIP_CONTROL
                        unclipped_depth: false,
                        // 需要开启 Features::CONSERVATIVE_RASTERIZATION
                        conservative: false,
                    },
                    depth_stencil: None, // 1.
                    multisample: wgpu::MultisampleState {
                        count: 1,                         // 2.
                        mask: !0,                         // 3.
                        alpha_to_coverage_enabled: false, // 4.
                    },
                    multiview: None, // 5.
                });
        let vertex_buffer = state
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(VERTICES),
                usage: wgpu::BufferUsages::VERTEX,
            });
        let num_vertices = VERTICES.len() as u32;
        Self {
            render_pipeline,
            vertex_buffer,
            num_vertices,
        }
    }
}

impl RenderWorkFlow for DirectWorkFlow {
    fn render(&self) {
        todo!()
    }
}
