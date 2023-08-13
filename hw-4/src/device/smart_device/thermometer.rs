use std::fmt;

use std::option::Option;

// Термометр

#[derive(Debug, Clone)]
pub struct SmartThermometer {
    pub description: String,
    pub temperature: u8,
}

impl fmt::Display for SmartThermometer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "(Description: {}\nTemperature: {})",
            self.description, self.temperature
        )
    }
}

#[allow(dead_code)]
impl SmartThermometer {
    fn new(description: String, temperature: Option<u8>) -> Self {
        SmartThermometer {
            description,
            temperature: temperature.unwrap_or(0),
        }
    }
}
