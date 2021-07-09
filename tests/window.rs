use surface::{SurfaceBuilder, SurfaceContext, Window};

#[test]
fn window() {
    let name    = "Title".into();
    let window  = Window::new(name);
    let context = SurfaceContext::Window(window);
    let surface = SurfaceBuilder::new(context).build().expect("Surface couldn't be created.");
    surface.run();
}