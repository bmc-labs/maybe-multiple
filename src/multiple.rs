// Copyright 2024 bmc::labs GmbH. All rights reserved.

#[cfg(feature = "serde")]
use serde::ser::SerializeSeq;

use thiserror::Error;

#[derive(Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(try_from = "Vec<T>")
)]
pub struct Multiple<T> {
    head: [T; 2],
    tail: Vec<T>,
}

////////////////////////////////////////////////////////////////////////////////
// Type implementations
////////////////////////////////////////////////////////////////////////////////

impl<T> Multiple<T> {
    #[allow(clippy::len_without_is_empty)] // this struct can never be empty by definition
    pub fn len(&self) -> usize {
        self.tail.len() + 2
    }
}

////////////////////////////////////////////////////////////////////////////////
// Trait implementations
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq, Eq, Error)]
#[error("could not convert from Vec: Vec contains fewer than 2 elements")]
pub struct VecSizeError;

impl<T> TryFrom<Vec<T>> for Multiple<T> {
    type Error = VecSizeError;

    fn try_from(v: Vec<T>) -> Result<Self, Self::Error> {
        let mut iter = v.into_iter();

        let (Some(a), Some(b)) = (iter.next(), iter.next()) else {
            return Err(VecSizeError);
        };

        Ok(Self {
            head: [a, b],
            tail: iter.collect(),
        })
    }
}

impl<T> From<Multiple<T>> for Vec<T> {
    fn from(m: Multiple<T>) -> Self {
        let mut v = Vec::from(m.head);
        v.extend(m.tail);
        v
    }
}

impl<T> PartialEq for Multiple<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.head == other.head && self.tail == other.tail
    }
}

impl<T> Eq for Multiple<T> where T: Eq {}

#[cfg(feature = "serde")]
impl<T> serde::Serialize for Multiple<T>
where
    T: serde::Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        seq.serialize_element(&self.head[0])?;
        seq.serialize_element(&self.head[1])?;
        for t in &self.tail {
            seq.serialize_element(t)?;
        }
        seq.end()
    }
}

////////////////////////////////////////////////////////////////////////////////
// Tests
////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::{Multiple, VecSizeError};
    use pretty_assertions::assert_eq;
    use proptest::collection::size_range;
    use test_strategy::proptest;

    #[test]
    fn manual_conversions() {
        let vec: Vec<u8> = vec![];
        assert_eq!(Multiple::try_from(vec).unwrap_err(), VecSizeError);

        let vec: Vec<u8> = vec![42];
        assert_eq!(Multiple::try_from(vec).unwrap_err(), VecSizeError);

        let vec: Vec<u8> = vec![42, 43];
        assert_eq!(Vec::from(Multiple::try_from(vec.clone()).unwrap()), vec);

        let vec: Vec<u8> = vec![42, 43, 44];
        assert_eq!(Vec::from(Multiple::try_from(vec.clone()).unwrap()), vec);
    }

    #[proptest]
    fn proptest_conversions(vec: Vec<u8>) {
        if vec.len() < 2 {
            assert_eq!(Multiple::try_from(vec).unwrap_err(), VecSizeError);
        } else {
            assert_eq!(Vec::from(Multiple::try_from(vec.clone()).unwrap()), vec);
        }
    }

    #[proptest]
    fn serialize_deserialize(#[any(size_range(2..128).lift())] vec: Vec<u8>) {
        let multiple = Multiple::try_from(vec.clone()).unwrap();
        assert_eq!(
            serde_json::to_string(&multiple).unwrap(),
            serde_json::to_string(&vec).unwrap()
        );

        let vec_str = serde_json::to_string(&vec).unwrap();
        assert_eq!(
            serde_json::from_str::<Multiple<_>>(&vec_str).unwrap(),
            multiple
        );
    }
}
