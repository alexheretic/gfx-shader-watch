use super::PsoCell;
use gfx::{traits::FactoryExt, *};
use log::{debug, error, info};
use notify::{self, Watcher};
use std::{
    error::Error,
    fs::File,
    io::prelude::*,
    path::{Path, PathBuf},
    sync::mpsc,
};

fn shader_bytes(path: &Path) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut shader = Vec::new();
    File::open(path)?.read_to_end(&mut shader)?;
    Ok(shader)
}

/// Container that watches shader files and reloads pipeline state object after modification
pub struct WatcherPsoCell<R: Resources, F: Factory<R>, I: pso::PipelineInit> {
    vertex_shader: PathBuf,
    fragment_shader: PathBuf,
    init: I,
    primitive: Primitive,
    rasterizer: state::Rasterizer,
    _watcher: notify::RecommendedWatcher,
    shader_mods: mpsc::Receiver<notify::Result<notify::Event>>,

    factory: F,
    pso: PipelineState<R, I::Meta>,
}

impl<R: Resources, F: Factory<R>, I: pso::PipelineInit + Clone> WatcherPsoCell<R, F, I> {
    fn recv_modified_pso(&mut self) -> Option<PipelineState<R, I::Meta>>
    where
        R: Resources,
        F: Factory<R>,
    {
        let mut path_changed = None;
        for notify::Event { paths, kind, .. } in self.shader_mods.try_iter().flatten() {
            if path_changed.is_none() && (kind.is_modify() || kind.is_create()) {
                path_changed = paths
                    .into_iter()
                    .find(|p| p == &self.vertex_shader || p == &self.fragment_shader)
            }
        }
        if let Some(changed) = path_changed {
            match self.build_pso() {
                Ok(pso) => {
                    info!("{:?} changed", changed);
                    return Some(pso);
                }
                Err(err) => error!("{:?}", err),
            };
        }
        None
    }

    fn build_pso(&mut self) -> Result<PipelineState<R, I::Meta>, Box<dyn Error>>
    where
        R: Resources,
        F: Factory<R>,
    {
        let fragment_shader = shader_bytes(&self.fragment_shader)?;

        let vertex_shader = shader_bytes(&self.vertex_shader)?;

        let set = self
            .factory
            .create_shader_set(&vertex_shader, &fragment_shader)?;
        Ok(self.factory.create_pipeline_state(
            &set,
            self.primitive,
            self.rasterizer,
            self.init.clone(),
        )?)
    }
}

impl<R: Resources, F: Factory<R>, I: pso::PipelineInit + Clone> PsoCell<R, F, I>
    for WatcherPsoCell<R, F, I>
{
    fn pso(&mut self) -> &mut PipelineState<R, I::Meta> {
        if let Some(updated) = self.recv_modified_pso() {
            self.pso = updated;
        }
        &mut self.pso
    }

    fn factory(&mut self) -> &mut F {
        &mut self.factory
    }
}

/// Builds `WatcherPsoCell`
#[derive(Debug)]
pub struct WatcherPsoCellBuilder<I: pso::PipelineInit> {
    vertex_shader: Option<PathBuf>,
    fragment_shader: Option<PathBuf>,
    primitive: Primitive,
    rasterizer: state::Rasterizer,
    init: I,
}

impl<I: pso::PipelineInit + Clone> WatcherPsoCellBuilder<I> {
    pub fn using(init_struct: I) -> WatcherPsoCellBuilder<I> {
        WatcherPsoCellBuilder {
            vertex_shader: None,
            fragment_shader: None,
            init: init_struct,
            primitive: Primitive::TriangleList,
            rasterizer: state::Rasterizer::new_fill(),
        }
    }

    pub fn vertex_shader<P: Into<PathBuf>>(mut self, path: P) -> WatcherPsoCellBuilder<I> {
        self.vertex_shader = Some(path.into());
        self
    }

    pub fn fragment_shader<P: Into<PathBuf>>(mut self, path: P) -> WatcherPsoCellBuilder<I> {
        self.fragment_shader = Some(path.into());
        self
    }

    pub fn primitive(mut self, p: Primitive) -> WatcherPsoCellBuilder<I> {
        self.primitive = p;
        self
    }

    pub fn rasterizer(mut self, r: state::Rasterizer) -> WatcherPsoCellBuilder<I> {
        self.rasterizer = r;
        self
    }

    pub fn build<R, F>(self, mut factory: F) -> Result<WatcherPsoCell<R, F, I>, Box<dyn Error>>
    where
        R: Resources,
        F: Factory<R>,
    {
        let (tx, shader_mods) = mpsc::channel();
        let mut watcher = notify::RecommendedWatcher::new(tx, <_>::default())?;
        let pso = {
            let vs = self.vertex_shader.as_ref().ok_or("missing vertex shader")?;
            let fs = self
                .fragment_shader
                .as_ref()
                .ok_or("missing fragment shader")?;
            let vs_dir = vs.parent().unwrap_or(vs);
            let fs_dir = fs.parent().unwrap_or(fs);

            debug!("Watching {:?}", &[vs, fs]);
            watcher.watch(vs_dir, notify::RecursiveMode::NonRecursive)?;
            if fs_dir != vs_dir {
                watcher.watch(fs_dir, notify::RecursiveMode::NonRecursive)?;
            }

            let fragment_shader = shader_bytes(fs)?;
            let vertex_shader = shader_bytes(vs)?;
            let set = factory.create_shader_set(&vertex_shader, &fragment_shader)?;
            factory.create_pipeline_state(
                &set,
                self.primitive,
                self.rasterizer,
                self.init.clone(),
            )?
        };

        Ok(WatcherPsoCell {
            vertex_shader: self.vertex_shader.ok_or("missing vertex shader")?,
            fragment_shader: self.fragment_shader.ok_or("missing fragment shader")?,
            init: self.init,
            primitive: self.primitive,
            rasterizer: self.rasterizer,
            _watcher: watcher,
            shader_mods,

            factory,
            pso,
        })
    }
}
