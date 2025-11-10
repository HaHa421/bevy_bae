use bevy_ecs::error::{DefaultErrorHandler, HandleError as _};
use bevy_ecs::system::command::run_system_cached_with;
use bevy_mod_props::PropsExt;

use crate::plan::PlannedOperator;
use crate::plan::mtr::Mtr;
use crate::prelude::*;
use crate::task::compound::{DecomposeInput, DecomposeResult, TypeErasedCompoundTask};

#[derive(EntityEvent)]
pub struct UpdatePlan {
    #[event_target]
    entity: Entity,
}

impl From<Entity> for UpdatePlan {
    fn from(entity: Entity) -> Self {
        Self { entity }
    }
}

impl UpdatePlan {
    pub fn new(entity: Entity) -> Self {
        Self::from(entity)
    }
}

pub(crate) fn update_plan(
    update: On<UpdatePlan>,
    mut commands: Commands,
    error_handler: Option<Res<DefaultErrorHandler>>,
) {
    let entity = update.entity;
    let error_handler = error_handler.map(|h| *h).unwrap_or_default();
    commands.queue(
        run_system_cached_with(update_plan_inner, UpdatePlan { entity })
            .handle_error_with(error_handler.0),
    );
}

fn update_plan_inner(
    update: In<UpdatePlan>,
    world: &mut World,
    mut conditions: Local<QueryState<(Entity, NameOrEntity, &Condition)>>,
    mut effects: Local<QueryState<(Entity, NameOrEntity, &Effect)>>,
    mut tasks: Local<QueryState<AnyOf<(&Operator, &TypeErasedCompoundTask)>>>,
    mut names: Local<QueryState<NameOrEntity>>,
) -> Result {
    let root = update.entity;
    let behav_name = names
        .get(world, root)
        .ok()
        .and_then(|name| name.name.map(|n| format!("{root} ({n})")))
        .unwrap_or_else(|| format!("{root}"));

    debug!("behavior {behav_name}: Updating plan");

    let mut world_state = world.entity(update.entity).props().clone();
    if let Some(condition_relations) = world.get::<Conditions>(root) {
        for (entity, name, condition) in conditions.iter_many(world, condition_relations) {
            let name = name
                .name
                .map(|n| format!("{entity} ({n})"))
                .unwrap_or_else(|| format!("{entity}"));
            let is_fulfilled = condition.is_fullfilled(&mut world_state);
            debug!("behavior {behav_name} -> condition {name}: {is_fulfilled}");
            if !is_fulfilled {
                debug!("behavior {behav_name}: aborting update due to unfulfilled condition");
                world.entity_mut(root).insert(Plan::default());
                return Ok(());
            }
        }
    }

    let Ok((operator, task)) = tasks
        .get(world, root)
        .map(|(o, t)| (o.cloned(), t.cloned()))
    else {
        world.entity_mut(root).insert(Plan::default());
        return Err(BevyError::from(format!(
            "{behav_name}: Called `update_plan` for an entity without any tasks. Ensure it has either an `Operator` or a `CompoundTask` like `Select` or `Sequence`"
        )));
    };
    let mut plan = if let Some(operator) = operator {
        // well that was easy: this root has just a single operator
        debug!("behavior {behav_name}: operator");
        Plan {
            operators: [PlannedOperator {
                system: operator.system_id(),
                entity: root,
                effects: vec![],
            }]
            .into(),
            mtr: Mtr::default(),
            full_entities: Vec::new(),
        }
    } else if let Some(compound_task) = task {
        debug!("behavior {behav_name}: compound task");
        let previous_mtr = if let Some(plan) = world.entity(root).get::<Plan>() {
            plan.mtr.clone()
        } else {
            Mtr::none()
        };
        let ctx = DecomposeInput {
            world_state,
            plan: Plan::default(),
            root,
            compound_task: root,
            previous_mtr: previous_mtr.clone(),
        };
        let result = world.run_system_with(compound_task.decompose, ctx)?;
        match result {
            DecomposeResult::Success { plan, .. } => {
                if previous_mtr == plan.mtr
                    && world.entity(root).get::<Plan>().is_some_and(|prev_plan| {
                        prev_plan.full_entities.len() == plan.operators.len()
                            && prev_plan
                                .full_entities
                                .iter()
                                .zip(plan.operators.iter())
                                .all(|(a, b)| *a == b.entity)
                    })
                {
                    // We found the same plan we are already running. Just keep that one.
                    return Ok(());
                }
                error!("prev mtr: {:?}", previous_mtr);
                error!("current mtr: {:?}", plan.mtr);
                plan
            }
            DecomposeResult::Failure => Plan::default(),
            DecomposeResult::Rejection => return Ok(()),
        }
    } else {
        unreachable!(
            "Bevy should guarantee that `AnyOf` contains at least one element that is `Some`"
        )
    };

    if !plan.is_empty()
        && let Some(effect_relations) = world.get::<Effects>(root)
    {
        for (entity, name, effect) in effects.iter_many(world, effect_relations) {
            let name = name
                .name
                .map(|n| format!("{entity} ({n})"))
                .unwrap_or_else(|| format!("{entity}"));
            debug!("behavior {behav_name} -> effect {name}: queued");
            plan.back_mut().unwrap().effects.push(effect.clone());
        }
    }
    debug!("behavior {behav_name}: finished with {plan:?}");

    let op_entities = plan
        .operators
        .iter()
        .map(|op| op.entity)
        .collect::<Vec<_>>();
    plan.full_entities = op_entities;

    world.entity_mut(root).insert(plan);
    Ok(())
}
