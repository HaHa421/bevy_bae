use crate::{
    prelude::*,
    task::{BaeTask, primitive::OperatorId},
};

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct Select;

impl CompoundTask for Select {
    fn decompose(
        entity: Entity,
        world: &World,
        props: &mut Props,
        tasks: &mut std::collections::VecDeque<OperatorId>,
    ) {
        todo!()
    }
}
