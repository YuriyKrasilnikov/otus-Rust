extern crate uuid;

use std::collections::HashMap;
use std::fmt;
use std::iter::FromIterator;
use std::option::Option;
use uuid::Uuid;

//Устройства
#[derive(Clone)]
struct SmartOutlet {
    description: String,
    power: u8,
}

#[derive(Clone)]
struct SmartThermometer {
    description: String,
    temperature: u8,
}

impl fmt::Display for SmartOutlet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "(Description: {}\nPower: {})",
            self.description, self.power
        )
    }
}

impl fmt::Display for SmartThermometer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "(Description: {}\nTemperature: {})",
            self.description, self.temperature
        )
    }
}

#[allow(dead_code)]
impl SmartOutlet {
    fn new(description: String, power: Option<u8>) -> Self {
        SmartOutlet {
            description,
            power: power.unwrap_or(0),
        }
    }
}

#[allow(dead_code)]
impl SmartThermometer {
    fn new(description: String, temperature: Option<u8>) -> Self {
        SmartThermometer {
            description,
            temperature: temperature.unwrap_or(0),
        }
    }
}

// Общая имплементация устройств
#[allow(dead_code)]
#[derive(Clone)]
struct SmartDevice<I: fmt::Display> {
    id: Uuid,     // у каждого девайса должен быть уникальный номер
    name: String, // у каждого девайса должено быть имя
    on: bool,     // каждый девайс может быть или работать или нет
    config: I,    // каждый девайс имеет свой тип информации о себе, который можно прочитать
}

impl<I: fmt::Display> fmt::Display for SmartDevice<I> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(Name: {}\nOn: {}\n{})", self.name, self.on, self.config)
    }
}

impl<I: fmt::Display> SmartDevice<I> {
    fn new(name: String, config: I, on: Option<bool>) -> Self {
        SmartDevice {
            id: Uuid::new_v4(),
            name,
            on: on.unwrap_or(false),
            config,
        }
    }
}

impl<I: fmt::Display> SmartDevice<I> {
    pub fn get_name(device: &Self) -> String {
        device.name.clone()
    }
}

// Список типов устройств

#[derive(Clone)]
enum Device {
    Outlet(SmartDevice<SmartOutlet>),
    Thermometer(SmartDevice<SmartThermometer>),
}

macro_rules! fn_device {
    ($ex_af_device:expr, $op:expr) => {
        match &$ex_af_device {
            Device::Outlet(device) => $op(device),
            Device::Thermometer(device) => $op(device),
        }
    };
}

impl fmt::Display for Device {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Device::Outlet(device) => write!(f, "{}", device),
            Device::Thermometer(device) => write!(f, "{}", device),
        }
    }
}

// Комната
#[allow(dead_code)]
#[derive(Clone)]
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
            .insert(fn_device!(device, SmartDevice::get_name), device);
        Ok(())
    }
}

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

fn main() {
    // Инициализация дома

    let mut house = SmartHouse::new("Дом 1".to_string());

    let _ = house.add_room("Комната 1".to_string());
    let _ = house.add_room("Комната 2".to_string());

    let socket1_device = Device::Outlet(SmartDevice::new(
        "Розетка-1".to_string(),
        SmartOutlet {
            description: "Розетка 1".to_string(),
            power: 220,
        },
        None,
    ));

    let socket2_device = Device::Outlet(SmartDevice::new(
        "Розетка-2".to_string(),
        SmartOutlet {
            description: "Розетка 2".to_string(),
            power: 220,
        },
        None,
    ));

    let thermo_device = Device::Thermometer(SmartDevice::new(
        "Термометр-1".to_string(),
        SmartThermometer {
            description: "Термометр 1".to_string(),
            temperature: 25,
        },
        None,
    ));

    let _ = house.add_device("Комната 1".to_string(), socket1_device.clone());

    let _ = house.add_device("Комната 1".to_string(), thermo_device.clone());

    let _ = house.add_device("Комната 2".to_string(), socket2_device.clone());

    println!("----------");
    println!("Получить список комнат:");
    let _ = house.get_rooms();

    println!("----------");
    println!("Репорт по всем комнатам:");
    let _ = house.report(None);

    println!("----------");
    println!("Репорт полько по 1 комнате:");
    let _ = house.report(Some(vec!["Комната 1".to_string()]));
}
