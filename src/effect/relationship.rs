//! Types needed for the [`effects`] macro.

use alloc::slice;
use bevy_ecs::relationship::{RelatedSpawner, RelatedSpawnerCommands};
use core::iter::Copied;

use crate::prelude::*;

/// Points from an [`Effect`] to its associated [`Operator`]
#[derive(Component, Deref, Reflect, Debug, PartialEq, Eq, Clone)]
#[relationship(relationship_target = Effects)]
#[reflect(Component)]
pub struct EffectOf(pub Entity);

/// Relationship target for [`Effect`]s. Created with [`effects!`].
/// Only valid on [`Operator`]s.
#[derive(Component, Clone, Deref, Reflect, Debug, Default, PartialEq, Eq)]
#[relationship_target(relationship = EffectOf, linked_spawn)]
#[reflect(Component)]
pub struct Effects(Vec<Entity>);

impl<'a> IntoIterator for &'a Effects {
    type Item = Entity;
    type IntoIter = Copied<slice::Iter<'a, Entity>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// Shorthand for a [`RelatedSpawner`] for [`EffectOf`] relations.
pub type EffectSpawner<'w> = RelatedSpawner<'w, EffectOf>;

/// Shorthand for a [`RelatedSpawnerCommands`] for [`EffectOf`] relations.
pub type EffectSpawnerCommands<'w> = RelatedSpawnerCommands<'w, EffectOf>;

/// Shorthand for creating an [`Effects`] relation
#[macro_export]
macro_rules! effects {
    [$($effect:expr),*$(,)?] => {
        ::bevy::prelude::related!($crate::prelude::Effects[$($effect),*])
    };
}

pub use effects;
