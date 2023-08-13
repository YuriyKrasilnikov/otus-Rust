use std::fmt;

// Список типов устройств

#[derive(Clone)]
enum Device {
    Outlet(SmartDevice<SmartOutlet>),
    Thermometer(SmartDevice<SmartThermometer>),
}

#[macro_export]
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

impl fmt::Debug for Device {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match self {
          Device::Outlet(device) => write!(f, "{}", device),
          Device::Thermometer(device) => write!(f, "{}", device),
      }
  }
}

pub use fn_device;