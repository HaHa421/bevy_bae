use crate::{
    prelude::*,
    task::{
        compound::{DecomposeContext, DecomposeResult},
        primitive::OperatorId,
    },
};

#[derive(Debug, Default, Reflect)]
pub struct Select;

impl CompoundTask for Select {
    fn decompose<'a>(ctx: DecomposeContext<'a>) -> DecomposeResult {
        todo!()
    }
}
