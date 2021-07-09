use surface::{SurfaceBuilder, SurfaceContext, Window};

#[test]
fn fullscreen() {
    let context = SurfaceContext::FullScreen;
    let surface = SurfaceBuilder::new(context).build().expect("Surface couldn't be created.");
    surface.run();
}