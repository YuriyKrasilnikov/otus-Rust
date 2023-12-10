use std::collections::HashMap;
use std::option::Option;

use crate::device::RwLockDevice;

use thiserror::Error;
#[derive(Debug, Error)]
pub enum RoomError {
    // #[error("Cannot get {devices:?}")]
    // GetError{
    //     devices: Option<Vec<String>>
    // },
    #[error("Cannot add the device named {name:?}")]
    AddError { name: String },
    #[error("Cannot remove the device named {name:?}")]
    RemoveError { name: String },
}

// Комната
#[derive(Debug)]
pub struct SmartRoom {
    name: String,
    devices: HashMap<String, RwLockDevice>,
}

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
    pub(crate) fn devices(&self) -> &HashMap<String, RwLockDevice> {
        &self.devices
    }

    pub(crate) fn get(&self, devices: Option<Vec<String>>) -> Vec<RwLockDevice> {
        if devices.is_some() {
            self.devices
                .iter()
                .filter_map(|(key, val)| {
                    if devices.clone().unwrap().contains(key) {
                        Some(val.to_owned().clone())
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        } else {
            self.devices
                .values()
                .map(|val| val.to_owned().clone())
                .collect::<Vec<_>>()
        }
    }

    pub(crate) fn add_device(
        &mut self,
        device: RwLockDevice,
        name: Option<String>,
    ) -> Result<String, RoomError> {
        let dev_name = name.unwrap_or(device.clone().read().unwrap().name.to_string());
        self.devices.insert(dev_name.clone(), device);
        Ok(dev_name)
    }

    pub(crate) fn remove_device(&mut self, name: String) -> Result<String, RoomError> {
        self.devices
            .remove(&name)
            .ok_or(RoomError::RemoveError { name: name.clone() })?;
        Ok(name)
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

#[cfg(test)]
mod tests {
    #[warn(unused_imports)]
    use super::*;

    use std::sync::RwLock;
    use tonic::codegen::Arc;

    #[warn(unused_imports)]
    use crate::device::outlet::SmartOutlet;
    #[warn(unused_imports)]
    use crate::device::Device;
    // #[warn(unused_imports)]
    // use device::thermometer::SmartThermometer;

    #[test]
    fn get_null() {
        let test_room = SmartRoom::new("test_room".to_string());
        assert_eq!(test_room.get(None).len(), 0);
    }

    #[test]
    fn add_device() {
        let test_outlet = Arc::new(RwLock::new(SmartOutlet::new(
            "test_outlet".to_string(),
            None,
        )));

        let test_dev = RwLockDevice::new(Arc::new(RwLock::new(Device::new(
            "test_device".to_string(),
            test_outlet,
            None,
        ))));
        let mut test_room = SmartRoom::new("test_room".to_string());

        assert_eq!(
            test_room.add_device(test_dev.clone(), None).unwrap(),
            "test_device".to_string()
        );
        assert_eq!(test_room.get(None).len(), 1);

        let result = &test_room.get(Some(vec!["test_device".to_string()]))[0];

        assert_eq!(result.read().unwrap().name(), "test_device".to_string());
    }

    #[test]
    fn remove_device() {
        let test_outlet = Arc::new(RwLock::new(SmartOutlet::new(
            "test_outlet".to_string(),
            None,
        )));
        let test_dev = RwLockDevice::new(Arc::new(RwLock::new(Device::new(
            "test_device".to_string(),
            test_outlet,
            None,
        ))));
        let mut test_room = SmartRoom::new("test_room".to_string());

        assert_eq!(
            test_room.add_device(test_dev, None).unwrap(),
            "test_device".to_string()
        );
        assert_eq!(test_room.get(None).len(), 1);

        assert_eq!(
            test_room.remove_device("test_device".to_string()).unwrap(),
            "test_device".to_string()
        );
        assert_eq!(test_room.get(None).len(), 0);
    }

    #[test]
    fn report() {
        let test_outlet = Arc::new(RwLock::new(SmartOutlet::new(
            "test_outlet".to_string(),
            None,
        )));
        let test_dev = RwLockDevice::new(Arc::new(RwLock::new(Device::new(
            "test_device".to_string(),
            test_outlet,
            None,
        ))));
        let mut test_room = SmartRoom::new("test_room".to_string());

        assert_eq!(
            test_room.add_device(test_dev, None).unwrap(),
            "test_device".to_string()
        );

        let report = test_room.report(None);

        // print!("{}\n", report);

        assert_eq!(
            report,
            "Name: test_room,\nDevices:\n[\n{\nName: test_device,\nOn: false,\nDescription: test_outlet,\nPower: 0\n},\n]".to_string()
        );
    }
}
