pub(crate) mod room;

use std::fmt;
use std::option::Option;

use std::collections::HashMap;

extern crate uuid;
use self::uuid::Uuid;

use self::room::RoomError;
use self::room::SmartRoom;

use device::Device;

// Умный дом

#[allow(dead_code)]
#[derive(Clone)]
pub struct SmartHouse {
    id: Uuid,
    name: String,
    rooms: HashMap<String, SmartRoom>,
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
    
    pub fn add_room(&mut self, name: String) -> fmt::Result {
        self.rooms.insert(
            name.clone(),
            SmartRoom::new(name)
        );
        Ok(())
    }

    pub fn add_device(&mut self, room: String, device: Device) -> Result<(), RoomError> {
        let smartroom = self.rooms.get_mut(&room).ok_or(RoomError {})?;
        //SmartRoom::add_devices(smartroom, device);
        let _ = smartroom.add_device(device);
        Ok(())
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

    pub fn devices(&self, room: String) -> Result<Vec<&String>, RoomError> {
        // "список устройств в комнате `room`"
        let smartroom = self.rooms.get(&room).ok_or(RoomError {})?;
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

    #[warn(unused_imports)]
    use device::Device;
    #[warn(unused_imports)]
    use device::outlet::SmartOutlet;
    // #[warn(unused_imports)]
    // use device::thermometer::SmartThermometer;

    #[test]
    fn test_get_null() {
        let test_house = SmartHouse::new("test_house".to_string());
        assert_eq!(test_house.get(None).len(), 0);
    }

    #[test]
    fn test_add_room() {
        let mut test_house = SmartHouse::new("test_house".to_string());

        assert_eq!(test_house.add_room("test_room".to_string()), Ok(()));
        assert_eq!(test_house.get(None).len(), 1);
        assert_eq!(
            test_house.get(Some(vec!["test_room".to_string()]))[0].name(),
            "test_room"
        );
    }
    

    #[test]
    fn test_add_device() {
        let name_room = "test_room".to_string();
        let name_device = "test_device".to_string();
        
        let good_result: Result<Vec<&String>, RoomError> = Ok(vec![&name_device]);

        let test_outlet = Box::new(SmartOutlet::new("test".to_string(), None));
        let test_dev = Device::new(name_device.clone(), test_outlet, None);
        let mut test_house = SmartHouse::new("test_house".to_string());

        let _ = test_house.add_room(name_room.clone());
        let _ = test_house.add_device(name_room.clone(),test_dev.clone());

        assert_eq!(test_house.devices(name_room.clone()), good_result);
    }
   
    #[test]
    fn test_report() {
        let name_house = "test_house".to_string();
        let name_room = "test_room".to_string();
        let name_device = "test_device".to_string();

        let good_result = "Name: test_house,\nRooms:\n[\n{\nName: test_room,\nDevices:\n[\n{\nName: test_device,\nOn: false,\nDescription: test,\nPower: 0\n},\n]\n},\n]".to_string();

        let test_outlet = Box::new(SmartOutlet::new("test".to_string(), None));
        let test_dev = Device::new(name_device.clone(), test_outlet, None);
        let mut test_house = SmartHouse::new(name_house.clone());

        let _ = test_house.add_room(name_room.clone());
        let _ = test_house.add_device(name_room.clone(),test_dev.clone());

        let report = test_house.report(None);

        // print!("{}\n",report);

        assert_eq!(report, good_result);
    
    }


}