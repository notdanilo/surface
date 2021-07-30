pub struct Window {
    pub(crate) name: String
}

impl Window {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
