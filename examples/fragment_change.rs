#[macro_use] extern crate log;
#[macro_use] extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate env_logger;
#[macro_use] extern crate gfx_shader_watch;

use std::time::*;
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
        pos: [f32; 2] = "a_Pos",
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

fn overwrite_fragment_shader(new_contents: &str) -> Result<(), Box<Error>> {
    let path = Path::new(file!()).canonicalize()?.parent().ok_or("no parent")?.join("shader/frag.glsl");
    let mut shader = OpenOptions::new()
        .write(true)
        .open(path)?;
    shader.set_len(0)?;
    shader.write_all(new_contents.as_bytes())?;
    Ok(())
}

pub fn main() {
    env_logger::init().unwrap();

    let events_loop = glutin::EventsLoop::new();
    let builder = glutin::WindowBuilder::new()
        .with_title("Triangle".to_string())
        .with_dimensions(1024, 768)
        .with_vsync();
    let (window, mut device, mut factory, main_color, _main_depth) =
        gfx_window_glutin::init::<Rgba8, Depth>(builder, &events_loop);

    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

    let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&TRIANGLE, ());
    let data = trianglepipe::Data {
        vbuf: vertex_buffer,
        out: main_color
    };

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

    warn!("Modifying shader to color red...");
    overwrite_fragment_shader(&FRAGMENT_SHADER
        .replace("gl_FragColor = white;", "gl_FragColor = red;")).unwrap();

    while Instant::now().duration_since(start) < show_duration * 2 {
        encoder.clear(&data.out, CLEAR_COLOR);
        encoder.draw(&slice, pso_cell.pso(), &data);
        encoder.flush(&mut device);
        window.swap_buffers().unwrap();
        device.cleanup();
    }

    warn!("Modifying shader to color green...");
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