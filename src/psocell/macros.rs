
/// Returns WatcherPsoCell type when compiled in debug mode,
/// SimplePsoCell type when compiled in release mode
/// Type will match that returned by `debug_watcher_pso_cell` macro
#[cfg(debug_assertions)]
#[macro_export]
macro_rules! debug_watcher_pso_cell_type {
    ($r_type:ty, $f_type:ty, pipe = $pipe_name:ident) =>
        (WatcherPsoCell<$r_type, $f_type, $pipe_name::Init<'static>>)
}

/// Returns WatcherPsoCell instance when compiled in debug mode,
/// SimplePsoCell instance when compiled in release mode
/// The type itself can be attained similarly with the `debug_watcher_pso_cell_type` macro
///
/// # Examples
/// ```rust,no_run
/// #[macro_use] extern crate gfx;
/// #[macro_use] extern crate gfx_shader_watch;
/// # extern crate gfx_window_glutin;
/// # extern crate glutin;
/// # use glutin::GlContext;
/// # use gfx::format::{Rgba8, Depth};
///
/// gfx_defines!{
///     pipeline mypipeline {
///         out: gfx::RenderTarget<gfx::format::Rgba8> = "Target0",
///     }
/// }
///
/// pub fn main() {
///    // {code initialising a gfx `factory`}
///    # let events_loop = glutin::EventsLoop::new();
///    # let window_builder = glutin::WindowBuilder::new();
///    # let context = glutin::ContextBuilder::new();
///    # let (_window, mut _device, mut factory, _main_color, _main_depth) =
///    #     gfx_window_glutin::init::<Rgba8, Depth>(window_builder, context, &events_loop);
///
///    let mut _pso_cell = debug_watcher_pso_cell!(
///        pipe = mypipeline,
///        vertex_shader = "shader/vert.glsl", // relative to this file
///        fragment_shader = "shader/frag.glsl",
///        factory = factory).expect("psocell");
/// }
/// ```
#[cfg(debug_assertions)]
#[macro_export]
macro_rules! debug_watcher_pso_cell {
    (pipe = $pipe_name:ident,
    vertex_shader = $vs:expr,
    fragment_shader = $fs:expr,
    factory = $factory:expr,
    $($opt:ident = $opt_val:expr),+) => {{
        use std::path::Path;
        use $crate::WatcherPsoCellBuilder;

        match Path::new(file!()).canonicalize() {
            Ok(path) => match path.parent().ok_or("Could not find current dir") {
                Ok(dir) => {
                    let vs = $vs.split("/").fold(dir.to_path_buf(), |path, s| path.join(s));
                    let fs = $fs.split("/").fold(dir.to_path_buf(), |path, s| path.join(s));
                    WatcherPsoCellBuilder::using($pipe_name::new())
                        .vertex_shader(vs)
                        .fragment_shader(fs)
                        $(.$opt($opt_val))+
                        .build($factory)
                },
                Err(err) => Err(err.into())
            },
            Err(err) => Err(err.into())
        }
    }};

    (pipe = $pipe_name:ident,
    vertex_shader = $vs:expr,
    fragment_shader = $fs:expr,
    factory = $factory:expr) => {{
        use gfx::Primitive;
        debug_watcher_pso_cell!(pipe = $pipe_name,
                                vertex_shader = $vs,
                                fragment_shader = $fs,
                                factory = $factory,
                                primitive = Primitive::TriangleList)
    }};
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! debug_watcher_pso_cell_type {
    ($r_type:ty, $f_type:ty, pipe = $pipe_name:ident) =>
        (SimplePsoCell<$r_type, $f_type, $pipe_name::Init<'static>>)
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! debug_watcher_pso_cell {
    (pipe = $pipe_name:ident,
    vertex_shader = $vs:expr,
    fragment_shader = $fs:expr,
    factory = $factory:expr) => {{
        $crate::SimplePsoCellBuilder::using($pipe_name::new())
            .vertex_shader(include_bytes!($vs))
            .fragment_shader(include_bytes!($fs))
            .build($factory)
    }};

    (pipe = $pipe_name:ident,
    vertex_shader = $vs:expr,
    fragment_shader = $fs:expr,
    factory = $factory:expr,
    $($opt:ident = $opt_val:expr),+) => {{
        $crate::SimplePsoCellBuilder::using($pipe_name::new())
            .vertex_shader(include_bytes!($vs))
            .fragment_shader(include_bytes!($fs))
            $(.$opt($opt_val))+
            .build($factory)
    }};
}
