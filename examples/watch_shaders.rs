use gfx::{
    self,
    format::{Depth, Srgba8},
    state::Rasterizer,
    traits::FactoryExt,
    Device, Primitive, *,
};
use gfx_shader_watch::*;
use glutin::{
    event::{Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use old_school_gfx_glutin_ext::*;
use std::{env, error::Error};

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

    // winit select x11 by default
    if cfg!(target_os = "linux") && env::var("WINIT_UNIX_BACKEND").is_err() {
        env::set_var("WINIT_UNIX_BACKEND", "x11");
    }

    let event_loop = EventLoop::new();
    let window_builder = WindowBuilder::new()
        .with_title("Triangle".to_string())
        .with_inner_size(glutin::dpi::PhysicalSize::new(1024, 768));

    let (window_ctx, mut device, mut factory, main_color, _main_depth) =
        glutin::ContextBuilder::new()
            .with_gfx_color_depth::<Srgba8, Depth>()
            .build_windowed(window_builder, &event_loop)?
            .init_gfx::<Srgba8, Depth>();

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

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::MainEventsCleared => window_ctx.window().request_redraw(),
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
            Event::RedrawRequested(_) => {
                encoder.clear(&data.out, CLEAR_COLOR);
                encoder.draw(&slice, pso_cell.pso(), &data);
                encoder.flush(&mut device);
                window_ctx.swap_buffers().unwrap();
                device.cleanup();
            }
            _ => (),
        }
    });
}
