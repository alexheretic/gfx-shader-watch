#[macro_use]
extern crate gfx;
// #[macro_use] extern crate gfx_shader_watch;
// extern crate gfx_window_glutin;
// extern crate glutin;

use gfx::format::Rgba8;

gfx_defines!{
    vertex Vertex {
        pos: [f32; 2] = "pos",
    }

    pipeline trianglepipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        out: gfx::RenderTarget<Rgba8> = "Target0",
    }
}

#[test]
fn watch_for_modifications() {
    // TODO when gfx/glutin headless support is added
}

#[test]
fn watch_for_remove_replace() {
    // TODO when gfx/glutin headless support is added
}
