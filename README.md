Gfx Shader Watch
================

Tool for [gfx-rs](https://github.com/gfx-rs/gfx) providing a PsoCell container that:
* (Debug mode) Watches for shader file changes and reloads automatically
* (Release mode) Includes shader file bytes at compile time

Watching and auto-loading shader file changes allows faster development of shader code without full program restarts or re-compiles. However, when releasing a final binary it is more convenient to simply include the shader code in source.
Hence, this library has the capability to automatically act appropriately in either mode.

## How It Works
There are two PsoCell variants `SimplePsoCell` & `WatcherPsoCell`, the former simply builds it's PipelineState once and
provides access. The latter refers to a shader source file that it will monitor, when changed it will reload it's
PipelineState on next access. To facilitate using `SimplePsoCell` in release mode, and `WatcherPsoCell` in debug mode
the `debug_watcher_pso_cell!` & `debug_watcher_pso_cell_type!` macros are available.

For example:
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
