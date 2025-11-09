use crate::{
    prelude::*,
    task::{
        compound::{DecomposeId, DecomposeInput, DecomposeResult},
        primitive::OperatorId,
    },
};

#[derive(Debug, Default, Reflect)]
pub struct Select;

impl CompoundTask for Select {
    fn register_decompose(commands: &mut Commands) -> DecomposeId {
        commands.register_system(decompose_select)
    }
}

fn decompose_select(In(ctx): In<DecomposeInput>) -> DecomposeResult {
    DecomposeResult::Success
}
