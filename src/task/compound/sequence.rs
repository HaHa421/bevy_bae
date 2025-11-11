//! Contains the [`Sequence`] [`CompoundTask`]

use crate::{
    plan::PlannedOperator,
    prelude::*,
    task::compound::{DecomposeId, DecomposeInput, DecomposeResult, TypeErasedCompoundTask},
};

/// A [`CompoundTask`] that decomposes into all subtasks, given that they are all valid.
#[derive(Debug, Component, Default, Reflect)]
#[reflect(Component)]
pub struct Sequence;

impl CompoundTask for Sequence {
    fn register_decompose(commands: &mut Commands) -> DecomposeId {
        commands.register_system(decompose_sequence)
    }
}

fn decompose_sequence(
    In(mut ctx): In<DecomposeInput>,
    world: &mut World,
    mut task_relations: Local<QueryState<&Tasks>>,
    mut individual_tasks: Local<
        QueryState<(
            Entity,
            AnyOf<(&Operator, &TypeErasedCompoundTask)>,
            Option<&Conditions>,
            Option<&Effects>,
        )>,
    >,
    mut conditions: Local<QueryState<&Condition>>,
    mut effects: Local<QueryState<&Effect>>,
) -> DecomposeResult {
    let Ok(tasks) = task_relations.get(world, ctx.compound_task) else {
        return DecomposeResult::Failure;
    };
    let individual_tasks: Vec<_> = individual_tasks
        .iter_many(world, tasks)
        .map(
            |(task_entity, (operator, compound_task), condition_relations, effect_relations)| {
                (
                    task_entity,
                    operator.cloned(),
                    compound_task.cloned(),
                    condition_relations.cloned(),
                    effect_relations.cloned(),
                )
            },
        )
        .collect();
    let mut found_anything = false;
    for (task_entity, operator, compound_task, condition_relations, effect_relations) in
        individual_tasks
    {
        let mut individual_conditions = Vec::new();
        if let Some(condition_relations) = condition_relations {
            for condition in conditions.iter_many(world, condition_relations.iter()) {
                if !condition.is_fullfilled(&mut ctx.world_state) {
                    return DecomposeResult::Failure;
                }
                individual_conditions.push(condition.clone());
            }
        }
        let conditions = if !found_anything {
            // Only the first "entry" subtask needs to inherit our conditions
            ctx.conditions.extend(individual_conditions);
            ctx.conditions.clone()
        } else {
            individual_conditions
        };
        if let Some(operator) = operator {
            ctx.plan.push_back(PlannedOperator {
                system: operator.system_id(),
                entity: task_entity,
                effects: vec![],
                conditions,
            });
        } else if let Some(compound_task) = compound_task {
            match world.run_system_with(
                compound_task.decompose,
                DecomposeInput {
                    planner: ctx.planner,
                    compound_task: task_entity,
                    world_state: ctx.world_state.clone(),
                    plan: ctx.plan.clone(),
                    previous_mtr: ctx.previous_mtr.clone(),
                    conditions,
                },
            ) {
                Ok(DecomposeResult::Success { plan, world_state }) => {
                    ctx.plan = plan;
                    ctx.world_state = world_state;
                }
                Ok(DecomposeResult::Rejection) => return DecomposeResult::Rejection,
                Ok(DecomposeResult::Failure) | Err(_) => return DecomposeResult::Failure,
            }
        } else {
            unreachable!()
        }
        if ctx.plan.is_empty() {
            return DecomposeResult::Failure;
        }
        if let Some(effect_relations) = effect_relations {
            for effect in effects.iter_many(world, effect_relations.iter()) {
                effect.apply(&mut ctx.world_state);
                ctx.plan.back_mut().unwrap().effects.push(effect.clone());
            }
        }
        found_anything = true;
    }

    if found_anything {
        DecomposeResult::Success {
            plan: ctx.plan,
            world_state: ctx.world_state,
        }
    } else {
        DecomposeResult::Failure
    }
}
