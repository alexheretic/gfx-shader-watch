use gfx::{
    self,
    format::{Depth, Srgba8},
    state::Rasterizer,
    traits::FactoryExt,
    Device, Primitive, *,
};
use gfx_shader_watch::*;
use glutin::surface::GlSurface;
use std::{error::Error, num::NonZeroU32};
use winit::{
    event::{Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
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
    env_logger::init();

    let event_loop = EventLoop::new();
    let window_builder = WindowBuilder::new()
        .with_title("Triangle".to_string())
        .with_inner_size(winit::dpi::PhysicalSize::new(1024, 768));

    let old_school_gfx_glutin_ext::Init {
        window,
        gl_surface,
        gl_context,
        mut device,
        mut factory,
        color_view,
        mut depth_view,
        ..
    } = old_school_gfx_glutin_ext::window_builder(&event_loop, window_builder)
        .build::<Srgba8, Depth>()?;

    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();
    let mut view_size = window.inner_size();

    let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&TRIANGLE, ());
    let mut data = trianglepipe::Data {
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
    let mut pso_cell = debug_watcher_pso_cell!(
        pipe = trianglepipe,
        vertex_shader = "shader/vert.glsl",
        fragment_shader = "shader/frag.glsl",
        factory = factory,
        primitive = Primitive::TriangleList,
        rasterizer = Rasterizer::new_fill()
    )?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                } => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            Event::MainEventsCleared => {
                // handle resizes
                let w_size = window.inner_size();
                if view_size != w_size {
                    if let (Some(w), Some(h)) = (
                        NonZeroU32::new(w_size.width),
                        NonZeroU32::new(w_size.height),
                    ) {
                        gl_surface.resize(&gl_context, w, h);
                        old_school_gfx_glutin_ext::resize_views(
                            w_size,
                            &mut data.out,
                            &mut depth_view,
                        );
                    }
                    view_size = w_size;
                }
                encoder.clear(&data.out, CLEAR_COLOR);
                encoder.draw(&slice, pso_cell.pso(), &data);
                encoder.flush(&mut device);
                gl_surface.swap_buffers(&gl_context).unwrap();
                device.cleanup();
            }
            _ => (),
        }
    });
}
