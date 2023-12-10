pub(crate) mod room;

// use std::sync::Arc;
// use std::sync::Mutex;
// use std::sync::RwLock;
use std::collections::HashMap;
use std::option::Option;

extern crate uuid;
use self::uuid::Uuid;

use self::room::RoomError;
use self::room::SmartRoom;

use crate::device::RwLockDevice;

// Умный дом

// #[derive(Clone)]
pub struct SmartHouse {
    id: Uuid,
    name: String,
    rooms: HashMap<String, SmartRoom>,
}

use thiserror::Error;
#[derive(Debug, Error)]
pub enum SmartHouseError {
    #[error("Cannot add the room named {name:?}")]
    AddRoomError { name: String },
    #[error("Cannot remove the room named {name:?}")]
    RemoveRoomError { name: String },
    #[error("Cannot get the room named {name:?}")]
    GetRoomError { name: String },
    #[error(transparent)]
    RoomError(#[from] RoomError),
}

#[allow(dead_code)]
impl SmartHouse {
    pub fn new(name: String) -> Self {
        // инициализация дома
        SmartHouse {
            id: Uuid::new_v4(),
            name,
            rooms: HashMap::new(),
        }
    }
    pub fn id(&self) -> &Uuid {
        &self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn rooms(&self) -> &HashMap<String, SmartRoom> {
        &self.rooms
    }

    pub fn add_room(&mut self, name: String) -> Result<String, SmartHouseError> {
        self.rooms
            .insert(name.clone(), SmartRoom::new(name.clone()));
        Ok(name)
    }

    pub fn remove_room(&mut self, name: String) -> Result<String, SmartHouseError> {
        self.rooms
            .remove(&name)
            .ok_or(SmartHouseError::RemoveRoomError { name: name.clone() })?;
        Ok(name)
    }

    pub fn add_device(
        &mut self,
        room: String,
        device: RwLockDevice,
    ) -> Result<String, SmartHouseError> {
        let smartroom = self
            .rooms
            .get_mut(&room)
            .ok_or(SmartHouseError::GetRoomError { name: room.clone() })?;
        Ok(smartroom.add_device(device, None)?)
    }

    pub fn remove_device(
        &mut self,
        room: String,
        device_name: String,
    ) -> Result<String, SmartHouseError> {
        let smartroom = self
            .rooms
            .get_mut(&room)
            .ok_or(SmartHouseError::GetRoomError { name: room.clone() })?;
        Ok(smartroom.remove_device(device_name)?)
    }

    pub fn get(&self, rooms: Option<Vec<String>>) -> Vec<&SmartRoom> {
        if rooms.is_some() {
            self.rooms
                .iter()
                .filter_map(|(k, v)| {
                    if rooms.clone().unwrap().contains(k) {
                        Some(v)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
                .clone()
        } else {
            self.rooms.values().collect::<Vec<_>>().clone()
        }
    }

    pub fn devices(&self, room: String) -> Result<Vec<&String>, SmartHouseError> {
        // "список устройств в комнате `room`"
        let smartroom = self
            .rooms
            .get(&room)
            .ok_or(SmartHouseError::GetRoomError { name: room.clone() })?;
        Ok(smartroom.devices().keys().collect())
    }

    pub fn report(&self, rooms: Option<Vec<String>>) -> String {
        let mut result = format!("Name: {},\n", self.name);
        result += "Rooms:\n[\n";
        for room in self.get(rooms) {
            result += "{\n";
            result += &room.report(None);
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
        let test_house = SmartHouse::new("test_house".to_string());
        assert_eq!(test_house.get(None).len(), 0);
    }

    #[test]
    fn add_room() {
        let mut test_house = SmartHouse::new("test_house".to_string());

        assert_eq!(
            test_house.add_room("test_room".to_string()).unwrap(),
            "test_room".to_string()
        );
        assert_eq!(test_house.get(None).len(), 1);
        assert_eq!(
            test_house.get(Some(vec!["test_room".to_string()]))[0].name(),
            "test_room"
        );
    }

    #[test]
    fn remove_room() {
        let mut test_house = SmartHouse::new("test_house".to_string());

        assert_eq!(
            test_house.add_room("test_room".to_string()).unwrap(),
            "test_room".to_string()
        );
        assert_eq!(test_house.get(None).len(), 1);

        assert_eq!(
            test_house.remove_room("test_room".to_string()).unwrap(),
            "test_room".to_string()
        );
        assert_eq!(test_house.get(None).len(), 0);
    }

    #[test]
    fn add_device() {
        let name_room = "test_room".to_string();
        let name_device = "test_device".to_string();

        let good_result = vec![&name_device];

        let test_outlet = Arc::new(RwLock::new(SmartOutlet::new(
            "test_outlet".to_string(),
            None,
        )));
        let test_dev = RwLockDevice::new(Arc::new(RwLock::new(Device::new(
            "test_device".to_string(),
            test_outlet,
            None,
        ))));
        let mut test_house = SmartHouse::new("test_house".to_string());

        let _ = test_house.add_room(name_room.clone());
        let _ = test_house.add_device(name_room.clone(), test_dev);

        assert_eq!(test_house.devices(name_room.clone()).unwrap(), good_result);
    }

    #[test]
    fn remove_device() {
        let name_room = "test_room".to_string();
        let name_device = "test_device".to_string();

        let test_outlet = Arc::new(RwLock::new(SmartOutlet::new(
            "test_outlet".to_string(),
            None,
        )));

        let test_dev = RwLockDevice::new(Arc::new(RwLock::new(Device::new(
            "test_device".to_string(),
            test_outlet,
            None,
        ))));

        let mut test_house = SmartHouse::new("test_house".to_string());

        let _ = test_house.add_room(name_room.clone());
        let _ = test_house.add_device(name_room.clone(), test_dev);

        assert_eq!(
            test_house.devices(name_room.clone()).unwrap(),
            vec![&name_device]
        );

        let _ = test_house.remove_device(name_room.clone(), name_device.clone());

        assert_eq!(
            test_house.devices(name_room.clone()).unwrap(),
            Vec::<&String>::new()
        );
    }

    #[test]
    fn report() {
        let name_house = "test_house".to_string();
        let name_room = "test_room".to_string();

        let test_outlet = Arc::new(RwLock::new(SmartOutlet::new(
            "test_outlet".to_string(),
            None,
        )));

        let test_dev = RwLockDevice::new(Arc::new(RwLock::new(Device::new(
            "test_device".to_string(),
            test_outlet,
            None,
        ))));
        let mut test_house = SmartHouse::new(name_house.clone());

        // print!("{}\n",test_house.report(None));
        assert_eq!(
            test_house.report(None),
            "Name: test_house,\nRooms:\n[\n]".to_string()
        );

        let _ = test_house.add_room(name_room.clone());

        // print!("{}\n",test_house.report(None));
        assert_eq!(
            test_house.report(None),
            "Name: test_house,\nRooms:\n[\n{\nName: test_room,\nDevices:\n[\n]\n},\n]".to_string()
        );

        let _ = test_house.add_device(name_room.clone(), test_dev);

        // print!("{}\n",test_house.report(None));
        assert_eq!(test_house.report(None), "Name: test_house,\nRooms:\n[\n{\nName: test_room,\nDevices:\n[\n{\nName: test_device,\nOn: false,\nDescription: test_outlet,\nPower: 0\n},\n]\n},\n]".to_string());
    }
}
