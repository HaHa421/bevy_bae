use crate::prelude::*;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Select;

impl CompoundTask for Select {}
