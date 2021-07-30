use surface::{Surface, SurfaceContext, Window, SurfaceBuilder, SurfaceManager};
use std::convert::TryFrom;

#[test]
fn window() {
    let manager = SurfaceManager::new();
    let name = String::from("window");
    let window = Window::new(name.clone());
    let context = SurfaceContext::Window(window);
    let builder = SurfaceBuilder::new(&manager).with_context(context);
    let _surface = Surface::try_from(builder).expect("Surface couldn't be created.");
    manager.run();
}

#[test]
fn winit_window() {
    let manager = SurfaceManager::from(winit::event_loop::EventLoop::new());
    let window = winit::window::WindowBuilder::new()
        .with_title("winit_window")
        .build(manager.event_loop())
        .expect("Couldn't create window.");
    let _surface = Surface::from(window);
    manager.run();
}
