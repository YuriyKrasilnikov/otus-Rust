pub mod outlet;
pub mod thermometer;

use std::fmt;

use std::option::Option;

extern crate uuid;
use self::uuid::Uuid;

// Общая имплементация устройств
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SmartDevice<I: fmt::Display> {
    pub id: Uuid,     // у каждого девайса должен быть уникальный номер
    pub name: String, // у каждого девайса должено быть имя
    pub on: bool,     // каждый девайс может быть или работать или нет
    pub config: I,    // каждый девайс имеет свой тип информации о себе, который можно прочитать
}

impl<I: fmt::Display> fmt::Display for SmartDevice<I> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(Name: {}\nOn: {}\n{})", self.name, self.on, self.config)
    }
}

impl<I: fmt::Display> SmartDevice<I> {
    pub fn new(name: String, config: I, on: Option<bool>) -> Self {
        SmartDevice {
            id: Uuid::new_v4(),
            name,
            on: on.unwrap_or(false),
            config,
        }
    }
}

impl<I: fmt::Display> SmartDevice<I> {
    pub(crate) fn get_name(device: &Self) -> String {
        device.name.clone()
    }
}
