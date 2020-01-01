use crate::traits::*;
use std::fmt;

pub mod dms;
pub mod radian;

use dms::DMS;
use radian::Radian;

#[derive(Debug, PartialEq)]
pub enum AngularUnit {
    DMS, // Degree, Minut, Second
    HMS, // Hour Minut, Second
    Radian,
}

impl fmt::Display for AngularUnit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let enum_label: &str = match self {
            AngularUnit::DMS => "dms",
            AngularUnit::HMS => "hms",
            AngularUnit::Radian => "rad",
        };

        write!(f, "{}", enum_label)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::unit_test_helpers::*;
}
