#[macro_use]
extern crate gfx;
extern crate gfx_shader_watch;
extern crate env_logger;
extern crate gfx_window_glutin;
extern crate glutin;

use gfx::format::{Depth, Rgba8};
use gfx::state::Rasterizer;
use gfx::traits::FactoryExt;
use gfx::Device;
use gfx::Primitive;
use gfx_shader_watch::*;
use glutin::GlContext;
use std::env;
use std::error::Error;

gfx_defines!{
    vertex Vertex {
        pos: [f32; 2] = "pos",
    }

    pipeline trianglepipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        out: gfx::RenderTarget<Rgba8> = "Target0",
    }
}

const TRIANGLE: [Vertex; 3] = [
    Vertex { pos: [-0.5, -0.5] },
    Vertex { pos: [0.5, -0.5] },
    Vertex { pos: [0.0, 0.5] },
];

const CLEAR_COLOR: [f32; 4] = [0.1, 0.2, 0.3, 1.0];

pub fn main() -> Result<(), Box<Error>> {
    env_logger::init();

    // winit select x11 by default
    if cfg!(target_os = "linux") && env::var("WINIT_UNIX_BACKEND").is_err() {
        env::set_var("WINIT_UNIX_BACKEND", "x11");
    }

    let mut events_loop = glutin::EventsLoop::new();
    let window_builder = glutin::WindowBuilder::new()
        .with_title("Triangle".to_string())
        .with_dimensions((1024, 768).into());
    let context = glutin::ContextBuilder::new().with_vsync(true);
    let (window, mut device, mut factory, main_color, _main_depth) =
        gfx_window_glutin::init::<Rgba8, Depth>(window_builder, context, &events_loop);

    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

    let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&TRIANGLE, ());
    let data = trianglepipe::Data {
        vbuf: vertex_buffer,
        out: main_color,
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

    let mut running = true;
    while running {
        events_loop.poll_events(|event| {
            if let glutin::Event::WindowEvent { event, .. } = event {
                match event {
                    glutin::WindowEvent::KeyboardInput {
                        input:
                            glutin::KeyboardInput {
                                virtual_keycode: Some(glutin::VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    }
                    | glutin::WindowEvent::CloseRequested => running = false,
                    _ => {}
                }
            }
        });

        encoder.clear(&data.out, CLEAR_COLOR);
        encoder.draw(&slice, pso_cell.pso(), &data);
        encoder.flush(&mut device);
        window.swap_buffers()?;
        device.cleanup();
    }

    Ok(())
}
