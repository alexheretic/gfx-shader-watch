mod watcher;
#[macro_use]
pub mod macros;

pub use crate::psocell::watcher::{WatcherPsoCell, WatcherPsoCellBuilder};

use gfx::{traits::FactoryExt, *};
use std::error::Error;

/// Container for pipeline state object and a factory
pub trait PsoCell<R: Resources, F: Factory<R>, I: pso::PipelineInit> {
    fn pso(&mut self) -> &mut PipelineState<R, I::Meta>;
    fn factory(&mut self) -> &mut F;
}

/// Container with compile time shader source, pipeline state is build on initialisation
#[derive(Debug)]
pub struct SimplePsoCell<R: Resources, F: Factory<R>, I: pso::PipelineInit> {
    pso: PipelineState<R, I::Meta>,
    factory: F,
}

impl<R: Resources, F: Factory<R>, I: pso::PipelineInit> PsoCell<R, F, I>
    for SimplePsoCell<R, F, I>
{
    #[inline]
    fn pso(&mut self) -> &mut PipelineState<R, I::Meta> {
        &mut self.pso
    }
    #[inline]
    fn factory(&mut self) -> &mut F {
        &mut self.factory
    }
}

/// Builds `SimplePsoCell`
#[derive(Debug)]
pub struct SimplePsoCellBuilder<I: pso::PipelineInit> {
    vertex_shader: Option<Vec<u8>>,
    fragment_shader: Option<Vec<u8>>,
    primitive: Primitive,
    rasterizer: state::Rasterizer,
    init: I,
}

impl<I: pso::PipelineInit + Clone> SimplePsoCellBuilder<I> {
    pub fn using(init_struct: I) -> SimplePsoCellBuilder<I> {
        SimplePsoCellBuilder {
            vertex_shader: None,
            fragment_shader: None,
            init: init_struct,
            primitive: Primitive::TriangleList,
            rasterizer: state::Rasterizer::new_fill(),
        }
    }

    pub fn vertex_shader(mut self, bytes: &[u8]) -> SimplePsoCellBuilder<I> {
        self.vertex_shader = Some(bytes.into());
        self
    }

    pub fn fragment_shader(mut self, bytes: &[u8]) -> SimplePsoCellBuilder<I> {
        self.fragment_shader = Some(bytes.into());
        self
    }

    pub fn primitive(mut self, p: Primitive) -> SimplePsoCellBuilder<I> {
        self.primitive = p;
        self
    }

    pub fn rasterizer(mut self, r: state::Rasterizer) -> SimplePsoCellBuilder<I> {
        self.rasterizer = r;
        self
    }

    pub fn build<R, F>(self, mut factory: F) -> Result<SimplePsoCell<R, F, I>, Box<dyn Error>>
    where
        R: Resources,
        F: Factory<R>,
    {
        let vs = self.vertex_shader.ok_or("missing vertex shader")?;
        let fs = self.fragment_shader.ok_or("missing fragment shader")?;
        let set = factory.create_shader_set(&vs, &fs)?;
        let pso =
            factory.create_pipeline_state(&set, self.primitive, self.rasterizer, self.init)?;
        Ok(SimplePsoCell { pso, factory })
    }
}
