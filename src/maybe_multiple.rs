// Copyright 2024 bmc::labs GmbH. All rights reserved.

use crate::Multiple;

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(untagged)
)]
pub enum MaybeMultiple<T> {
    None,
    Some(T),
    Multiple(Multiple<T>),
}

////////////////////////////////////////////////////////////////////////////////
// Type implementations
////////////////////////////////////////////////////////////////////////////////

impl<T> MaybeMultiple<T> {
    #[must_use = "to assert that this doesn't have a value, wrap this in an `assert!()` instead"]
    #[inline]
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    #[must_use = "to assert that this has a value, wrap this in an `assert!()` instead"]
    #[inline]
    pub fn is_some(&self) -> bool {
        matches!(self, Self::Some(_))
    }

    #[must_use = "to assert that this has a value, wrap this in an `assert!()` instead"]
    #[inline]
    pub fn is_multiple(&self) -> bool {
        matches!(self, Self::Multiple(_))
    }

    #[inline]
    pub fn from_vec(v: Vec<T>) -> Self {
        Self::from(v)
    }
}

////////////////////////////////////////////////////////////////////////////////
// Trait implementations
////////////////////////////////////////////////////////////////////////////////

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

////////////////////////////////////////////////////////////////////////////////
// Tests
////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::MaybeMultiple;
    use pretty_assertions::assert_eq;
    use proptest::collection::size_range;
    use test_strategy::proptest;

    #[proptest]
    fn proptest_conversions(v: Vec<u8>) {
        match v.len() {
            0 => assert_eq!(MaybeMultiple::from_vec(v), MaybeMultiple::None),
            1 => {
                let e = v[0];
                assert_eq!(MaybeMultiple::from(v), MaybeMultiple::Some(e));
            }
            _ => assert_eq!(
                MaybeMultiple::from(v.clone()),
                MaybeMultiple::Multiple(v.try_into().unwrap())
            ),
        }
    }

    #[test]
    fn serialize_deserialize_none() {
        assert_eq!(
            serde_json::to_string(&MaybeMultiple::<u8>::None).unwrap(),
            "null"
        );
        assert_eq!(
            serde_json::from_str::<MaybeMultiple<u8>>("null").unwrap(),
            MaybeMultiple::None
        );
    }

    #[proptest]
    fn serialize_deserialize_some(val: u8) {
        let maybe_multiple = MaybeMultiple::Some(val);
        assert_eq!(
            serde_json::to_string(&maybe_multiple).unwrap(),
            serde_json::to_string(&val).unwrap()
        );

        let val_str = serde_json::to_string(&val).unwrap();
        assert_eq!(
            serde_json::from_str::<MaybeMultiple<_>>(&val_str).unwrap(),
            maybe_multiple
        );
    }

    #[proptest]
    fn serialize_deserialize_multiple(#[any(size_range(2..128).lift())] vec: Vec<u8>) {
        let maybe_multiple = MaybeMultiple::from_vec(vec.clone());
        assert_eq!(
            serde_json::to_string(&maybe_multiple).unwrap(),
            serde_json::to_string(&vec).unwrap()
        );

        let vec_str = serde_json::to_string(&vec).unwrap();
        assert_eq!(
            serde_json::from_str::<MaybeMultiple<_>>(&vec_str).unwrap(),
            maybe_multiple
        );
    }
}
