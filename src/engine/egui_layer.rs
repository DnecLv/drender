use egui::{FontId, RichText};
use egui::{Color32, Context, Ui};

use wgpu::CommandBuffer;
use winit::event_loop;
use raw_window_handle::HasRawDisplayHandle;
use super::state::State;
pub struct EguiLayer {
    egui_ctx: egui::Context,
    egui_state: egui_winit::State,
    egui_renderer: egui_wgpu::Renderer,
    egui_repaint: i32,
}

impl EguiLayer {
    pub fn new(state: &State,event_loop: &dyn HasRawDisplayHandle) -> Self {
        let egui_ctx = egui::Context::default();
        let egui_state = egui_winit::State::new(event_loop);
        let egui_renderer = egui_wgpu::Renderer::new(
            &state.device,
            state.config.format,
            None,
            1
        );
        Self {
            egui_ctx,
            egui_state,
            egui_renderer,
            egui_repaint: 0,
        }
    }
    pub fn render(&mut self, state: &State, encoder: &mut wgpu::CommandEncoder,view: &wgpu::TextureView) -> Vec<CommandBuffer> {
        // ! 这里写输入。。。；到时候要改
        let egui_raw_input = egui::RawInput::default();
        let egui_input = self.egui_state.take_egui_input(&state.window);
        let egui_full_output: egui::FullOutput = self.egui_ctx.run(egui_raw_input, |ctx: &Context| {
            // ! 这里写UI
            // self.ui_contents();
            let mut bg = ctx.style().visuals.window_fill();
            bg = egui::Color32::from_rgba_premultiplied(bg.r(), bg.g(), bg.b(), 230);

            let panel_frame = egui::Frame {
                fill: bg,
                rounding: 10.0.into(),
                stroke: ctx.style().visuals.widgets.noninteractive.fg_stroke,
                outer_margin: 0.5.into(), // so the stroke is within the bounds
                inner_margin: 12.0.into(),
                ..Default::default()
            };

            let window = egui::Window::new("Settings")
                .id(egui::Id::new("particles_window_options")) // required since we change the title
                .resizable(false)
                .collapsible(true)
                .title_bar(true)
                .scroll2([false, true])
                .movable(true)
                .frame(panel_frame)
                .enabled(true);
            window.show(ctx, |ui| {
                ui.separator();
                ui.heading("LBM-Fluid Field Operations");
                ui.horizontal_wrapped(|ui| {
                    ui.label("0. Click the screen to");
                    ui.colored_label(Color32::from_rgb(110, 235, 110), "add obstacles");
                });
            });


            // egui_app.ui_contents(ctx);
            // egui::CentralPanel::default()
            //     .frame(egui::Frame::none().inner_margin(egui::Margin::same(10.0)))
            //     .show(egui_ctx, |ui| {
            //         ui.label(
            //             RichText::new(title)
            //                 .font(FontId::proportional(20.))
            //                 .strong(),
            //         );
            //     });
        });
        let egui_primitives = self.egui_ctx.tessellate(egui_full_output.shapes);
        let screen_descriptor = egui_wgpu::renderer::ScreenDescriptor {
            size_in_pixels: [state.config.width, state.config.height],
            pixels_per_point: 1.0,
        };
        let egui_cmd_bufs = {
            for (id, image_delta) in egui_full_output.textures_delta.set {
                self.egui_renderer.update_texture(&state.device, &state.queue, id, &image_delta)
            }
            self.egui_renderer.update_buffers(
                &state.device,
                &state.queue,
                encoder,
                &egui_primitives,
                &screen_descriptor,
            )
        };
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
                label: None,
            });
            self.egui_renderer.render(&mut render_pass, &egui_primitives, &screen_descriptor);
        }

        for id in egui_full_output.textures_delta.free {
            self.egui_renderer.free_texture(&id);
        }
        egui_cmd_bufs
    }
}
