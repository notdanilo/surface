mod window;

pub use window::*;

pub enum SurfaceContext {
    Window(Window),
    Screen
}
