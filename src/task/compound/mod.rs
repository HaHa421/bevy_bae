use core::any::TypeId;
use std::collections::VecDeque;

use bevy_ecs::world::FilteredEntityRef;
use disqualified::ShortName;

use crate::{prelude::*, task::primitive::OperatorId};

pub mod relationship;
pub mod select;
pub mod sequence;

pub trait CompoundAppExt {
    fn add_compound_task<C: CompoundTask>(&mut self) -> &mut Self;
}

impl CompoundAppExt for App {
    fn add_compound_task<C: CompoundTask>(&mut self) -> &mut Self {
        todo!();
    }
}

pub trait CompoundTask: Component {
    fn decompose(
        entity: Entity,
        world: &World,
        props: &mut Props,
        tasks: &mut VecDeque<OperatorId>,
    );
}

#[derive(Component)]
struct TypeErasedCompoundTask {
    entity: Entity,
    name: ShortName<'static>,
    type_id: TypeId,
    tasks: for<'a> fn(&Self, &'a FilteredEntityRef) -> Option<&'a [Entity]>,
    decompose: for<'a> fn(
        entity: Entity,
        world: &'a World,
        props: &'a mut Props,
        tasks: &'a mut VecDeque<OperatorId>,
    ),
}

impl TypeErasedCompoundTask {
    #[must_use]
    fn new<C: CompoundTask>(entity: Entity) -> Self {
        Self {
            entity,
            name: ShortName::of::<C>(),
            type_id: TypeId::of::<C>(),
            tasks: Self::tasks_typed::<C>,
            decompose: C::decompose,
        }
    }

    fn tasks_typed<'a, C: CompoundTask>(
        &self,
        context: &'a FilteredEntityRef,
    ) -> Option<&'a [Entity]> {
        context.get::<Tasks<C>>().map(|actions| &***actions)
    }
}
