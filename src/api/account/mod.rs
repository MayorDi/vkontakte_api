mod methods;
pub use methods::*;

#[derive(Debug, Clone, Copy)]
pub struct User {
    pub id: usize,
    pub first_name: &'static str,
    pub last_name: &'static str,
}

impl User {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            first_name: "",
            last_name: "",
        }
    }
}