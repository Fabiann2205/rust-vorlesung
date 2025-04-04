use std::fmt::Display;

use serde::{de::Visitor, Deserialize, Serialize};

#[derive(Debug)]
/// Error creating BSN
// TODO: update the enum to make it more descriptive
// as there can be several reasons for a BSN to not be valid
pub enum Error {
    /// The BSN was invalid
    InvalidBsn,
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidBsn => write!(f, "Invalid BSN number"),
        }
    }
}

/// A valid BSN (burgerservicenummer), a Dutch
/// personal identification number that is similar
/// to the US Social Security Number.
/// More info (Dutch): https://www.rvig.nl/bsn
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Bsn {
    inner: String,
}

impl Bsn {
    pub fn try_from_string<B: ToString>(bsn: B) -> Result<Self, Error> {
        let bsn_str = bsn.to_string();
        Self::validate(&bsn_str)?;
        Ok(Bsn { inner: bsn_str })
    }

    pub fn validate(bsn: &str) -> Result<(), Error> {
        if bsn.len() != 9 {
            return Err(Error::InvalidBsn);
        }
        let digits = bsn.chars().map(|c| c.to_digit(10).unwrap());
        let sum: i32 = digits.enumerate().map(|(i, d)| if i == 8 { d as i32 } else { (9 - i as i32) * d as i32 }).sum();
        if sum % 11 == 0 {
            Ok(())
        } else {
            Err(Error::InvalidBsn)
        }
    }
}

impl Serialize for Bsn {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.inner)
    }
}

impl<'de> Deserialize<'de> for Bsn {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct BsnVisitor;

        impl<'d> Visitor<'d> for BsnVisitor {
            type Value = Bsn;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "A string representing a valid BSN")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Bsn::try_from_string(value).map_err(E::custom)
            }
        }

        deserializer.deserialize_str(BsnVisitor)
    }
}

#[cfg(test)]
mod tests {
    use crate::Bsn;

    #[test]
    fn test_validation() {
        let bsns = include_str!("../valid_bsns.in").lines();
        bsns.for_each(|bsn| assert!(Bsn::validate(bsn).is_ok(), "BSN {bsn} is valid, but did not pass validation"));

        let bsns = include_str!("../invalid_bsns.in").lines();
        bsns.for_each(|bsn| assert!(Bsn::validate(bsn).is_err(), "BSN {bsn} invalid, but passed validation"));
    }

    #[test]
    fn test_serde() {
        let json = serde_json::to_string(&Bsn::try_from_string("999998456").unwrap()).unwrap();
        assert_eq!(json, "\"999998456\"");
        let bsn: Bsn = serde_json::from_str("\"999998456\"").unwrap();
        assert_eq!(bsn, Bsn::try_from_string("999998456".to_string()).unwrap());

        serde_json::from_str::<Bsn>("\"1112223333\"").unwrap_err();
    }
}
