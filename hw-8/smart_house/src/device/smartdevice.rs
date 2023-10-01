use std::fmt::{self};

use anyhow::{Result, Error};

use async_trait::async_trait;


#[async_trait]
pub trait SmartDevices: fmt::Debug + fmt::Display {
    async fn listening(&mut self, _: &str, _: &str, _: &str, _: &Vec<u8>) -> Result<(), Error> {
        panic!("trait is unrealized")
    }
}