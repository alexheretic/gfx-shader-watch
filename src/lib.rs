//! Provides ability to change shader source files and have them reload on the fly. Also provides
//! macros to simply include the shader code at compile time when running in release mode.
//!
//! # Examples
//! ```rust,no_run
//! # #[macro_use] extern crate gfx;
//! # use gfx::traits::FactoryExt;
//! # use gfx::Device;
//! # use gfx::format::{Rgba8, Depth};
//! use gfx_shader_watch::*;
//! use gfx::{Primitive, state::Rasterizer};
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
//!     # let mut event_loop = glutin::event_loop::EventLoop::new();
//!     # let window_builder = glutin::window::WindowBuilder::new()
//!     #     .with_title("Triangle".to_string())
//!     #     .with_inner_size(glutin::dpi::PhysicalSize::new(1024, 768));
//!     # let context = glutin::ContextBuilder::new()
//!     #     .with_vsync(true);
//!     # let (window, mut device, mut factory, main_color, _main_depth) =
//!     #     gfx_window_glutin::init::<Rgba8, Depth, _>(window_builder, context, &event_loop).unwrap();
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
//!         rasterizer = Rasterizer::new_fill()).expect("psocell");
//!
//!     let mut running = true;
//!     while running {
//!         // ...
//!     #    encoder.clear(&data.out, [0.1, 0.2, 0.3, 1.0]);
//!         encoder.draw(&slice, pso_cell.pso(), &data);
//!         // ...
//!     #    encoder.flush(&mut device);
//!     #    window.swap_buffers().unwrap();
//!     #    device.cleanup();
//!     }
//! }
//! ```
mod psocell;

pub use crate::psocell::*;
