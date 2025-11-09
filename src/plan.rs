use alloc::collections::VecDeque;

use bevy_mod_props::PropsExt;

use crate::prelude::*;
use crate::task::compound::{DecomposeInput, DecomposeResult, TypeErasedCompoundTask};
use crate::task::primitive::OperatorId;

#[derive(EntityEvent)]
pub struct UpdatePlan {
    #[event_target]
    entity: Entity,
}

#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct PlanningStack {
    #[reflect(ignore)]
    pub world_state: Vec<Props>,
    #[reflect(ignore)]
    pub plan: Vec<OperatorId>,
}

impl PlanningStack {
    pub fn clear(&mut self) {
        self.world_state.clear();
        self.plan.clear();
    }
}

pub(crate) fn update_plan(
    update: On<UpdatePlan>,
    world: &mut World,
    mut conditions: Local<QueryState<&Condition>>,
    mut tasks: Local<QueryState<AnyOf<(&Operator, &TypeErasedCompoundTask)>>>,
    mut names: Local<QueryState<NameOrEntity>>,
) -> Result {
    let root = update.entity;

    {
        if let Some(mut stack) = world.entity_mut(root).get_mut::<PlanningStack>() {
            stack.clear();
        } else {
            world.entity_mut(root).insert(PlanningStack::default());
        }
    }

    let mut world_state = world.entity(update.entity).props().clone();
    if let Some(condition_relations) = world.get::<Conditions>(root) {
        let is_fulfilled = conditions
            .iter_many(world, condition_relations)
            .all(|condition| condition.is_fullfilled(&mut world_state));
        if !is_fulfilled {
            return Ok(());
        }
    }

    let Ok((operator, task)) = tasks
        .get(world, root)
        .map(|(o, t)| (o.cloned(), t.cloned()))
    else {
        let name = names
            .get(world, root)
            .ok()
            .and_then(|name| name.name.map(|n| format!("{root} ({n})")))
            .unwrap_or_else(|| format!("{root}"));
        return Err(BevyError::from(format!(
            "{name}: Called `UpdatePlan` for an entity without any tasks. Ensure it has either an `Operator` or a `CompoundTask` like `Select` or `Sequence`"
        )));
    };
    if let Some(operator) = operator {
        // well that was easy: this root has just a single operator
        let mut root = world.entity_mut(root);
        root.get_mut::<PlanningStack>()
            .unwrap()
            .plan
            .push(operator.system_id());
    } else if let Some(compound_task) = task {
        let ctx = DecomposeInput {
            root,
            compound_task: root,
        };
        let mut root = world.entity_mut(root);
        root.get_mut::<PlanningStack>()
            .unwrap()
            .world_state
            .push(world_state);
        let result = world.run_system_with(compound_task.decompose, ctx)?;
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
