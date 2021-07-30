use crate::Size2;

pub struct OnResizeEvent {
    pub previous_size: Size2,
    pub size: Size2
}

impl OnResizeEvent {
    pub fn new(previous_size: Size2, size: Size2) -> Self {
        Self { previous_size, size }
    }
}
