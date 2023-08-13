use std::fmt;

use std::option::Option;

// Розетка

#[derive(Debug, Clone)]
pub struct SmartOutlet {
    pub description: String,
    pub power: u8,
}

impl fmt::Display for SmartOutlet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "(Description: {}\nPower: {})",
            self.description, self.power
        )
    }
}

#[allow(dead_code)]
impl SmartOutlet {
    fn new(description: String, power: Option<u8>) -> Self {
        SmartOutlet {
            description,
            power: power.unwrap_or(0),
        }
    }
}
