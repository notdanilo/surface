use crate::{SurfaceContext, Surface, SurfaceManager};

pub struct SurfaceBuilder<'manager, T: 'static> {
    pub cursor: bool,
    pub context: SurfaceContext,
    pub(crate) manager: &'manager SurfaceManager<T>
}

impl<'manager, T: 'static> SurfaceBuilder<'manager, T> {
    pub fn new(manager: &'manager SurfaceManager<T>) -> Self {
        let cursor = true;
        let context = SurfaceContext::Screen;
        Self { context, cursor, manager }
    }

    pub fn with_context(mut self, context: SurfaceContext) -> Self {
        self.context = context;
        self
    }

    pub fn with_cursor(mut self, cursor: bool) -> Self {
        self.cursor = cursor;
        self
    }

    pub fn build(self) -> Result<Surface, String> {
        Surface::new(self)
    }
}
