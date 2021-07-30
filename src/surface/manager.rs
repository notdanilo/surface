use winit::event_loop::{EventLoop, ControlFlow};

pub struct SurfaceManager<T: 'static> {
    event_loop: EventLoop<T>
}

impl<T: 'static> From<EventLoop<T>> for SurfaceManager<T> {
    fn from(event_loop: EventLoop<T>) -> Self {
        Self { event_loop }
    }
}

impl Default for SurfaceManager<()> {
    fn default() -> Self {
        let event_loop = EventLoop::new();
        Self { event_loop }
    }
}

impl SurfaceManager<()> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<T: 'static> SurfaceManager<T> {
    pub fn event_loop(&self) -> &EventLoop<T> { &self.event_loop }

    pub fn event_loop_mut(&mut self) -> &mut EventLoop<T> { &mut self.event_loop }

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
                winit::event::Event::WindowEvent { event, .. } => {
                    match event {
                        winit::event::WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                        winit::event::WindowEvent::Resized(_logical_size) => {
                            // let dpi_factor = context.get_hidpi_factor();
                            // let size = logical_size.to_physical(dpi_factor);
                            // context.resize(size);
                            // if let ContextDisplay::Window(window) = display {
                            //     window.set_size((size.width as usize, size.height as usize));
                            // }
                        },
                        winit::event::WindowEvent::CursorMoved {device_id:_, position:_, ..} => {
                            // let dpi_factor = context.get_hidpi_factor();
                            // let position = position.to_physical(dpi_factor);
                            // println!("Cursor {}x{}", position.x, position.y);
                        },
                        winit::event::WindowEvent::KeyboardInput {device_id:_,input,is_synthetic:_} => {
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
}