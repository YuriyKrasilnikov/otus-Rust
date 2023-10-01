use std::fmt;

use std::option::Option;

use anyhow::{anyhow, Result, Error};

use super::SmartDevices;

use device_quic::common::make_client_endpoint;

// Термометр

#[derive(Debug, Clone, PartialEq)]
pub struct SmartThermometer {
    description: String,
    temperature: i8,
}


impl SmartDevices for SmartThermometer {
    fn listening(&mut self, data: Vec<u8>) -> Result<(), Error> {

        let bytes: [u8; 1] = data
            .try_into()
            .map_err(
            |v: Vec<u8>| anyhow!("Expected a Vec of length {} but it was {}", 1, v.len())
            )?;

        self.temperature = i8::from_ne_bytes(bytes);
        Ok(())
    }
}

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
    pub(crate) fn new(description: String, temperature: Option<i8>) -> Self {
        SmartThermometer {
            description,
            temperature: temperature.unwrap_or(0),
        }
    }
    pub(crate) fn description(&self) -> &str {
        &self.description
    }
    pub(crate) fn temperature(&self) -> &i8 {
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
