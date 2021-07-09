// FIXME: Move it to another place.
pub type Size2 = (usize, usize);

// FIXME: Move Window to another crate (called display or surface or something)
//  `gpu` shouldn't handle mouse or keyboard.

pub struct OnResizeEvent {
    pub previous_size: Size2,
    pub size: Size2
}

impl OnResizeEvent {
    pub fn new(previous_size: Size2, size: Size2) -> Self {
        Self { previous_size, size }
    }
}

use winit::window::{WindowBuilder, Fullscreen};
use winit::event_loop::{EventLoop, ControlFlow};
use winit::event;

pub struct Surface {
    context: SurfaceContext,
    pub event_loop: EventLoop<()>,
    pub window: winit::window::Window,
    pub(crate) on_resize_callback: Option<Box<dyn FnMut(OnResizeEvent) + 'static>>
}

pub struct Window {
    name: String
}

impl Window {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

pub struct SurfaceBuilder {
    cursor: bool,
    context: SurfaceContext
}

pub enum SurfaceContext {
    Window(Window),
    FullScreen
}

impl SurfaceBuilder {
    pub fn new(context: SurfaceContext) -> Self {
        let cursor = true;
        Self { context, cursor }
    }

    pub fn with_cursor(mut self, cursor: bool) -> Self {
        self.cursor = cursor;
        self
    }

    pub fn build(self) -> Result<Surface, String> {
        Surface::new(self)
    }
}

impl Surface {
    fn new(builder: SurfaceBuilder) -> Result<Self, String> {
        let on_resize_callback = None;

        let event_loop = EventLoop::new();
        let mut window_builder = WindowBuilder::new();

        window_builder = match builder.context {
            // TODO: Explore Fullscreen enumeration possibilities.
            SurfaceContext::FullScreen => window_builder.with_fullscreen(Some(Fullscreen::Borderless(None))),
            SurfaceContext::Window(ref window) => {
                window_builder.with_title(&window.name)
            }
        };

        let window = window_builder.build(&event_loop);

        window.map(|window| {
            window.set_cursor_visible(builder.cursor);
            let context = builder.context;
            Self { context, event_loop, window, on_resize_callback }
        }).map_err(|error| format!("{:#?}", error))

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

    pub fn context(&self) -> &SurfaceContext {
        &self.context
    }

    pub fn context_mut(&mut self) -> &mut SurfaceContext {
        &mut self.context
    }

    pub fn run(self) -> ! {
        // let context = &mut self.context;
        // let display = &mut self.display;
        self.event_loop.run(move |event, _event_loop, control_flow| {
            // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
            // dispatched any events. This is ideal for games and similar applications.
            *control_flow = ControlFlow::Poll;

            // ControlFlow::Wait pauses the event loop if no events are available to process.
            // This is ideal for non-game applications that only update in response to user
            // input, and uses significantly less power/CPU time than ControlFlow::Poll.
            *control_flow = ControlFlow::Wait;

            match event {
                event::Event::WindowEvent { event, .. } => {
                    match event {
                        event::WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                        event::WindowEvent::Resized(_logical_size) => {
                            // let dpi_factor = context.get_hidpi_factor();
                            // let size = logical_size.to_physical(dpi_factor);
                            // context.resize(size);
                            // if let ContextDisplay::Window(window) = display {
                            //     window.set_size((size.width as usize, size.height as usize));
                            // }
                        },
                        event::WindowEvent::CursorMoved {device_id:_, position:_, ..} => {
                            // let dpi_factor = context.get_hidpi_factor();
                            // let position = position.to_physical(dpi_factor);
                            // println!("Cursor {}x{}", position.x, position.y);
                        },
                        event::WindowEvent::KeyboardInput {device_id:_,input,is_synthetic:_} => {
                            input.virtual_keycode.map(|_vk| {

                            });
                        }
                        _ => ()
                    }
                },
                _ => ()
            }
        })
    }

    pub fn on_resize<Callback: FnMut(OnResizeEvent) + 'static>(&mut self, callback: Option<Callback>) {
        self.on_resize_callback = callback.map(|callback| Box::new(callback) as Box<dyn FnMut(OnResizeEvent)>);
    }
}
