use std::fmt;

use std::option::Option;

use uuid::Uuid;


// Общая имплементация устройств
#[allow(dead_code)]
#[derive(Debug, Clone)]
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