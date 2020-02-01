//! Provides ability to change shader source files and have them reload on the fly. Also provides
//! macros to simply include the shader code at compile time when running in release mode.
//!
//! # Examples
//! ```ignore
//! use gfx_shader_watch::*;
//! use gfx::{Primitive, state::Rasterizer};
//!
//! gfx_defines! {
//!     pipeline mypipeline {
//!         out: gfx::RenderTarget<gfx::format::Rgba8> = "Target0",
//!     }
//! }
//!
//! // Container has SimplePsoCell or WatcherPsoCell type, depending on compile mode
//! // if you need to refer to the type, use the `debug_watcher_pso_cell_type!` macro
//! let mut pso_cell = debug_watcher_pso_cell!(
//!     pipe = mypipeline,
//!     vertex_shader = "shader/vert.glsl",
//!     fragment_shader = "shader/frag.glsl",
//!     factory = factory,
//!     primitive = Primitive::TriangleList,
//!     rasterizer = Rasterizer::new_fill()).expect("psocell");
//!
//! encoder.draw(&slice, pso_cell.pso(), &data);
//! ```
mod psocell;

pub use crate::psocell::*;
