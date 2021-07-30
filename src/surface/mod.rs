use winit::window::{Fullscreen, WindowBuilder};

mod builder;
mod context;
mod manager;

pub use builder::*;
pub use context::*;
pub use manager::*;

use crate::OnResizeEvent;
use std::convert::{TryFrom, TryInto};

pub struct Surface {
    window: winit::window::Window,
    pub(crate) on_resize_callback: Option<Box<dyn FnMut(OnResizeEvent) + 'static>>
}

impl From<winit::window::Window> for Surface {
    fn from(window: winit::window::Window) -> Self {
        let on_resize_callback = None;
        Self { window, on_resize_callback }
    }
}

impl<'manager, T: 'static> TryFrom<SurfaceBuilder<'manager, T>> for Surface {
    type Error = String;
    fn try_from(builder: SurfaceBuilder<'manager, T>) -> Result<Self, Self::Error> {
        let mut window_builder = WindowBuilder::new();
        window_builder = match builder.context {
            // TODO: Explore Fullscreen enumeration possibilities.
            SurfaceContext::Screen => window_builder.with_fullscreen(Some(Fullscreen::Borderless(None))),
            SurfaceContext::Window(window) => window_builder.with_title(&window.name)
        };

        let window = window_builder.build(builder.manager.event_loop());

        window
            .map(|window| window.into())
            .map_err(|error| format!("{:#?}", error))

        // match &builder.surface {
        //     ContextDisplay::Window(window) => {
        //         window_builder = window_builder.with_title(window.title())
        //             .with_dimensions(glutin::dpi::LogicalSize::new(window.size().0 as f64, window.size().1 as f64));
        //     },
        //     ContextDisplay::Screen => {
        //         window_builder = window_builder.with_title("")
        //             .with_fullscreen(Some(events_loop.get_primary_monitor()));
        //     },
        //     ContextDisplay::None => {
        //         window_builder = window_builder.with_title("")
        //             .with_fullscreen(Some(events_loop.get_primary_monitor()))
        //             .with_visibility(false);
        //     }
        // }
    }
}

impl Surface {
    fn new<T: 'static>(builder: SurfaceBuilder<T>) -> Result<Self, String> {
        builder.try_into()
    }

    pub fn on_resize<Callback: FnMut(OnResizeEvent) + 'static>(&mut self, callback: Option<Callback>) {
        self.on_resize_callback = callback.map(|callback| Box::new(callback) as Box<dyn FnMut(OnResizeEvent)>);
    }

    pub fn window(&self) -> &winit::window::Window {
        &self.window
    }

    pub fn window_mut(&mut self) -> &mut winit::window::Window {
        &mut self.window
    }
}
