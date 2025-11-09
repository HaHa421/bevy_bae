use crate::{
    prelude::*,
    task::{
        compound::{DecomposeId, DecomposeInput, DecomposeResult, TypeErasedCompoundTask},
        primitive::OperatorId,
    },
};

#[derive(Debug, Default, Reflect)]
pub struct Sequence;

impl CompoundTask for Sequence {
    fn register_decompose(commands: &mut Commands) -> DecomposeId {
        commands.register_system(decompose_sequence)
    }
}

fn decompose_sequence(In(ctx): In<DecomposeInput>) -> DecomposeResult {
    DecomposeResult::Success
}
