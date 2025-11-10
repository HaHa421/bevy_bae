use bevy_ecs::system::SystemId;

use crate::{
    plan::{Plan, mtr::Mtr},
    prelude::*,
};

pub mod relationship;
pub mod select;
pub mod sequence;

pub trait CompoundTask: Component {
    fn register_decompose(commands: &mut Commands) -> DecomposeId;
}

pub type DecomposeId = SystemId<In<DecomposeInput>, DecomposeResult>;

#[derive(Debug)]
pub struct DecomposeInput {
    pub root: Entity,
    pub compound_task: Entity,
    pub world_state: Props,
    pub plan: Plan,
    pub previous_mtr: Mtr,
}

#[derive(Component, Clone)]
pub(crate) struct TypeErasedCompoundTask {
    pub(crate) decompose: DecomposeId,
}

impl TypeErasedCompoundTask {
    #[must_use]
    fn new(id: DecomposeId) -> Self {
        Self { decompose: id }
    }
}

pub enum DecomposeResult {
    Success { plan: Plan, world_state: Props },
    Rejection,
    Failure,
}

pub trait CompoundAppExt {
    fn add_compound_task<C: CompoundTask>(&mut self) -> &mut Self;
}

impl CompoundAppExt for App {
    fn add_compound_task<C: CompoundTask>(&mut self) -> &mut Self {
        self.add_observer(insert_type_erased_task::<C>)
            .add_observer(remove_type_erased_task::<C>);
        self
    }
}

fn insert_type_erased_task<C: CompoundTask>(insert: On<Insert, C>, mut commands: Commands) {
    let system_id = C::register_decompose(&mut commands);
    commands
        .entity(insert.entity)
        .try_insert(TypeErasedCompoundTask::new(system_id));
}
fn remove_type_erased_task<C: CompoundTask>(remove: On<Remove, C>, mut commands: Commands) {
    commands
        .entity(remove.entity)
        .try_remove::<TypeErasedCompoundTask>();
}
