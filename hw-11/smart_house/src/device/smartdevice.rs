use std::fmt::{self};

use anyhow::{Error, Result};

pub trait SmartDevices: fmt::Debug + fmt::Display {
    fn listening(&mut self, _data: Vec<u8>) -> Result<(), Error> {
        panic!("trait listening is unrealized")
    }
}
