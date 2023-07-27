mod methods;
pub use methods::*;

pub struct User {
    pub id: usize,
    pub first_name: String,
    pub last_name: String,
}

impl User {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            first_name: String::new(),
            last_name: String::new(),
        }
    }
}