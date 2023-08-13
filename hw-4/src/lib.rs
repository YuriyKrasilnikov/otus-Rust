pub mod house {
  pub mod room;
}

pub mod device {
  pub struct Device;

  pub mod smart_device{
    pub struct SmartDevice;
    pub mod outlet;
    pub mod thermometer;
  }
}