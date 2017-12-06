#[macro_use] extern crate log;
#[macro_use] extern crate gfx;
#[macro_use] extern crate gfx_shader_watch;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate env_logger;

use glutin::GlContext;
use std::time::*;
use std::env;
use std::path::Path;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::Write;
use gfx::traits::FactoryExt;
use gfx::Device;
use gfx::format::{Rgba8, Depth};
use gfx_shader_watch::*;

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
    Vertex { pos: [ -0.5, -0.5 ] },
    Vertex { pos: [  0.5, -0.5 ] },
    Vertex { pos: [  0.0,  0.5 ] }
];

const CLEAR_COLOR: [f32; 4] = [0.1, 0.2, 0.3, 1.0];

static FRAGMENT_SHADER: &str = include_str!("shader/frag.glsl");

/// Emulates a manual change of a shader contents, ie the dev changing the shader code
fn overwrite_fragment_shader(new_contents: &str) -> Result<(), Box<Error>> {
    let path = Path::new(file!()).canonicalize()?
        .parent().ok_or("no parent")?
        .join("shader").join("frag.glsl");
    let mut shader = OpenOptions::new()
        .write(true)
        .open(path)?;
    shader.set_len(0)?;
    shader.write_all(new_contents.as_bytes())?;
    Ok(())
}

pub fn main() {
    env_logger::init().unwrap();
    if cfg!(target_os = "linux") {
        // winit wayland is currently still wip
        if env::var("WINIT_UNIX_BACKEND").is_err() {
            env::set_var("WINIT_UNIX_BACKEND", "x11");
        }
    }

    let events_loop = glutin::EventsLoop::new();
    let window_builder = glutin::WindowBuilder::new()
        .with_title("Triangle".to_string())
        .with_dimensions(1024, 768);
    let context = glutin::ContextBuilder::new()
        .with_vsync(true);
    let (window, mut device, mut factory, main_color, _main_depth) =
        gfx_window_glutin::init::<Rgba8, Depth>(window_builder, context, &events_loop);

    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

    let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&TRIANGLE, ());
    let data = trianglepipe::Data {
        vbuf: vertex_buffer,
        out: main_color
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
        factory = factory).expect("psocell");

    let start = Instant::now();
    let show_duration = Duration::from_millis(333);
    while Instant::now().duration_since(start) < show_duration {
        encoder.clear(&data.out, CLEAR_COLOR);
        encoder.draw(&slice, pso_cell.pso(), &data);
        encoder.flush(&mut device);
        window.swap_buffers().unwrap();
        device.cleanup();
    }

    info!("Simulate developer modifying shader to color red...");
    overwrite_fragment_shader(&FRAGMENT_SHADER
        .replace("gl_FragColor = white;", "gl_FragColor = red;")).unwrap();

    while Instant::now().duration_since(start) < show_duration * 2 {
        encoder.clear(&data.out, CLEAR_COLOR);
        encoder.draw(&slice, pso_cell.pso(), &data);
        encoder.flush(&mut device);
        window.swap_buffers().unwrap();
        device.cleanup();
    }

    info!("Simulate developer modifying shader to color green...");
    overwrite_fragment_shader(&FRAGMENT_SHADER
        .replace("gl_FragColor = white;", "gl_FragColor = green;")).unwrap();

    while Instant::now().duration_since(start) < show_duration * 3 {
        encoder.clear(&data.out, CLEAR_COLOR);
        encoder.draw(&slice, pso_cell.pso(), &data);
        encoder.flush(&mut device);
        window.swap_buffers().unwrap();
        device.cleanup();
    }

    overwrite_fragment_shader(FRAGMENT_SHADER).unwrap();
}
