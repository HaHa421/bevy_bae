use alloc::collections::VecDeque;

use bevy_mod_props::PropsExt;

use crate::prelude::*;
use crate::task::compound::{
    DecomposeContext, DecomposeQueries, DecomposeResult, TypeErasedCompoundTask,
};

#[derive(EntityEvent)]
pub struct UpdatePlan {
    #[event_target]
    entity: Entity,
}

pub(crate) fn update_plan(
    update: On<UpdatePlan>,
    world: &mut World,
    mut conditions: Local<QueryState<&Condition>>,
    mut effects: Local<QueryState<&Effect>>,
    mut tasks: Local<QueryState<AnyOf<(&Operator, &TypeErasedCompoundTask)>>>,
    mut names: Local<QueryState<NameOrEntity>>,
) -> Result {
    let root = update.entity;
    let mut world_state = world.entity(update.entity).props().clone();
    if let Some(condition_relations) = world.get::<Conditions>(root) {
        let is_fulfilled = conditions
            .iter_many(world, condition_relations)
            .all(|condition| condition.is_fullfilled(&mut world_state));
        if !is_fulfilled {
            // RestoreToLastDecomposedTask
            return Ok(());
        }
    }
    let mut plan = Vec::new();

    let mut entity_and_name = |world, entity| {
        names
            .get(world, entity)
            .ok()
            .and_then(|name| name.name.map(|n| format!("{entity} ({n})")))
            .unwrap_or_else(|| format!("{entity}"))
    };

    let Ok((operator, task)) = tasks
        .get(world, root)
        .map(|(o, t)| (o.cloned(), t.cloned()))
    else {
        let name = entity_and_name(world, root);
        return Err(BevyError::from(format!(
            "{name}: Called `UpdatePlan` for an entity without any tasks. Ensure it has either an `Operator` or a `CompoundTask` like `Select` or `Sequence`"
        )));
    };
    if let Some(operator) = operator {
        // well that was easy: this root has just a single operator
        plan.push(operator.system_id());
    } else if let Some(compound_task) = task {
        let ctx = DecomposeContext {
            root,
            compound_task: root,
            world,
            world_state: &mut world_state,
            plan: &mut plan,
            queries: DecomposeQueries {
                conditions: &mut conditions,
                effects: &mut effects,
                tasks: &mut tasks,
                names: &mut names,
            },
        };
        let result = (compound_task.decompose)(ctx);
        match result {
            DecomposeResult::Success => {
                todo!();
            }
            DecomposeResult::Failure => {
                todo!();
            }
            DecomposeResult::Partial => todo!(),
            DecomposeResult::Rejection => todo!(),
        }
    } else {
        unreachable!(
            "Bevy should guarantee that `AnyOf` contains at least one element that is `Some`"
        )
    }
    // No need to apply the effects of the root, as they cannot affect any planning.
    // But if we ever decided to automatically apply effects to the real props, we should put that here!

    Ok(())
}
