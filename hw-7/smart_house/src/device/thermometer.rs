use std::fmt;

use std::option::Option;

use super::SmartDevices;

// Термометр

#[derive(Debug, Clone, PartialEq)]
pub struct SmartThermometer {
    description: String,
    temperature: u8,
}

impl SmartDevices for SmartThermometer {}

impl fmt::Display for SmartThermometer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Description: {},\nTemperature: {}",
            self.description, self.temperature
        )
    }
}

#[allow(dead_code)]
impl SmartThermometer {
    pub(crate) fn new(description: String, temperature: Option<u8>) -> Self {
        SmartThermometer {
            description,
            temperature: temperature.unwrap_or(0),
        }
    }
    pub(crate) fn description(&self) -> &str {
        &self.description
    }
    pub(crate) fn temperature(&self) -> &u8 {
        &self.temperature
    }
}

#[cfg(test)]
mod tests {
    #[warn(unused_imports)]
    use super::*;

    #[test]
    fn new() {
        assert_eq!(
            SmartThermometer::new("test".to_string(), None),
            SmartThermometer {
                description: "test".to_string(),
                temperature: 0,
            }
        );
    }

    #[test]
    fn display() {
        assert_eq!(
            SmartThermometer::new("test".to_string(), None).to_string(),
            "Description: test,\nTemperature: 0"
        );
    }
}
