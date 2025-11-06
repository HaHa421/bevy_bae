use ustr::Ustr;

use crate::prelude::*;

pub mod relationship;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Effect {
    #[reflect(ignore, default = "Effect::noop")]
    effect: Box<dyn FnMut(&mut Props) + Send + Sync + 'static>,
}

impl Effect {
    pub fn new(predicate: impl FnMut(&mut Props) + Send + Sync + 'static) -> Self {
        Self {
            effect: Box::new(predicate),
        }
    }

    pub fn set(name: impl Into<Ustr>, value: impl Into<Value>) -> Self {
        let name = name.into();
        let value = value.into();
        Self::new(move |props| props.set(name, value))
    }

    pub fn toggle(name: impl Into<Ustr>) -> Self {
        let name = name.into();

        Self::new(move |props| props.set(name, !props[name].bool()))
    }

    pub fn inc<T: Into<Value> + Default>(
        name: impl Into<Ustr>,
        value: impl Into<Value> + Default,
    ) -> Self {
        Self::mutate(name, value, |a, b| a += b)
    }

    pub fn dec<T>(name: impl Into<Ustr>, value: impl Into<Value> + Default) -> Self {
        Self::mutate(name, value, |a, b| a -= b)
    }

    pub fn mul(name: impl Into<Ustr>, value: impl Into<Value> + Default) -> Self {
        Self::mutate(name, value, |a, b| a *= b)
    }

    pub fn div(name: impl Into<Ustr>, value: impl Into<Value> + Default) -> Self {
        Self::mutate(name, value, |a, b| a /= b)
    }

    pub fn pow(name: impl Into<Ustr>, value: impl Into<Value> + Default) -> Self {
        Self::mutate(name, value, |a, b| *a = a.pow(b))
    }

    pub fn rem(name: impl Into<Ustr>, value: impl Into<Value> + Default) -> Self {
        Self::mutate(name, value, |a, b| a %= b)
    }

    pub fn mutate(
        name: impl Into<Ustr>,
        value: impl Into<Value> + Default,
        mutate: impl Fn(&mut Value, Value) -> bool + 'static,
    ) -> Self {
        let name = name.into();
        let value = value.into();
        Self::new(move |props| mutate(props.entry(name).or_insert(Default::default), value))
    }

    fn noop() -> Box<dyn FnMut(&mut Props) + Send + Sync + 'static> {
        Box::new(|_| {})
    }
}
