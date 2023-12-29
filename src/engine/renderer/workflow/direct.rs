use super::RenderWorkFlow;
pub struct DirectWorkFlow{
    pub render_pipeline: wgpu::RenderPipeline,
    pub vertex_buffer: wgpu::Buffer,
    pub num_vertices: u32,
}

impl RenderWorkFlow for DirectWorkFlow{
    fn render(&mut self, state: &State, encoder: &mut wgpu::CommandEncoder,view: &wgpu::TextureView) {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Final Render Pass"),
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
    }
}