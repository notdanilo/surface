use surface::{SurfaceBuilder, SurfaceManager};

#[test]
fn fullscreen() {
    let manager = SurfaceManager::new();
    let _surface = SurfaceBuilder::new(&manager).build().expect("Surface couldn't be created.");
    manager.run();
}