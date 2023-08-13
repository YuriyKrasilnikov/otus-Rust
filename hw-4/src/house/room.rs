use std::collections::HashMap;
use std::fmt;
use std::iter::FromIterator;
use std::option::Option;

use device::smart_device::SmartDevice;
use device::{fn_device, Device};

// Комната
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SmartRoom {
    pub(crate) name: String,
    pub(crate) devices: HashMap<String, Device>,
}

#[derive(Debug)]
pub struct RoomError;

impl fmt::Display for RoomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Room not found")
    }
}
impl std::error::Error for RoomError {}

#[allow(dead_code)]
impl SmartRoom {
    fn get(&self, devices: Option<Vec<String>>) -> Vec<&Device> {
        if devices.is_some() {
            self.devices
                .iter()
                .filter_map(|(k, v)| {
                    if devices.clone().unwrap().contains(k) {
                        Some(v)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
                .clone()
        } else {
            self.devices.values().collect::<Vec<_>>().clone()
        }
    }

    pub(crate) fn report(&self, devices: Option<Vec<String>>) -> fmt::Result {
        println!("Room Name: {}", self.name);
        println!("Devices:");
        for device in self.get(devices) {
            println!("{}", device)
        }
        Ok(())
    }

    pub(crate) fn get_devices_name(&self) -> Vec<&String> {
        Vec::from_iter(self.devices.keys())
    }

    pub(crate) fn add_devices(&mut self, device: Device) -> fmt::Result {
        self.devices
            .insert(fn_device!(device, SmartDevice::get_name), device);
        Ok(())
    }
}
