use std::fmt;
use std::option::Option;
use std::collections::HashMap;
use std::iter::FromIterator;

use device::Device;
use device::smart_device::SmartDevice;

// Комната
#[allow(dead_code)]
#[derive(Debug, Clone)]
struct SmartRoom {
    name: String,
    devices: HashMap<String, Device>,
}

#[derive(Debug)]
struct RoomError;

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

    fn report(&self, devices: Option<Vec<String>>) -> fmt::Result {
        println!("Room Name: {}", self.name);
        println!("Devices:");
        for device in self.get(devices) {
            println!("{}", device)
        }
        Ok(())
    }

    fn get_devices_name(&self) -> Vec<&String> {
        Vec::from_iter(self.devices.keys())
    }

    fn add_devices(&mut self, device: Device) -> fmt::Result {
        self.devices
            .insert(
              SmartDevice::get_name(device),
              device
            );
        Ok(())
    }
}