use gfx::{
    self,
    format::{Depth, Srgba8},
    state::Rasterizer,
    traits::FactoryExt,
    Device, Primitive, *,
};
use gfx_shader_watch::*;
use glutin::{
    context::PossiblyCurrentContext,
    surface::{GlSurface, Surface, WindowSurface},
};
use std::{env, error::Error, num::NonZeroU32};
use winit::{
    dpi::PhysicalSize,
    event::{KeyEvent, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::{Key, NamedKey},
    window::Window,
};

gfx_defines! {
    vertex Vertex {
        pos: [f32; 2] = "pos",
    }

    pipeline trianglepipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        out: gfx::RenderTarget<Srgba8> = "Target0",
    }
}
impl Eq for trianglepipe::Meta {}

const TRIANGLE: [Vertex; 3] = [
    Vertex { pos: [-0.5, -0.5] },
    Vertex { pos: [0.5, -0.5] },
    Vertex { pos: [0.0, 0.5] },
];

const CLEAR_COLOR: [f32; 4] = [0.1, 0.2, 0.3, 1.0];

pub fn main() -> Result<(), Box<dyn Error>> {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "gfx_shader_watch=debug");
    }
    env_logger::init();

    Ok(EventLoop::new()?.run_app(&mut WinitApp::None)?)
}

enum WinitApp {
    None,
    Resumed(App),
}

impl winit::application::ApplicationHandler for WinitApp {
    fn resumed(&mut self, events: &ActiveEventLoop) {
        events.set_control_flow(ControlFlow::Poll);
        *self = Self::Resumed(App::new(events).unwrap());
    }

    fn window_event(
        &mut self,
        events: &ActiveEventLoop,
        _: winit::window::WindowId,
        event: WindowEvent,
    ) {
        if let Self::Resumed(app) = self {
            app.window_event(events, event);
        }
    }

    fn about_to_wait(&mut self, _events: &ActiveEventLoop) {
        if let Self::Resumed(App { window, .. }) = self {
            window.request_redraw();
        };
    }
}

struct App {
    window: Window,
    gl_surface: Surface<WindowSurface>,
    gl_context: PossiblyCurrentContext,
    device: gfx_device_gl::Device,
    encoder: gfx::Encoder<gfx_device_gl::Resources, gfx_device_gl::CommandBuffer>,
    depth_view: gfx::handle::DepthStencilView<gfx_device_gl::Resources, Depth>,
    window_size: PhysicalSize<u32>,
    data: trianglepipe::Data<gfx_device_gl::Resources>,
    slice: gfx::Slice<gfx_device_gl::Resources>,
    pso_cell: debug_watcher_pso_cell_type!(
        gfx_device_gl::Resources,
        gfx_device_gl::Factory,
        pipe = trianglepipe
    ),
}

impl App {
    fn new(events: &ActiveEventLoop) -> Result<Self, Box<dyn Error>> {
        let window_attrs = Window::default_attributes()
            .with_title("Try changing shader/frag.glsl".to_string())
            .with_inner_size(winit::dpi::PhysicalSize::new(1024, 768));

        let old_school_gfx_glutin_ext::Init {
            window,
            gl_surface,
            gl_context,
            device,
            mut factory,
            color_view,
            depth_view,
            ..
        } = old_school_gfx_glutin_ext::window_builder(events, window_attrs)
            .build::<Srgba8, Depth>()?;

        let encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();
        let window_size = window.inner_size();

        let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&TRIANGLE, ());
        let data = trianglepipe::Data {
            vbuf: vertex_buffer,
            out: color_view,
        };

        // This object holds the pipeline state object and it's own factory
        // accessed by pso_cell.pso() & pso_cell.factory()
        // When compiled in debug mode it will watch the files "shader/vert.glsl" & "shader/frag.glsl"
        // relative to this source file, if changed will try to re-create the pso on the next call to
        // pso_cell.pso(). Thus the shaders a reloaded on the fly.
        //
        // When compiled in release mode the shaders are added using include_bytes! and no watcher is
        // setup. The release PsoCell is simply a container for the one-time created pso & factory.
        let pso_cell = debug_watcher_pso_cell!(
            pipe = trianglepipe,
            vertex_shader = "shader/vert.glsl",
            fragment_shader = "shader/frag.glsl",
            factory = factory,
            primitive = Primitive::TriangleList,
            rasterizer = Rasterizer::new_fill()
        )?;

        Ok(Self {
            window,
            gl_surface,
            gl_context,
            device,
            encoder,
            depth_view,
            window_size,
            data,
            slice,
            pso_cell,
        })
    }

    fn window_event(&mut self, events: &ActiveEventLoop, event: WindowEvent) {
        let Self {
            window,
            gl_surface,
            gl_context,
            device,
            encoder,
            depth_view,
            window_size,
            data,
            slice,
            pso_cell,
        } = self;

        match event {
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        logical_key: Key::Named(NamedKey::Escape),
                        ..
                    },
                ..
            } => events.exit(),
            WindowEvent::RedrawRequested => {
                // handle resizes
                let w_size = window.inner_size();
                if *window_size != w_size {
                    if let (Some(w), Some(h)) = (
                        NonZeroU32::new(w_size.width),
                        NonZeroU32::new(w_size.height),
                    ) {
                        gl_surface.resize(gl_context, w, h);
                        old_school_gfx_glutin_ext::resize_views(w_size, &mut data.out, depth_view);
                    }
                    *window_size = w_size;
                }
                encoder.clear(&data.out, CLEAR_COLOR);
                encoder.draw(slice, pso_cell.pso(), data);
                encoder.flush(device);
                gl_surface.swap_buffers(gl_context).unwrap();
                device.cleanup();
            }
            _ => (),
        }
    }
}
