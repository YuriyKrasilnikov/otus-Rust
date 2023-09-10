use std::fmt;

use std::option::Option;

use super::dyn_partial_eq::DynPartialEq;
use super::SmartDevices;

// Розетка

#[derive(Debug, Clone, DynPartialEq, PartialEq)]
pub struct SmartOutlet {
    description: String,
    power: u8,
}

impl SmartDevices for SmartOutlet {}

impl fmt::Display for SmartOutlet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Description: {},\nPower: {}",
            self.description, self.power
        )
    }
}

#[allow(dead_code)]
impl SmartOutlet {
    pub(crate) fn new(description: String, power: Option<u8>) -> Self {
        SmartOutlet {
            description,
            power: power.unwrap_or(0),
        }
    }
    pub(crate) fn description(&self) -> &str {
        &self.description
    }
    pub(crate) fn power(&self) -> &u8 {
        &self.power
    }
}

#[cfg(test)]
mod tests {
    #[warn(unused_imports)]
    use super::*;

    #[test]
    fn new() {
        assert_eq!(
            SmartOutlet::new("test".to_string(), None),
            SmartOutlet {
                description: "test".to_string(),
                power: 0,
            }
        );
    }

    #[test]
    fn display() {
        assert_eq!(
            SmartOutlet::new("test".to_string(), None).to_string(),
            "Description: test,\nPower: 0"
        );
    }
}
