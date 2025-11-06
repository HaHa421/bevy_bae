use bevy_ecs::system::SystemId;
use bevy_ecs::{lifecycle::HookContext, world::DeferredWorld};
use core::marker::PhantomData;

use crate::prelude::*;

pub mod builtin;
pub mod relationship;

#[derive(Component)]
pub struct RegisteredCondition {
    pub system_id: SystemId<In<Entity>, bool>,
}

#[derive(Component)]
#[component(on_add = Condition::<S, M>::queue_into_condition)]
pub struct Condition<S: System<In = In<Entity>, Out = bool>, M: Send + Sync + 'static> {
    system: Option<S>,
    marker: PhantomData<M>,
}

pub trait IntoCondition {
    type System: System<In = In<Entity>, Out = bool> + Send + Sync + 'static;
    type Marker: Send + Sync + 'static;

    fn into_condition(self) -> Condition<Self::System, Self::Marker>;
}

impl<S: System<In = In<Entity>, Out = bool>, M: Send + Sync + 'static> Condition<S, M> {
    pub fn new<I>(system: I) -> Self
    where
        I: IntoSystem<In<Entity>, bool, M, System = S>,
    {
        Self {
            system: Some(IntoSystem::into_system(system)),
            marker: PhantomData,
        }
    }

    fn queue_into_condition(mut world: DeferredWorld, ctx: HookContext) {
        let entity = ctx.entity;
        world.commands().queue(move |world: &mut World| -> Result {
            if world.get_entity(entity).is_err() {
                // Already despawned
                return Ok(());
            }
            let system = {
                let mut entity_world = world.entity_mut(entity);
                let Some(mut func_condition) = entity_world.get_mut::<Condition<S, M>>() else {
                    // Already removed
                    return Ok(());
                };
                func_condition.system.take().unwrap()
            };
            let system_id = world.register_system(system);
            world
                .entity_mut(entity)
                .insert(RegisteredCondition { system_id })
                .remove::<Condition<S, M>>();

            Ok(())
        });
    }
}

impl<S: System<In = In<Entity>, Out = bool>, M: Send + Sync + 'static> IntoCondition
    for Condition<S, M>
{
    type System = S;
    type Marker = M;

    fn into_condition(self) -> Condition<Self::System, Self::Marker> {
        self
    }
}

impl<S: System<In = In<Entity>, Out = bool> + Clone, M: Send + Sync + 'static + Clone> Clone
    for Condition<S, M>
{
    fn clone(&self) -> Self {
        Self {
            system: self.system.clone(),
            marker: PhantomData,
        }
    }
}
