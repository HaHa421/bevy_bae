use bevy_ecs::system::SystemId;
use bevy_ecs::{lifecycle::HookContext, world::DeferredWorld};
use core::marker::PhantomData;

use crate::prelude::*;

#[derive(Component)]
pub struct RegisteredTaskSystem {
    pub system_id: SystemId<In<Entity>, TaskStatus>,
}

#[derive(Component)]
#[component(on_add = TaskSystem::<S, M>::queue_into_step)]
pub struct TaskSystem<S: System<In = In<Entity>, Out = TaskStatus>, M: Send + Sync + 'static> {
    system: Option<S>,
    marker: PhantomData<M>,
}

impl<S: System<In = In<Entity>, Out = TaskStatus>, M: Send + Sync + 'static> TaskSystem<S, M> {
    pub fn new<I>(system: I) -> Self
    where
        I: IntoSystem<In<Entity>, TaskStatus, M, System = S>,
    {
        Self {
            system: Some(IntoSystem::into_system(system)),
            marker: PhantomData,
        }
    }

    fn queue_into_step(mut world: DeferredWorld, ctx: HookContext) {
        let entity = ctx.entity;
        world.commands().queue(move |world: &mut World| -> Result {
            if world.get_entity(entity).is_err() {
                // Already despawned
                return Ok(());
            }
            let system = {
                let mut entity_world = world.entity_mut(entity);
                let Some(mut func_step) = entity_world.get_mut::<TaskSystem<S, M>>() else {
                    // Already removed
                    return Ok(());
                };
                func_step.system.take().unwrap()
            };
            let system_id = world.register_system(system);
            world
                .entity_mut(entity)
                .insert(RegisteredTaskSystem { system_id })
                .remove::<TaskSystem<S, M>>();

            Ok(())
        });
    }
}

impl<S: System<In = In<Entity>, Out = TaskStatus> + Clone, M: Send + Sync + 'static + Clone> Clone
    for TaskSystem<S, M>
{
    fn clone(&self) -> Self {
        Self {
            system: self.system.clone(),
            marker: PhantomData,
        }
    }
}
