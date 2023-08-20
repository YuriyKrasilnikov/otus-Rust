use std::collections::HashMap;
use std::fmt;
use std::option::Option;

use device::Device;

// Комната
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SmartRoom {
    name: String,
    devices: HashMap<String, Device>,
}

#[derive(Debug, PartialEq)]
pub struct RoomError;

impl fmt::Display for RoomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Room not found")
    }
}
impl std::error::Error for RoomError {}

#[allow(dead_code)]
impl SmartRoom {
    pub(crate) fn new(name: String) -> Self {
        SmartRoom {
            name,
            devices: HashMap::new(),
        }
    }
    pub(crate) fn name(&self) -> &str {
        &self.name
    }
    pub(crate) fn devices(&self) -> &HashMap<String, Device> {
        &self.devices
    }

    pub(crate) fn get(&self, devices: Option<Vec<String>>) -> Vec<&Device> {
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

    pub(crate) fn report(&self, devices: Option<Vec<String>>) -> String {
        let mut result = format!("Name: {},\n", self.name);
        result += "Devices:\n[\n";
        for device in self.get(devices) {
            result += "{\n";
            result += &device.to_string();
            result += "\n},\n";
        }
        result += "]";
        result
    }

    pub(crate) fn add_device(&mut self, device: Device) -> fmt::Result {
        self.devices.insert(device.name().to_string(), device);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[warn(unused_imports)]
    use super::*;

    #[warn(unused_imports)]
    use device::outlet::SmartOutlet;
    #[warn(unused_imports)]
    use device::Device;
    // #[warn(unused_imports)]
    // use device::thermometer::SmartThermometer;

    #[test]
    fn test_get_null() {
        let test_room = SmartRoom::new("test_room".to_string());
        assert_eq!(test_room.get(None).len(), 0);
    }

    #[test]
    fn test_add_device() {
        let test_outlet = Box::new(SmartOutlet::new("test".to_string(), None));
        let test_dev = Device::new("test_device".to_string(), test_outlet, None);
        let mut test_room = SmartRoom::new("test_room".to_string());

        assert_eq!(test_room.add_device(test_dev.clone()), Ok(()));
        assert_eq!(test_room.get(None).len(), 1);
        assert_eq!(
            test_room.get(Some(vec!["test_device".to_string()])),
            vec![&test_dev]
        );
    }

    #[test]
    fn test_report() {
        let test_outlet = Box::new(SmartOutlet::new("test".to_string(), None));
        let test_dev = Device::new("test_device".to_string(), test_outlet, None);
        let mut test_room = SmartRoom::new("test_room".to_string());

        assert_eq!(test_room.add_device(test_dev.clone()), Ok(()));

        let report = test_room.report(None);

        print!("{}\n", report);

        assert_eq!(report,"Name: test_room,\nDevices:\n[\n{\nName: test_device,\nOn: false,\nDescription: test,\nPower: 0\n},\n]".to_string());
    }
}
