use crate::prelude::*;

pub(crate) fn update_empty_plans(
    mut plans: Query<(Entity, NameOrEntity, &Plan)>,
    mut commands: Commands,
) {
    for (entity, name, plan) in plans.iter_mut() {
        if plan.is_empty() {
            commands.entity(entity).trigger(UpdatePlan::new);
            debug!(entity=?name.entity, name=?name.name, "Plan is empty, triggering replan.");
        }
    }
}

pub(crate) fn execute_plan(
    world: &mut World,
    mut plans: Local<QueryState<(Entity, NameOrEntity, &mut Plan)>>,
) {
    let plans = plans
        .iter(world)
        .filter_map(|(entity, name, plan)| {
            Some((entity, name.name.cloned(), plan.front()?.clone()))
        })
        .collect::<Vec<_>>();
    for (entity, name, planned_operator) in plans {
        if !world.entity_mut(entity).contains::<Props>() {
            world.entity_mut(entity).insert(Props::default());
        }
        debug!(?entity, ?name, "Executing plan");
        let mut all_conditions_met = true;
        {
            let mut entity_mut = world.entity_mut(entity);
            let mut props = entity_mut.get_mut::<Props>().unwrap();
            for condition in planned_operator.conditions {
                if condition.is_fullfilled(&mut props) {
                } else {
                    debug!(
                        ?entity,
                        ?name,
                        "Encountered unsatisfied condition, aborting plan"
                    );
                    all_conditions_met = false;
                    break;
                }
            }
        }
        let result: Result<OperatorStatus, _> = if all_conditions_met {
            let input = OperatorInput {
                planner: entity,
                operator: planned_operator.entity,
            };
            world.run_system_with(planned_operator.system, input)
        } else {
            Ok(OperatorStatus::Failure)
        };

        let force_replan = match result {
            Ok(OperatorStatus::Success) => {
                debug!(entity=?entity, name=?name, "Operator completed successfully, moving to next step");
                let step = world
                    .entity_mut(entity)
                    .get_mut::<Plan>()
                    .unwrap()
                    .pop_front()
                    .unwrap();

                let mut entity = world.entity_mut(entity);
                let mut props = entity.get_mut::<Props>().unwrap();
                for effect in step.effects {
                    if !effect.plan_only {
                        effect.apply(&mut props);
                    }
                }

                false
            }
            Ok(OperatorStatus::Ongoing) => {
                debug!(?entity, ?name, "Operator ongoing");
                // Even if the current plan is empty, we still want to continue the execution of the last step!
                continue;
            }
            Ok(OperatorStatus::Failure) => {
                debug!(?entity, ?name, "Operator failed, aborting plan");
                true
            }
            Err(err) => {
                debug!(
                    ?entity,
                    ?name,
                    ?err,
                    "Operator system failed, aborting plan"
                );
                true
            }
        };
        if force_replan
            || world
                .entity(entity)
                .get::<Plan>()
                .is_none_or(|plan| plan.is_empty())
        {
            world.entity_mut(entity).insert(Plan::default());
            debug!(?entity, ?name, "triggering replan");
        }
    }
}
