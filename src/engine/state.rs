// use std::sync::Arc;
use std::rc::Rc;

pub struct State {
    pub window: winit::window::Window,
    pub surface: wgpu::Surface,
    pub adapter: wgpu::Adapter,
    pub device: Rc<wgpu::Device>,
    pub queue: Rc<wgpu::Queue>,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
}
use wgpu::util::DeviceExt;
use winit::window::Window;

impl State {
    pub async fn new(window: Window) -> Self {
        let size = window.inner_size();
        // instance 变量是 GPU 实例
        // Backends::all 对应 Vulkan、Metal、DX12、WebGL 等所有后端图形驱动
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });
        // instance => surface 展示平面
        let surface = unsafe { instance.create_surface(&window).unwrap() };
        // instance => adapter 适配器 适配器是固定在特定图形后端的。假如你使用的是 Windows 且有 2 个显卡（集成显卡 + 独立显卡），则至少有 4 个适配器可供使用，分别有 2 个固定在 Vulkan 和 DirectX 后端。
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                compatible_surface: Some(&surface),
                ..Default::default()
            })
            .await
            .unwrap();
        // adapter => device 设备
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    // WebGL 后端并不支持 wgpu 的所有功能，
                    // 所以如果要以 web 为构建目标，就必须禁用一些功能。
                    limits: if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                    label: None,
                },
                None, // 追踪 API 调用路径
            )
            .await
            .unwrap();
        if cfg!(debug_assertions) {
            let adapter_info = adapter.get_info();
            let gpu_info = format!(
                "正在使用 {}, 后端图形接口为 {:?}。",
                adapter_info.name, adapter_info.backend
            );
            println!("{gpu_info}");
        }
        let caps = surface.get_capabilities(&adapter);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: caps.formats[0],
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: caps.alpha_modes[0],
            view_formats: vec![],
        };

        surface.configure(&device, &config);

        State {
            window: window,
            surface,
            adapter,
            device: Rc::new(device),
            queue: Rc::new(queue),
            config,
            size,
        }
    }
    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface
                .configure(&self.device, &self.config);
        }
    }
}
