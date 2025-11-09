use crate::{
    prelude::*,
    task::{
        compound::{DecomposeContext, DecomposeResult, TypeErasedCompoundTask},
        primitive::OperatorId,
    },
};

#[derive(Debug, Default, Reflect)]
pub struct Sequence;

impl CompoundTask for Sequence {
    fn decompose<'a>(ctx: DecomposeContext<'a>) -> DecomposeResult {
        DecomposeResult::Success
    }
}
