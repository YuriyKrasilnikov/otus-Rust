extern crate hw4;

use hw4::device::smart_device::outlet::SmartOutlet;
use hw4::device::smart_device::thermometer::SmartThermometer;
use hw4::device::smart_device::SmartDevice;
use hw4::device::Device;
use hw4::house::SmartHouse;

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
