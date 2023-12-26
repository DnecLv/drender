use super::state::State;
use wgpu::util::DeviceExt;
use std::iter;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}
impl Vertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}
const VERTICES: &[Vertex] = &[
    Vertex {
        position: [0.0, 0.5, 0.0],
        color: [1.0, 0.0, 0.0],
    },
    Vertex {
        position: [-0.5, -0.5, 0.0],
        color: [0.0, 1.0, 0.0],
    },
    Vertex {
        position: [0.5, -0.5, 0.0],
        color: [0.0, 0.0, 1.0],
    },
];
pub struct Renderer{
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    num_vertices: u32,
}
impl Renderer{
    pub fn new(app:&State) -> Self{
        let shader = app
        .device
        .create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../shader/shader.wgsl").into()),
        });
        let render_pipeline_layout =
            app.device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Render Pipeline Layout"),
                    bind_group_layouts: &[],
                    push_constant_ranges: &[],
                });
        let render_pipeline = app
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
                        format: app.config.format,
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
        let vertex_buffer = app
            .device 
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(VERTICES),
                usage: wgpu::BufferUsages::VERTEX,
            });
        let num_vertices = VERTICES.len() as u32;
        Self{
            render_pipeline,
            vertex_buffer,
            num_vertices,
        }
    }
    pub fn render(&self,app:&State) -> Result<(), wgpu::SurfaceError> {
        let output = app.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = app
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[
                // 这就是片元着色器中 @location(0) 标记指向的颜色附件
                Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        // store: wgpu::StoreOp::Store,
                        store: true,
                    },
                }),
            ],
            depth_stencil_attachment: None,
        });

        // 新添加!
        render_pass.set_pipeline(&self.render_pipeline); // 2. 
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        // render_pass.draw(0..3, 0..1);
        render_pass.draw(0..self.num_vertices, 0..1);

        drop(render_pass);
        // submit 命令能接受任何实现了 IntoIter trait 的参数
        // if let Some(egui_cmd_bufs) = egui_cmd_bufs {
        //     self.app.queue.submit(
        //         egui_cmd_bufs
        //             .into_iter()
        //             .chain(iter::once(encoder.finish())),
        //     );
        // } else {
        app.queue.submit(iter::once(encoder.finish()));
        // }
        // self.app.queue.submit(
        //     egui_cmd_bufs
        //         .into_iter()
        //         .chain(std::iter::once(encoder.finish()))
        // );
        // self.app.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        Ok(())
    }
}