pub mod outlet;
pub mod thermometer;

use std::fmt::{self, Debug};

use std::option::Option;

extern crate uuid;
use self::uuid::Uuid;

extern crate dyn_partial_eq;
use self::dyn_partial_eq::dyn_partial_eq;

#[dyn_partial_eq]
pub(crate) trait SmartDevices: fmt::Debug + fmt::Display + SmartDevicesClone{}

pub(crate) trait SmartDevicesClone {
    fn clone_box(&self) -> Box<dyn SmartDevices>;
    // fn partial_eq_box(&self) -> bool;
}

impl<T> SmartDevicesClone for T
where
    T: 'static + SmartDevices + Clone,
{
    fn clone_box(&self) -> Box<dyn SmartDevices> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn SmartDevices> {
    fn clone(&self) -> Box<dyn SmartDevices> {
        self.clone_box()
    }
}

// Общая имплементация устройств
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct Device {
    id: Uuid,     // у каждого девайса должен быть уникальный номер
    name: String, // у каждого девайса должено быть имя
    on: bool,     // каждый девайс может быть или работать или нет
    config: Box<dyn SmartDevices>,    // каждый девайс имеет свой тип информации о себе, который можно прочитать
}

impl fmt::Display for Device {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Name: {},\nOn: {},\n{}", self.name, self.on, self.config)
    }
}

impl Device {
    #[allow(dead_code)]
    pub(crate) fn new(name: String, config: Box<dyn SmartDevices>, on: Option<bool>) -> Self {
        Device {
            id: Uuid::new_v4(),
            name,
            on: on.unwrap_or(false),
            config,
        }
    }
    #[allow(dead_code)]
    pub(crate) fn id(&self) -> &Uuid {
        &self.id
    }
    #[allow(dead_code)]
    pub(crate) fn name(&self) -> &str {
        &self.name
    }
    #[allow(dead_code)]
    pub(crate) fn on(&self) -> &bool {
        &self.on
    }
    #[allow(dead_code, clippy::borrowed_box)]
    pub(crate) fn config(&self) -> &Box<dyn SmartDevices> {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    #[warn(unused_imports)]
    use super::*;

    #[warn(unused_imports)]
    use self::outlet::SmartOutlet;
    // #[warn(unused_imports)]
    // use self::thermometer::SmartThermometer;

    #[test]
    fn test_get_name() {
        let test_outlet = Box::new(SmartOutlet::new("test".to_string(), None));
        let test_dev = Device::new("test_device".to_string(), test_outlet, None);
        assert_eq!(test_dev.name(),"test_device".to_string());
    }

}
