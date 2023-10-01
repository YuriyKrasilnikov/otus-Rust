use std::fmt::{self};

use anyhow::{Result, Error};

pub trait SmartDevices: fmt::Debug + fmt::Display {
    fn listening(&mut self, _data: Vec<u8>) -> Result<(), Error> {
        panic!("trait listening is unrealized")
    }
}