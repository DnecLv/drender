use std::time::{Duration, Instant};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Timer {
    moment: Instant,
}

impl Default for Timer {
    fn default() -> Self {
        Self::new()
    }
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            moment: Instant::now(),
        }
    }

    pub fn reset(&mut self) {
        self.moment = Instant::now();
    }

    pub fn elapsed(&self) -> Duration {
        self.moment.elapsed()
    }

    pub fn elapsed_in_millis(&self) -> f32 {
        let elapsed = self.elapsed();
        let secs = elapsed.as_secs() as u32;
        let millis = elapsed.subsec_micros();
        (secs * 1_000) as f32 + (millis as f32 / 1000.0)
    }
}

use wgpu::util::DeviceExt;

pub fn create_simple_uniform_buffer(device: &wgpu::Device, label: &str, data: &[u8]) -> wgpu::Buffer
// can be copy
{
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some(label),
        contents: data,
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    })
}