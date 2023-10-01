use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;
// use std::sync::Mutex;
use std::sync::RwLock;
use std::option::Option;

use crate::device::Device;

// Комната
#[derive(Debug)]
pub struct SmartRoom {
    name: String,
    devices: HashMap<String, Arc<RwLock<Device>>>,
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
    pub(crate) fn new(name: String) -> Self {
        SmartRoom {
            name,
            devices: HashMap::new(),
        }
    }
    pub(crate) fn name(&self) -> &str {
        &self.name
    }
    pub(crate) fn devices(&self) -> &HashMap<String, Arc<RwLock<Device>>> {
        &self.devices
    }

    pub(crate) fn get(&self, devices: Option<Vec<String>>) -> Vec<Arc<RwLock<Device>>>{
        if devices.is_some() {
            self.devices
                .iter()
                .filter_map(|(key, val)| {
                    if devices
                        .clone()
                        .unwrap()
                        .contains(key)
                    {
                        Some(
                            val
                                .to_owned()
                                .clone()
                        )
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        } else {
            self
                .devices
                .values()
                .map(|val|{
                    val
                        .to_owned()
                        .clone()
                })
                .collect::<Vec<_>>()
        }
    }

    pub(crate) fn add_device(&mut self, device: Arc<RwLock<Device>>, name: Option<String>) -> fmt::Result {

        self.devices
            .insert(name
                .unwrap_or(
                    device
                    .clone()
                    .read()
                    .unwrap()
                    .name
                    .to_string()
                ),
                device
            );
        Ok(())
    }

    pub(crate) fn remove_device(&mut self, name: String) -> fmt::Result {
        self.devices.remove(&name);
        Ok(())
    }

    pub(crate) fn report(&self, devices: Option<Vec<String>>) -> String {
        let mut result = format!("Name: {},\n", self.name);
        result += "Devices:\n[\n";
        for device in self.get(devices) {
            result += "{\n";
            result += &device.read().unwrap().to_string();
            result += "\n},\n";
        }
        result += "]";
        result
    }
}

// #[cfg(test)]
// mod tests {
//     #[warn(unused_imports)]
//     use super::*;

//     use tonic::codegen::Arc;

//     #[warn(unused_imports)]
//     use crate::device::outlet::SmartOutlet;
//     #[warn(unused_imports)]
//     use crate::device::Device;
//     // #[warn(unused_imports)]
//     // use device::thermometer::SmartThermometer;

//     #[test]
//     fn get_null() {
//         let test_room = SmartRoom::new("test_room".to_string());
//         assert_eq!(test_room.get(None).len(), 0);
//     }

//     #[test]
//     fn add_device() {
//         let test_outlet = Arc::new(
//                 SmartOutlet::new(
//                     "test".to_string(),
//                     None
//                 )
//         );

//         let test_dev = Device::new("test_device".to_string(), test_outlet, None);
//         let mut test_room = SmartRoom::new("test_room".to_string());

//         assert_eq!(test_room.add_device(test_dev.clone(), None), Ok(()));
//         assert_eq!(test_room.get(None).len(), 1);

//         let result = test_room.get(Some(vec!["test_device".to_string()]))[0];

//         assert_eq!(result.name(), "test_device".to_string());
//     }

//     #[test]
//     fn remove_device() {
//         let test_outlet = Arc::new(
//                 SmartOutlet::new(
//                     "test".to_string(),
//                     None
//                 )
//         );
//         let test_dev = Device::new("test_device".to_string(), test_outlet, None);
//         let mut test_room = SmartRoom::new("test_room".to_string());

//         assert_eq!(test_room.add_device(test_dev, None), Ok(()));
//         assert_eq!(test_room.get(None).len(), 1);

//         assert_eq!(test_room.remove_device("test_device".to_string()), Ok(()));
//         assert_eq!(test_room.get(None).len(), 0);
//     }

//     #[test]
//     fn report() {
//         let test_outlet = Arc::new(
//                 SmartOutlet::new(
//                     "test".to_string(),
//                     None
//                 )
//         );
//         let test_dev = Device::new("test_device".to_string(), test_outlet, None);
//         let mut test_room = SmartRoom::new("test_room".to_string());

//         assert_eq!(test_room.add_device(test_dev, None), Ok(()));

//         let report = test_room.report(None);

//         print!("{}\n", report);

//         assert_eq!(report,"Name: test_room,\nDevices:\n[\n{\nName: test_device,\nOn: false,\nDescription: test,\nPower: 0\n},\n]".to_string());
//     }
// }
