use alloc::slice;
use bevy_ecs::relationship::{RelatedSpawner, RelatedSpawnerCommands};
use core::iter::Copied;

use crate::{prelude::*, task::validation::BaeTaskPresent};

#[derive(Component, Deref, Reflect, Debug, PartialEq, Eq, Clone)]
#[relationship(relationship_target = Tasks)]
#[reflect(Component)]
pub struct TaskOf(pub Entity);

#[derive(Component, Clone, Deref, Reflect, Debug, Default, PartialEq, Eq)]
#[relationship_target(relationship = TaskOf, linked_spawn)]
#[reflect(Component)]
#[require(BaeTaskPresent)]
pub struct Tasks(Vec<Entity>);

impl<'a> IntoIterator for &'a Tasks {
    type Item = Entity;
    type IntoIter = Copied<slice::Iter<'a, Entity>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub type TaskSpawner<'w> = RelatedSpawner<'w, TaskOf>;

pub type TaskSpawnerCommands<'w> = RelatedSpawnerCommands<'w, TaskOf>;

#[macro_export]
macro_rules! tasks {
    [$($task:expr),*$(,)?] => {
        ::bevy::prelude::related!($crate::prelude::Tasks[$($task),*])
    };
}

pub use tasks;
