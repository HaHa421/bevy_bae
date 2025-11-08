use crate::{prelude::*, task::primitive::OperatorId};

#[derive(Debug, Default, Reflect)]
pub struct Select;

impl CompoundTask for Select {
    fn decompose(
        _entity: Entity,
        _world: &World,
        _props: &mut Props,
        _tasks: &mut alloc::collections::VecDeque<OperatorId>,
        _index: usize,
    ) {
        todo!()
    }
}
