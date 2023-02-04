use gfx::{
    self,
    format::{Depth, Srgba8},
    traits::FactoryExt,
    Device, *,
};
use gfx_shader_watch::*;
use glutin::surface::GlSurface;
use log::info;
use std::{env, error::Error, fs::OpenOptions, io::Write, path::Path, time::*};
use winit::{event_loop::EventLoop, window::WindowBuilder};

gfx_defines! { vertex V { p: f32 = "p", } pipeline p { vbuf: gfx::VertexBuffer<V> = (), } }
impl Eq for p::Meta {}

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

static FRAGMENT_SHADER: &str = include_str!("shader/frag.glsl");

/// Emulates a manual change of a shader contents, ie the dev changing the shader code
fn overwrite_fragment_shader(new_contents: &str) -> Result<(), Box<dyn Error>> {
    let path = Path::new(file!())
        .canonicalize()?
        .parent()
        .ok_or("no parent")?
        .join("shader")
        .join("frag.glsl");
    let mut shader = OpenOptions::new().write(true).open(path)?;
    shader.set_len(0)?;
    shader.write_all(new_contents.as_bytes())?;
    Ok(())
}

pub fn main() -> Result<(), Box<dyn Error>> {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "gfx_shader_watch=debug");
    }
    env_logger::init();

    let event_loop = EventLoop::new();
    let window_builder = WindowBuilder::new()
        .with_title("Triangle".to_string())
        .with_inner_size(winit::dpi::PhysicalSize::new(1024, 768));

    let old_school_gfx_glutin_ext::Init {
        gl_surface,
        gl_context,
        mut device,
        mut factory,
        color_view,
        ..
    } = old_school_gfx_glutin_ext::window_builder(&event_loop, window_builder)
        .build::<Srgba8, Depth>()?;

    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

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
    let mut pso_cell = debug_watcher_pso_cell!(
        pipe = trianglepipe,
        vertex_shader = "shader/vert.glsl",
        fragment_shader = "shader/frag.glsl",
        factory = factory
    )?;

    let start = Instant::now();
    let show_duration = Duration::from_millis(333);
    while Instant::now().duration_since(start) < show_duration {
        encoder.clear(&data.out, CLEAR_COLOR);
        encoder.draw(&slice, pso_cell.pso(), &data);
        encoder.flush(&mut device);
        gl_surface.swap_buffers(&gl_context)?;
        device.cleanup();
    }

    info!("Simulate developer modifying shader to color red...");
    overwrite_fragment_shader(
        &FRAGMENT_SHADER.replace("gl_FragColor = white;", "gl_FragColor = red;"),
    )?;

    while Instant::now().duration_since(start) < show_duration * 2 {
        encoder.clear(&data.out, CLEAR_COLOR);
        encoder.draw(&slice, pso_cell.pso(), &data);
        encoder.flush(&mut device);
        gl_surface.swap_buffers(&gl_context)?;
        device.cleanup();
    }

    info!("Simulate developer modifying shader to color green...");
    overwrite_fragment_shader(
        &FRAGMENT_SHADER.replace("gl_FragColor = white;", "gl_FragColor = green;"),
    )?;

    while Instant::now().duration_since(start) < show_duration * 3 {
        encoder.clear(&data.out, CLEAR_COLOR);
        encoder.draw(&slice, pso_cell.pso(), &data);
        encoder.flush(&mut device);
        gl_surface.swap_buffers(&gl_context)?;
        device.cleanup();
    }

    overwrite_fragment_shader(FRAGMENT_SHADER)?;

    Ok(())
}
