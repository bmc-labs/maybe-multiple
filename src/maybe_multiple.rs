// Copyright 2024 bmc::labs GmbH. All rights reserved.

use crate::Multiple;

#[derive(Debug)]
pub enum MaybeMultiple<T> {
    None,
    Some(T),
    Multiple(Multiple<T>),
}

impl<T> MaybeMultiple<T> {
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    pub fn is_some(&self) -> bool {
        matches!(self, Self::Some(_))
    }
}

impl<T> Default for MaybeMultiple<T> {
    fn default() -> Self {
        Self::None
    }
}

impl<T> From<T> for MaybeMultiple<T> {
    fn from(t: T) -> Self {
        Self::Some(t)
    }
}

impl<T> From<Vec<T>> for MaybeMultiple<T> {
    fn from(mut v: Vec<T>) -> Self {
        match v.len() {
            0 => Self::None,
            1 => Self::Some(v.pop().expect("input vec has one element")),
            _ => Self::Multiple(v.try_into().expect("input vec has more than one element")),
        }
    }
}

impl<T> From<MaybeMultiple<T>> for Vec<T> {
    fn from(maybe_multiple: MaybeMultiple<T>) -> Self {
        match maybe_multiple {
            MaybeMultiple::None => vec![],
            MaybeMultiple::Some(v) => vec![v],
            MaybeMultiple::Multiple(m) => m.into(),
        }
    }
}
