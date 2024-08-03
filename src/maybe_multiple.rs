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
    Single(T),
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
    pub fn is_single(&self) -> bool {
        matches!(self, Self::Single(_))
    }

    #[must_use = "to assert that this has a value, wrap this in an `assert!()` instead"]
    #[inline]
    pub fn is_multiple(&self) -> bool {
        matches!(self, Self::Multiple(_))
    }

    /// Collapses a [`Vec`] into a [`MaybeMultiple`].
    ///
    /// This function collapses an empty [`Vec`] into [`MaybeMultiple::None`], a single element
    /// [`Vec`] into [`MaybeMultiple::Single`], and a multiple element [`Vec`] into
    /// [`MaybeMultiple::Multiple`]. The [`Vec`] is consumed.
    ///
    /// This function is provided in lue of `impl From<Vec<T>> for MaybeMultiple<T>`, which would
    /// arguably be more convenient, but wouldn't make the conversion as explicit as this function
    /// does. The reason why [`MaybeMultiple`] exists is to make this very specific use case very
    /// explicit, so hiding the conversion behind an `.into()` call is not desirable.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use crate::maybe_multiple::MaybeMultiple;
    /// #
    /// let vec: Vec<u8> = vec![];
    /// let maybe_multiple = MaybeMultiple::collapse_vec_into(vec);
    /// assert!(maybe_multiple.is_none());
    ///
    /// let maybe_multiple = MaybeMultiple::collapse_vec_into(vec![42]);
    /// assert!(maybe_multiple.is_single());
    ///
    /// let maybe_multiple = MaybeMultiple::collapse_vec_into(vec![42, 43, 44]);
    /// assert!(maybe_multiple.is_multiple());
    /// ```
    #[inline]
    pub fn collapse_vec_into(mut v: Vec<T>) -> Self {
        match v.len() {
            0 => Self::None,
            1 => Self::Single(v.pop().expect("input vec has one element")),
            _ => Self::Multiple(v.try_into().expect("input vec has more than one element")),
        }
    }

    /// Inverse of [`MaybeMultiple::collapse_vec_into`]. See docs of that function for more info.
    #[inline]
    pub fn expand_into_vec(self) -> Vec<T> {
        match self {
            Self::None => vec![],
            Self::Single(v) => vec![v],
            Self::Multiple(m) => m.into(),
        }
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
        Self::Single(t)
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
            0 => {
                assert_eq!(
                    MaybeMultiple::collapse_vec_into(v.clone()),
                    MaybeMultiple::None
                );
                assert_eq!(MaybeMultiple::<u8>::None.expand_into_vec(), v);
            }
            1 => {
                let e = v[0];
                assert_eq!(
                    MaybeMultiple::collapse_vec_into(v.clone()),
                    MaybeMultiple::Single(e)
                );
                assert_eq!(MaybeMultiple::Single(e).expand_into_vec(), v);
            }
            _ => {
                assert_eq!(
                    MaybeMultiple::collapse_vec_into(v.clone()),
                    MaybeMultiple::Multiple(v.clone().try_into().unwrap())
                );
                assert_eq!(
                    MaybeMultiple::Multiple(v.clone().try_into().unwrap()).expand_into_vec(),
                    v
                );
            }
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
    fn serialize_deserialize_single(val: u8) {
        let maybe_multiple = MaybeMultiple::Single(val);
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
        let maybe_multiple = MaybeMultiple::collapse_vec_into(vec.clone());
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
