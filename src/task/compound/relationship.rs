use crate::prelude::*;
use crate::task::BaeTask;
use alloc::slice;
use bevy_ecs::{
    relationship::{RelatedSpawner, RelatedSpawnerCommands},
    spawn::{SpawnRelatedBundle, SpawnableList},
};
use core::{fmt::Debug, iter::Copied, marker::PhantomData};

#[derive(Component, Deref, Reflect, Debug, PartialEq, Eq, Clone)]
#[relationship(relationship_target = BaeTasks<T>)]
#[reflect(Component)]
pub struct BaeTaskOf<T: CompoundTask> {
    #[deref]
    #[relationship]
    entity: Entity,
    #[reflect(ignore)]
    marker: PhantomData<T>,
}

#[derive(Component, Deref, Reflect, Debug, Default, PartialEq, Eq)]
#[relationship_target(relationship = BaeTaskOf<T>, linked_spawn)]
#[reflect(Component)]
pub struct BaeTasks<T: CompoundTask> {
    #[deref]
    #[relationship]
    entities: Vec<Entity>,
    #[reflect(ignore)]
    marker: PhantomData<T>,
}

impl<'a, T: CompoundTask> IntoIterator for &'a BaeTasks<T> {
    type Item = Entity;
    type IntoIter = Copied<slice::Iter<'a, Entity>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<S: SpawnableList<BaeTaskOf<T>> + Send + Sync + 'static, T: CompoundTask> BaeTask
    for SpawnRelatedBundle<BaeTaskOf<T>, S>
{
}

pub type BaeTaskSpawner<'w, T> = RelatedSpawner<'w, BaeTaskOf<T>>;
pub type BaeTaskSpawnerCommands<'w, T> = RelatedSpawnerCommands<'w, BaeTaskOf<T>>;

#[macro_export]
macro_rules! tasks {
    ($compound:ty[$($condition:expr),*$(,)?]) => {
        ::bevy::prelude::related!($crate::prelude::BaeTasks<$compound>[$($crate::prelude::IntoTaskBundle::into_task_bundle($condition)),*])
    };
}

pub use tasks;

#[diagnostic::on_unimplemented(
    message = "`{Self}` is not a valid task bundle. The first element must be either an `Operator` or a component that implements `CompositeTask`, like `Select` or `Sequence`.",
    label = "invalid task bundle"
)]
pub trait IntoTaskBundle {
    fn into_task_bundle(self) -> impl Bundle;
}

impl<B: BaeTask> IntoTaskBundle for B {
    fn into_task_bundle(self) -> impl Bundle {
        self
    }
}

macro_rules! impl_into_task_bundle {
    ($($C:ident),*) => {
        impl<B: BaeTask, $($C: Bundle,)*> IntoTaskBundle for (B, $($C),*) {
            #[allow(non_snake_case, reason = "tuple unpack")]
            fn into_task_bundle(self) -> impl Bundle {
                let (b, $($C),* ) = self;
                (b, $($C),*)
            }
        }
    }
}

variadics_please::all_tuples!(impl_into_task_bundle, 0, 14, C);
