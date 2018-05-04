//! Provides ability to change shader source files and have them reload on the fly. Also provides
//! macros to simply include the shader code at compile time when running in release mode.
//!
//! # Examples
//! ```rust,no_run
//! #[macro_use] extern crate gfx;
//! #[macro_use] extern crate gfx_shader_watch;
//! # extern crate gfx_window_glutin;
//! # extern crate glutin;
//!
//! # use glutin::GlContext;
//! # use gfx::traits::FactoryExt;
//! # use gfx::Device;
//! # use gfx::format::{Rgba8, Depth};
//! use gfx_shader_watch::*;
//! use gfx::Primitive;
//! use gfx::state::Rasterizer;
//!
//! # gfx_defines!{
//! #     vertex Vertex {
//! #         pos: [f32; 2] = "pos",
//! #     }
//! #     pipeline mypipeline {
//! #         vbuf: gfx::VertexBuffer<Vertex> = (),
//! #         out: gfx::RenderTarget<Rgba8> = "Target0",
//! #     }
//! # }
//! # const TRIANGLE: [Vertex; 3] = [
//! #     Vertex { pos: [ -0.5, -0.5 ] },
//! #     Vertex { pos: [  0.5, -0.5 ] },
//! #     Vertex { pos: [  0.0,  0.5 ] }
//! # ];
//! pub fn main() {
//!     // {code to setup window / gfx factory etc }
//!     # let mut events_loop = glutin::EventsLoop::new();
//!     # let window_builder = glutin::WindowBuilder::new()
//!     #     .with_title("Triangle".to_string())
//!     #     .with_dimensions(1024, 768);
//!     # let context = glutin::ContextBuilder::new()
//!     #     .with_vsync(true);
//!     # let (window, mut device, mut factory, main_color, _main_depth) =
//!     #     gfx_window_glutin::init::<Rgba8, Depth>(window_builder, context, &events_loop);
//!     # let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();
//!     # let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&TRIANGLE, ());
//!     # let data = mypipeline::Data {
//!     #     vbuf: vertex_buffer,
//!     #     out: main_color
//!     # };
//!
//!     // Container has SimplePsoCell or WatcherPsoCell type, depending on compile mode
//!     // if you need to refer to the type, use the `debug_watcher_pso_cell_type!` macro
//!     let mut pso_cell = debug_watcher_pso_cell!(
//!         pipe = mypipeline,
//!         vertex_shader = "shader/vert.glsl",
//!         fragment_shader = "shader/frag.glsl",
//!         factory = factory,
//!         primitive = Primitive::TriangleList,
//!         raterizer = Rasterizer::new_fill()).expect("psocell");
//!
//!     let mut running = true;
//!     while running {
//!         // ...
//!     #    events_loop.poll_events(|event| {
//!     #        if let glutin::Event::WindowEvent{ event, .. } = event {
//!     #            match event {
//!     #                glutin::WindowEvent::KeyboardInput {
//!     #                    input: glutin::KeyboardInput {
//!     #                        virtual_keycode: Some(glutin::VirtualKeyCode::Escape),
//!     #                        .. },
//!     #                    ..
//!     #                } |
//!     #                glutin::WindowEvent::CloseRequested => running = false,
//!     #                _ => {},
//!     #            }
//!     #        }
//!     #    });
//!     #    encoder.clear(&data.out, [0.1, 0.2, 0.3, 1.0]);
//!         encoder.draw(&slice, pso_cell.pso(), &data);
//!         // ...
//!     #    encoder.flush(&mut device);
//!     #    window.swap_buffers().unwrap();
//!     #    device.cleanup();
//!     }
//! }
//! ```

#[macro_use] extern crate log;
extern crate gfx;
extern crate notify;

mod psocell;

pub use psocell::*;
