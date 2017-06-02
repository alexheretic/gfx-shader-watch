
#[cfg(debug_assertions)]
#[macro_export]
macro_rules! debug_watcher_pso_cell_type {
    ($r_type:ty, $f_type:ty, pipe = $pipe_name:ident) =>
        (WatcherPsoCell<$r_type, $f_type, $pipe_name::Init<'static>>)
}

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! debug_watcher_pso_cell {
    (pipe = $pipe_name:ident,
    vertex_shader = $vs:expr,
    fragment_shader = $fs:expr,
    factory = $factory:expr) => {{
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
