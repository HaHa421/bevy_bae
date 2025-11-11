use bevy_ecs::name::NameOrEntityItem;

pub(crate) trait NameOrEntityExt {
    fn entity_and_name(&self) -> String;
}

impl<'w, 's> NameOrEntityExt for NameOrEntityItem<'w, 's> {
    fn entity_and_name(&self) -> String {
        self.name
            .map(|n| format!("{} ({})", self.entity, n))
            .unwrap_or_else(|| format!("{}", self.entity))
    }
}
