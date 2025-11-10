use alloc::collections::VecDeque;
use bevy_derive::DerefMut;

use crate::{plan::mtr::Mtr, prelude::*, task::primitive::OperatorId};

pub mod execution;
pub mod mtr;
pub mod update;

#[derive(Component, Clone, Default, Reflect, Debug, Deref, DerefMut)]
#[reflect(Component)]
pub struct Plan {
    #[reflect(ignore)]
    #[deref]
    pub operators: VecDeque<PlannedOperator>,
    pub mtr: Mtr,
}

#[derive(Clone, Debug)]
pub struct PlannedOperator {
    pub system: OperatorId,
    pub entity: Entity,
    pub effects: Vec<Effect>,
}
