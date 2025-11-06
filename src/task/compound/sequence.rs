use crate::{
    prelude::*,
    task::{compound::TypeErasedCompoundTask, primitive::OperatorId},
};

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct Sequence;

impl CompoundTask for Sequence {
    fn decompose(
        entity: Entity,
        world: &World,
        props: &mut Props,
        tasks: &mut std::collections::VecDeque<OperatorId>,
    ) {
        let Some(mut children) = world.try_query::<&Children>() else {
            warn!("Children was not registered as a component");
            return;
        };
        let Some(mut conditions) = world.try_query::<&Condition>() else {
            warn!("Conditions was not registered as a component");
            return;
        };
        let Some(mut effects) = world.try_query::<&Effect>() else {
            warn!("Effects was not registered as a component");
            return;
        };
        if let Some(condition_relations) = world.get::<Conditions>(entity) {
            for condition in conditions.iter_many(world, condition_relations) {
                if !condition.is_fullfilled(props) {
                    return;
                }
            }
        }
        for child in children.get(world, entity).unwrap().iter() {
            if let Some(condition_relations) = world.get::<Conditions>(child) {
                for condition in conditions.iter_many(world, condition_relations) {
                    if !condition.is_fullfilled(props) {
                        continue;
                    }
                }
            }

            if let Some(operator) = world.get::<Operator>(child) {
                tasks.push_back(operator.system_id());
            }
            if let Some(compound) = world.get::<TypeErasedCompoundTask>(child) {
                (compound.decompose)(child, world, props, tasks);
            }
            if let Some(effects_relations) = world.get::<Effects>(child) {
                for effect in effects.iter_many(world, effects_relations) {
                    effect.apply(props);
                }
            }
        }
    }
}
