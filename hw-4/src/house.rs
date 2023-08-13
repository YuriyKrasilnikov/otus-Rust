use std::option::Option;

use std::collections::HashMap;
use uuid::Uuid;



// Умный дом

#[allow(dead_code)]
#[derive(Clone)]
struct SmartHouse {
    id: Uuid,
    name: String,
    rooms: HashMap<String, SmartRoom>,
}

#[allow(dead_code)]
impl SmartHouse {
    fn new(name: String) -> Self {
        // инициализация дома
        SmartHouse {
            id: Uuid::new_v4(),
            name,
            rooms: HashMap::new(),
        }
    }

    fn add_room(&mut self, name: String) -> fmt::Result {
        self.rooms.insert(
            name.clone(),
            SmartRoom {
                name,
                devices: HashMap::new(),
            },
        );
        Ok(())
    }

    fn get_rooms(&self) -> Vec<&String> {
        // список комнат
        Vec::from_iter(self.rooms.keys())
    }

    fn add_device(&mut self, room: String, device: Device) -> Result<(), RoomError> {
        let smartroom = self.rooms.get_mut(&room).ok_or(RoomError {})?;
        //SmartRoom::add_devices(smartroom, device);
        let _ = smartroom.add_devices(device);
        Ok(())
    }

    fn get(&self, rooms: Option<Vec<String>>) -> Vec<&SmartRoom> {
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

    fn devices(&self, room: String) -> Result<Vec<&String>, RoomError> {
        // "список устройств в комнате `room`"
        let smartroom = self.rooms.get(&room).ok_or(RoomError {})?;
        Ok(smartroom.get_devices_name())
    }

    fn report(&self, rooms: Option<Vec<String>>) -> fmt::Result {
        for room in self.get(rooms) {
            let _ = room.report(None);
        }
        Ok(())
    }
}