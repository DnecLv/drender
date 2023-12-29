use egui::{Color32, Context, Ui};
use egui::{FontId, RichText};

use crate::utils::Timer;

use super::state::State;
use raw_window_handle::HasRawDisplayHandle;
use wgpu::CommandBuffer;
use winit::event_loop;
pub struct EguiLayer {
    egui_ctx: egui::Context,
    egui_state: egui_winit::State,
    egui_renderer: egui_wgpu::Renderer,
    egui_repaint: i32,
    controller: Controller,
}

impl EguiLayer {
    pub fn new(state: &State, event_loop: &dyn HasRawDisplayHandle) -> Self {
        let egui_ctx = egui::Context::default();
        let egui_state = egui_winit::State::new(event_loop);
        let egui_renderer = egui_wgpu::Renderer::new(&state.device, state.config.format, None, 1);
        Self {
            egui_ctx,
            egui_state,
            egui_renderer,
            egui_repaint: 2,
            controller: Controller::new(),
        }
    }
    pub fn render(
        &mut self,
        state: &State,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
    ) -> Option<Vec<wgpu::CommandBuffer>> {
        // if self.egui_repaint <= 0 {
        //     return None;
        // }
        // self.egui_repaint -= 1;
        self.controller.update();

        let egui_input = self.egui_state.take_egui_input(&state.window);
        let egui_full_output = self.egui_ctx.run(egui_input, |ctx: &Context| {
            self.controller.ui_contents(ctx);
        });
        let egui_primitives = self.egui_ctx.tessellate(egui_full_output.shapes);
        let screen_descriptor = egui_wgpu::renderer::ScreenDescriptor {
            size_in_pixels: [state.config.width, state.config.height],
            pixels_per_point: 1.0,
        };
        let egui_cmd_bufs = {
            for (id, image_delta) in egui_full_output.textures_delta.set {
                self.egui_renderer
                    .update_texture(&state.device, &state.queue, id, &image_delta)
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
            self.egui_renderer
                .render(&mut render_pass, &egui_primitives, &screen_descriptor);
        }

        for id in egui_full_output.textures_delta.free {
            self.egui_renderer.free_texture(&id);
        }
        Some(egui_cmd_bufs)
    }
    pub fn ui_event(&mut self, event: &winit::event::WindowEvent<'_>) {
        self.egui_state.on_event(&self.egui_ctx, event);
        // let response = self.egui_state.on_event(&self.egui_ctx, event);
        // self.egui_repaint = if response.consumed {
        // 20
        // } else {
        // self.egui_repaint.max(1)
        // };
        // println!("egui_repaint: {}",self.egui_repaint);
    }
}

struct Controller {
    count: i32,
    fps:f32,
    timer:Timer,
}
impl Controller {
    fn new() -> Self {
        Self {
            count: 0,
            fps:0.0,
            timer:Timer::new()
        }
    }
    fn update(&mut self) {
        if self.count < 100 {
            self.count += 1;
        }
        else{
            self.count = 0;
            self.fps = 100000.0 / self.timer.elapsed_in_millis();
            self.timer.reset();
        }
    }
    fn ui_contents(&mut self, ctx: &egui::Context) {
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
            // ui.horizontal_wrapped(|ui| {
            //     if ui.button(RichText::new("add's button")).clicked() {
            //         self.count += 1;
            //     };
            //     ui.label(format!("count: {}", self.count));
            //     ui.label(format!("fps: {:.1}", self.fps));
            //     // ui.colored_label(Color32::from_rgb(110, 235, 110), "add obstacles");
            // });
            ui.label(format!("fps: {:.1}", self.fps));
        });
    }
}
