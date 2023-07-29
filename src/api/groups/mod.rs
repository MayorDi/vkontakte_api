//! # Groups
//! 
//! Методы для работы с сообществами.

mod methods;
pub use methods::*;

pub struct Group {
    pub group_id: usize
}

impl Group {
    pub fn new(group_id: usize) -> Self {
        Group { group_id }
    }
}