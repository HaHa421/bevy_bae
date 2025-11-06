use crate::prelude::*;

pub struct AlwaysTrue;

impl Into<Condition> for AlwaysTrue {
    fn into(self) -> Condition {
        Condition::new(|_| true)
    }
}
