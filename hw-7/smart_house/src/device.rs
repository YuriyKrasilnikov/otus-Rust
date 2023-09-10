pub mod outlet;
pub mod thermometer;

use std::fmt::{self, Debug};

use std::option::Option;

use uuid::Uuid;

use std::sync::Arc;
// use std::sync::Mutex;
use std::sync::RwLock;

use tonic::{Request, Response, Status};

use ctp::devices;
use ctp::devices::device_control_server::DeviceControl;
use ctp::devices::{DeviceStatus, Empty, Toggle};

pub(crate) trait SmartDevices: fmt::Debug + fmt::Display {}

// Общая имплементация устройств
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Device {
    pub(self) id: Uuid,     // у каждого девайса должен быть уникальный номер
    pub(self) name: String, // у каждого девайса должено быть имя
    pub(self) on: bool,     // каждый девайс может быть или работать или нет
    pub(self) config: Arc<dyn SmartDevices + Sync + Send>, // каждый девайс имеет свой тип информации о себе, который можно прочитать
}

impl fmt::Display for Device {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Name: {},\nOn: {},\n{}", self.name, self.on, self.config)
    }
}

impl Device {
    #[allow(dead_code)]
    pub(crate) fn new(
        name: String,
        config: Arc<dyn SmartDevices + Sync + Send>,
        on: Option<bool>,
    ) -> Self {
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
    pub(crate) fn config(&self) -> &Arc<dyn SmartDevices + Sync + Send> {
        &self.config
    }
}

pub(self) struct RwLockDevice {
    device: RwLock<Device>,
}

#[tonic::async_trait]
impl DeviceControl for RwLockDevice {
    async fn switch(&self, request: Request<Toggle>) -> Result<Response<Empty>, Status> {
        println!("Received request from: {:?}", request);

        let status = self.device.read().unwrap().on;
        self.device.write().unwrap().on = !status;

        let response = devices::Empty {};

        Ok(Response::new(response))
    }

    async fn get_status(&self, _request: Request<Empty>) -> Result<Response<DeviceStatus>, Status> {
        let response = devices::DeviceStatus {
            id: self.device.read().unwrap().id().to_string(),
            name: self.device.read().unwrap().name().to_string(),
            on: self.device.read().unwrap().on().to_owned(),
            config: format!("{:?}", self.device.read().unwrap().config()),
        };

        Ok(Response::new(response))
    }
}

#[cfg(test)]
mod tests {
    #[warn(unused_imports)]
    use super::*;

    use self::outlet::SmartOutlet;
    use self::thermometer::SmartThermometer;

    use tokio::spawn;
    use tokio::sync::oneshot;
    use tokio::time::{sleep, Duration};

    use ctp::devices::device_control_client::DeviceControlClient;
    use ctp::devices::device_control_server::DeviceControlServer;
    use tonic::transport::Server;

    #[test]
    fn get_name() {
        let test_outlet = Arc::new(SmartOutlet::new("test".to_string(), None));
        let test_dev = Device::new("test_device".to_string(), test_outlet, None);
        assert_eq!(test_dev.name(), "test_device".to_string());
    }

    #[test]
    fn get_config_outlet() {
        let test_outlet = Arc::new(SmartOutlet::new("test_outlet".to_string(), None));
        let test_dev = Device::new("test_device".to_string(), test_outlet, None);
        let config = test_dev.config();
        assert_eq!(
            format!("{config:?}"),
            "SmartOutlet { description: \"test_outlet\", power: 0 }"
        );
    }

    #[test]
    fn get_config_thermometer() {
        let test_thermometer =
            Arc::new(SmartThermometer::new("test_thermometer".to_string(), None));
        let test_dev = Device::new("test_device".to_string(), test_thermometer, None);
        let config = test_dev.config();
        assert_eq!(
            format!("{config:?}"),
            "SmartThermometer { description: \"test_thermometer\", temperature: 0 }"
        );
    }

    // TEST connect

    #[tokio::test]
    async fn test_client_server() {
        let test_outlet = Arc::new(SmartOutlet::new("test_outlet".to_string(), None));
        let test_dev = Device::new("test_device".to_string(), test_outlet, None);

        let test_dev_rwlock = RwLockDevice {
            device: RwLock::new(test_dev.clone()),
        };

        let addr = "127.0.0.1:50052";

        let (signal_tx, signal_rx) = oneshot::channel();

        spawn(
            Server::builder()
                .add_service(DeviceControlServer::new(test_dev_rwlock))
                .serve_with_shutdown(addr.parse().unwrap(), async {
                    signal_rx.await.ok();
                    println!("Server shutdown");
                }),
        );

        let _ = sleep(Duration::from_millis(1000)).await;

        println!("Server start at {}", addr);

        #[warn(unused_mut)]
        let mut client = DeviceControlClient::connect("http://".to_owned() + addr)
            .await
            .unwrap();

        println!("Client start at {}", "http://".to_owned() + addr);

        let response = client.get_status(Request::new(Empty {})).await;

        assert_eq!(
            format!("{:?}", response.unwrap().into_inner()),
            format!("DeviceStatus {{ id: \"{}\", name: \"test_device\", on: false, config: \"SmartOutlet {{ description: \\\"test_outlet\\\", power: 0 }}\" }}", test_dev.clone().id())
        );

        let response = client.switch(Request::new(Toggle { on: true })).await;

        assert_eq!(format!("{:?}", response.unwrap().into_inner()), "Empty");

        let response = client.get_status(Request::new(Empty {})).await;

        assert_eq!(
            format!("{:?}", response.unwrap().into_inner()),
            format!("DeviceStatus {{ id: \"{}\", name: \"test_device\", on: true, config: \"SmartOutlet {{ description: \\\"test_outlet\\\", power: 0 }}\" }}", test_dev.clone().id())
        );

        let _ = signal_tx.send(());
    }
}
