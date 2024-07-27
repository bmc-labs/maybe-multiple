// Copyright 2024 bmc::labs GmbH. All rights reserved.

use thiserror::Error;

#[derive(Debug, PartialEq, Eq, Error)]
#[error("could not convert from Vec: Vec contains fewer than 2 elements")]
pub struct VecSizeError;

#[derive(Debug)]
pub struct Multiple<T> {
    head: [T; 2],
    tail: Vec<T>,
}

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

#[cfg(test)]
mod tests {
    use super::{Multiple, VecSizeError};
    use pretty_assertions::assert_eq;
    use test_strategy::proptest;

    #[test]
    fn manual_conversions() {
        let v: Vec<u8> = vec![];
        assert_eq!(Multiple::try_from(v).unwrap_err(), VecSizeError);

        let v: Vec<u8> = vec![42];
        assert_eq!(Multiple::try_from(v).unwrap_err(), VecSizeError);

        let v: Vec<u8> = vec![42, 43];
        assert_eq!(Vec::from(Multiple::try_from(v.clone()).unwrap()), v);

        let v: Vec<u8> = vec![42, 43, 44];
        assert_eq!(Vec::from(Multiple::try_from(v.clone()).unwrap()), v);
    }

    #[proptest]
    fn proptest_conversions(v: Vec<u8>) {
        if v.len() < 2 {
            assert_eq!(Multiple::try_from(v).unwrap_err(), VecSizeError);
        } else {
            assert_eq!(Vec::from(Multiple::try_from(v.clone()).unwrap()), v);
        }
    }
}
