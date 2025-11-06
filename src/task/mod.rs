use crate::prelude::*;

pub mod compound;
pub mod primitive;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum TaskStatus {
    Continue,
    Success,
    Failure,
}

trait BaeTask: Component {}
