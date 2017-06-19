gfx_shader_watch
================
<a href="https://crates.io/crates/gfx_shader_watch">
  <img src="http://img.shields.io/crates/v/gfx_shader_watch.svg" alt = "gfx_shader_watch on crates.io">
</a>
<a href="https://docs.rs/gfx_shader_watch">
  <img src="https://docs.rs/gfx_shader_watch/badge.svg" alt="Documentation on docs.rs">
</a>

Tool for [gfx-rs](https://github.com/gfx-rs/gfx) providing a PsoCell container that:
* (Debug mode) Watches for shader file changes and reloads automatically
* (Release mode) Includes shader file bytes at compile time

Watching and auto-loading shader file changes allows faster development of shader code without full program restarts or re-compiles. However, when releasing a final binary it is more convenient to simply include the shader code in source.
Naturaly this library can automatically act the desired way in either mode.

## How It Works
There are two PsoCell variants `SimplePsoCell` & `WatcherPsoCell`, the former simply builds it's PipelineState once and
provides access. The latter refers to a shader source file that it will monitor, when changed it will reload it's
PipelineState on next access. To facilitate using `SimplePsoCell` in release mode, and `WatcherPsoCell` in debug mode
the `debug_watcher_pso_cell!` & `debug_watcher_pso_cell_type!` macros are available.

Code example:
```rust
pub fn main() {
    // ... setup window / gfx factory & pipeline etc

    // Container has SimplePsoCell or WatcherPsoCell type, depending on compile mode
    // if you need to refer to the type, use the `debug_watcher_pso_cell_type!` macro
    let mut pso_cell = debug_watcher_pso_cell!(
        pipe = mypipeline,
        vertex_shader = "shader/vert.glsl", // relative to this file
        fragment_shader = "shader/frag.glsl",
        factory = factory).expect("psocell");

    loop {
      // ...
      encoder.draw(&slice, pso_cell.pso(), &data);
      // ...
    }
}
```

## Examples
Try running `cargo run --example watch-shaders` you should see a white triangle. Now open `examples/shader/frag.glsl` and modify it (ie change `gl_FragColor = white;` -> `gl_FragColor = red;`). You'll see the triangle shaded with the new code without the program reloading.
