use alloc::collections::VecDeque;
use bevy_ecs::world::FilteredEntityRef;
use core::any::TypeId;

use disqualified::ShortName;

use crate::{
    prelude::*,
    task::{
        primitive::OperatorId,
        validation::{
            BaeTaskPresent, insert_bae_task_present_on_add, remove_bae_task_present_on_remove,
        },
    },
};

pub mod relationship;
pub mod select;
pub mod sequence;

pub trait CompoundTask: Send + Sync + 'static {
    fn decompose<'a>(ctx: DecomposeContext<'a>) -> DecomposeResult;
}

#[derive(Debug)]
pub struct DecomposeContext<'a> {
    pub root: Entity,
    pub compound_task: Entity,
    pub world: &'a mut World,
    pub world_state: &'a mut Props,
    pub plan: &'a mut Vec<OperatorId>,
    pub queries: DecomposeQueries<'a>,
}

#[derive(Debug)]
pub struct DecomposeQueries<'a> {
    pub conditions: &'a mut QueryState<&'static Condition>,
    pub effects: &'a mut QueryState<&'static Effect>,
    pub tasks: &'a mut QueryState<AnyOf<(&'static Operator, &'static TypeErasedCompoundTask)>>,
    pub names: &'a mut QueryState<NameOrEntity>,
}

#[derive(Component, Clone)]
pub(crate) struct TypeErasedCompoundTask {
    pub(crate) entity: Entity,
    pub(crate) name: ShortName<'static>,
    pub(crate) type_id: TypeId,
    pub(crate) decompose: for<'a> fn(DecomposeContext<'a>) -> DecomposeResult,
    pub(crate) tasks: for<'a> fn(&Self, &'a FilteredEntityRef) -> Option<&'a [Entity]>,
}

impl TypeErasedCompoundTask {
    #[must_use]
    fn new<C: CompoundTask>(entity: Entity) -> Self {
        Self {
            entity,
            name: ShortName::of::<C>(),
            type_id: TypeId::of::<C>(),
            decompose: C::decompose,
            tasks: Self::tasks_typed::<C>,
        }
    }
}

pub enum DecomposeResult {
    Success,
    Partial,
    Rejection,
    Failure,
}

impl TypeErasedCompoundTask {
    fn tasks_typed<'a, C: CompoundTask>(
        &self,
        context: &'a FilteredEntityRef,
    ) -> Option<&'a [Entity]> {
        context.get::<Tasks<C>>().map(|actions| &***actions)
    }
}

pub trait CompoundAppExt {
    fn add_compound_task<C: CompoundTask>(&mut self) -> &mut Self;
}

impl CompoundAppExt for App {
    fn add_compound_task<C: CompoundTask>(&mut self) -> &mut Self {
        self.add_observer(insert_type_erased_task::<C>)
            .add_observer(remove_type_erased_task::<C>)
            .add_observer(insert_bae_task_present_on_add::<Tasks<C>>)
            .add_observer(remove_bae_task_present_on_remove::<Tasks<C>>);
        let _ = self.try_register_required_components::<Tasks<C>, BaeTaskPresent>();
        self
    }
}

fn insert_type_erased_task<C: CompoundTask>(insert: On<Insert, Tasks<C>>, mut commands: Commands) {
    commands
        .entity(insert.entity)
        .try_insert(TypeErasedCompoundTask::new::<C>(insert.entity));
}
fn remove_type_erased_task<C: CompoundTask>(remove: On<Remove, Tasks<C>>, mut commands: Commands) {
    commands
        .entity(remove.entity)
        .try_remove::<TypeErasedCompoundTask>();
}
