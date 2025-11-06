use crate::prelude::*;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Sequence;

impl CompoundTask for Sequence {}
