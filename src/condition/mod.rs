use std::ops::{Range, RangeBounds};

use ustr::Ustr;

use crate::prelude::*;

pub mod builtin;
pub mod relationship;

#[derive(Component)]
pub struct Condition {
    predicate: Box<dyn Fn(&Props) -> bool + Send + Sync + 'static>,
}

impl Condition {
    pub fn new(predicate: impl Fn(&Props) -> bool + Send + Sync + 'static) -> Self {
        Self {
            predicate: Box::new(predicate),
        }
    }

    pub fn eq(name: impl Into<Ustr>, value: impl Into<Value>) -> Self {
        Self::predicate(name, value, |a, b| a == b)
    }

    pub fn neq(name: impl Into<Ustr>, value: impl Into<Value>) -> Self {
        Self::predicate(name, value, |a, b| a != b)
    }

    pub fn gt(name: impl Into<Ustr>, value: impl Into<Value>) -> Self {
        Self::predicate(name, value, |a, b| a > b)
    }

    pub fn gte(name: impl Into<Ustr>, value: impl Into<Value>) -> Self {
        Self::predicate(name, value, |a, b| a >= b)
    }

    pub fn lt(name: impl Into<Ustr>, value: impl Into<Value>) -> Self {
        Self::predicate(name, value, |a, b| a < b)
    }

    pub fn lte(name: impl Into<Ustr>, value: impl Into<Value>) -> Self {
        Self::predicate(name, value, |a, b| a <= b)
    }

    pub fn in_range<T: PartialOrd + Into<Value>>(
        name: impl Into<Ustr>,
        range: impl RangeBounds<T>,
    ) -> Self {
        Self::new(|props| range.contains(props.get_value(name).num()))
    }

    pub fn predicate(
        name: impl Into<Ustr>,
        value: impl Into<Value>,
        predicate: impl Fn(Option<Value>, Option<Value>) -> bool + Send + Sync + 'static,
    ) -> Self {
        let name = name.into();
        let value = value.into();
        Self::new(move |props| predicate(props.get_value(name), Some(value)))
    }
}
