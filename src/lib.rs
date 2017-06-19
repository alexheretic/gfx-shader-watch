//! Provides ability to change shader source files and have them reload on the fly. Also provides
//! macros to simply include the shader code at compile time when running in release mode. 
//!
//! # Examples
//! ```rust
//! pub fn main() {
//!     // ... setup window / gfx factory & pipeline etc
//!
//!     // Container has SimplePsoCell or WatcherPsoCell type, depending on compile mode
//!     // if you need to refer to the type, use the `debug_watcher_pso_cell_type!` macro
//!     let mut pso_cell = debug_watcher_pso_cell!(
//!         pipe = mypipeline,
//!         vertex_shader = "shader/vert.glsl", // relative to this file
//!         fragment_shader = "shader/frag.glsl",
//!         factory = factory).expect("psocell");
//!
//!     loop {
//!       // ...
//!       encoder.draw(&slice, pso_cell.pso(), &data);
//!       // ...
//!     }
//! }
//! ```

#[macro_use] extern crate log;
extern crate gfx;
extern crate notify;

mod psocell;

pub use psocell::*;
