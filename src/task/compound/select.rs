use crate::{
    prelude::*,
    task::compound::{DecomposeId, DecomposeInput, DecomposeResult, TypeErasedCompoundTask},
};

#[derive(Debug, Default, Reflect)]
pub struct Select;

impl CompoundTask for Select {
    fn register_decompose(commands: &mut Commands) -> DecomposeId {
        commands.register_system(decompose_select)
    }
}

fn decompose_select(
    In(mut ctx): In<DecomposeInput>,
    world: &mut World,
    mut task_relations: Local<QueryState<&Tasks<Sequence>>>,
    mut individual_tasks: Local<
        QueryState<(
            Entity,
            AnyOf<(&Operator, &TypeErasedCompoundTask)>,
            &Conditions,
            &Effects,
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
                    condition_relations.clone(),
                    effect_relations.clone(),
                )
            },
        )
        .collect();

    let mut found_anything = false;
    for (task_entity, operator, compound_task, condition_relations, effect_relations) in
        individual_tasks
    {
        if !conditions
            .iter_many(world, condition_relations.iter())
            .all(|c| c.is_fullfilled(&mut ctx.world_state))
        {
            continue;
        }
        if let Some(operator) = operator {
            ctx.plan.push(operator.system_id());
        } else if let Some(compound_task) = compound_task {
            match world.run_system_with(
                compound_task.decompose,
                DecomposeInput {
                    root: ctx.root,
                    compound_task: task_entity,
                    world_state: ctx.world_state.clone(),
                    plan: ctx.plan.clone(),
                },
            ) {
                Ok(DecomposeResult::Success { plan, world_state }) => {
                    ctx.plan = plan;
                    ctx.world_state = world_state;
                }
                Ok(DecomposeResult::Failure) => continue,
                Ok(DecomposeResult::Rejection) => todo!(),
                Err(_) => continue,
            }
        } else {
            unreachable!()
        }
        for effect in effects.iter_many(world, effect_relations.iter()) {
            effect.apply(&mut ctx.world_state)
        }
        // only use the first match
        found_anything = true;
        break;
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
