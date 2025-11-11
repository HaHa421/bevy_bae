//! Types needed for the [`tasks!`] macro.

use alloc::slice;
use bevy_ecs::relationship::{RelatedSpawner, RelatedSpawnerCommands};
use core::iter::Copied;

use crate::{prelude::*, task::validation::BaeTaskPresent};

/// Points from an [`Operator`] or [`CompoundTask`] to its associated higher [`CompoundTask`].
#[derive(Component, Deref, Reflect, Debug, PartialEq, Eq, Clone)]
#[relationship(relationship_target = Tasks)]
#[reflect(Component)]
pub struct TaskOf(pub Entity);

/// Relationship target for [`Operator`]s or [`CompoundTask`]s. Created with [`tasks!`].
/// Only valid on [`CompoundTask`]s.
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

/// Shorthand for a [`RelatedSpawner`] for [`TaskOf`] relations.
pub type TaskSpawner<'w> = RelatedSpawner<'w, TaskOf>;

/// Shorthand for a [`RelatedSpawnerCommands`] for [`TaskOf`] relations.
pub type TaskSpawnerCommands<'w> = RelatedSpawnerCommands<'w, TaskOf>;

/// Shorthand for creating a [`Tasks`] relation
#[macro_export]
macro_rules! tasks {
    [$($task:expr),*$(,)?] => {
        ::bevy::prelude::related!($crate::prelude::Tasks[$($task),*])
    };
}

pub use tasks;
