pub mod outlet;
pub mod thermometer;
pub(crate) mod smartdevice;

use std::fmt::{self, Debug};

use std::option::Option;

use uuid::Uuid;

use async_trait::async_trait;
use anyhow::{anyhow, Result};

use std::sync::Arc;
// use std::sync::Mutex;
use std::sync::RwLock;
use std::sync::{LockResult, RwLockReadGuard, RwLockWriteGuard};

use tonic::{Request, Response, Status};

use self::smartdevice::SmartDevices;

use device_grpc::devices;
use device_grpc::devices::device_control_server::DeviceControl;
use device_grpc::devices::{DeviceStatus, Empty, Toggle};

use device_quic::common::make_client_endpoint;

#[derive(Debug, Clone)]
pub struct RwLockDevice {
    device: Arc<RwLock<Device>>,
}

impl RwLockDevice {
    pub fn read(&self) -> LockResult<RwLockReadGuard<'_, Device>> {
        self.device.read()
    }

    pub fn write(&self) -> LockResult<RwLockWriteGuard<'_, Device>> {
        self.device.write()
    }
}

// #[async_trait]
impl RwLockDevice {
    async fn listening(&self, client_addr: &str, server_addr: &str, cert_address: &str, server_cert: &Vec<u8>) -> Result<(), anyhow::Error> {        
        let endpoint_client = make_client_endpoint(
            client_addr.parse().unwrap(),
            &[&server_cert]
        )
            .map_err(|e| anyhow!("failed to make client endpoint: {}", e))
            .unwrap();
        // connect to server
        let outcoming_conn = endpoint_client
            .connect(
                server_addr.parse().unwrap(),
                cert_address
            )
            .map_err(|e| anyhow!("failed to make connecting: {}", e))
            .unwrap();
    
        let connection = outcoming_conn
            .await
            .map_err(|e| anyhow!("failed to create client connection: {}", e))
            .unwrap();
    
        while let Ok(mut recv) = connection
            .accept_uni()
            .await
            .map_err(|e| anyhow!("failed to create client Uni listener: {}", e))
            {
            // println!("start");
            'listener: loop {
                // Because it is a unidirectional stream, we can only receive not send back.
                let tempe = &mut [255_u8; 1];
                let size = recv.read_exact(tempe)
                    .await
                    .map_err(|e| anyhow!("failed to read: {}", e));
                // println!("size {:?}", size);
                if size.is_err() { 
                    // println!("finish");
                    break 'listener;
                }
                let _ = self
                        .write()
                        .unwrap()
                        .config()
                        .write()
                        .unwrap()
                        .listening(
                            tempe.to_vec()
                        );
                
                // println!("recv {:?}", i8::from_ne_bytes(*tempe));
                // self.temperature = i8::from_ne_bytes(*tempe);
            }
        }
        // Make sure the server has a chance to clean up
        endpoint_client.wait_idle().await;
        // println!("exit");
        Ok(())
    }
}

// Общая имплементация устройств
#[derive(Debug, Clone)]
pub struct Device{
    pub id: Uuid,     // у каждого девайса должен быть уникальный номер
    pub name: String, // у каждого девайса должено быть имя
    pub on: bool,     // каждый девайс может быть или работать или нет
    pub config: Arc<RwLock<dyn SmartDevices + Send + Sync>>, // каждый девайс имеет свой тип информации о себе, который можно прочитать
}

impl fmt::Display for Device {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Name: {},\nOn: {},\n{}",
            self.name,
            self.on,
            self.config
                .read()
                .unwrap()
        )
    }
}

impl Device {
    #[allow(dead_code)]
    pub(crate) fn new(
        name: String,
        config: Arc<RwLock<dyn SmartDevices + Send + Sync>>,
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
    pub(crate) fn config(&self) -> &Arc<RwLock<dyn SmartDevices + Send + Sync>> {
        &self.config
    }
}

#[tonic::async_trait]
impl DeviceControl for RwLockDevice {
    async fn switch(&self, request: Request<Toggle>) -> Result<Response<Empty>, Status> {
        println!("Received request from: {:?}", request);

        let status = self.read().unwrap().on;
        self
            .write()
            .unwrap()
            .on = !status;

        let response = devices::Empty {};

        Ok(Response::new(response))
    }

    async fn get_status(&self, _request: Request<Empty>) -> Result<Response<DeviceStatus>, Status> {
        let response = devices::DeviceStatus {
            id: self.read().unwrap().id().to_string(),
            name: self.read().unwrap().name().to_string(),
            on: self.read().unwrap().on().to_owned(),
            config: format!("{:?}", self
                .read()
                .unwrap()
                .config()
            ),
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

    use anyhow::{anyhow, Result, Error};

    use tokio;
    use tokio::sync::oneshot;
    use tokio::time::{sleep, Duration};

    use device_grpc::devices::device_control_client::DeviceControlClient;
    use device_grpc::devices::device_control_server::DeviceControlServer;
    use tonic::transport::Server;

    use device_quic::common::make_server_endpoint;
    // use device_quic::common::make_client_endpoint;

    #[test]
    fn get_name() {
        let test_outlet = Arc::new(
            RwLock::new(
                SmartOutlet::new(
                    "test_outlet".to_string(),
                    None
                )
            )
        );
        let test_dev = Device::new("test_device".to_string(), test_outlet, None);
        assert_eq!(test_dev.name(), "test_device".to_string());
    }

    #[test]
    fn get_config_outlet() {
        let test_outlet = Arc::new(
            RwLock::new(
                SmartOutlet::new(
                    "test_outlet".to_string(),
                    None
                )
            )
        );
        let test_dev = Device::new("test_device".to_string(), test_outlet, None);
        let config = test_dev
            .config();
        assert_eq!(
            format!("{config:?}"),
            "SmartOutlet { description: \"test_outlet\", power: 0 }"
        );
    }

    #[test]
    fn get_config_thermometer() {
        let test_thermometer = Arc::new(
            RwLock::new(
                SmartThermometer::new(
                    "test_thermometer".to_string(),
                    None
                )
            )
        );
        let test_dev = Device::new(
            "test_device".to_string(),
             test_thermometer,
             None
        );
        let config = test_dev.config();
        assert_eq!(
            format!("{config:?}"),
            "SmartThermometer { description: \"test_thermometer\", temperature: 0 }"
        );
    }

    // TEST connect

    #[tokio::test]
    async fn test_client_server_outlet() {
        let test_outlet = Arc::new(
            RwLock::new(
                SmartOutlet::new(
                    "test_outlet".to_string(),
                    None
                )
            )
        );
        let test_dev = Device::new("test_device".to_string(), test_outlet, None);

        let test_dev_rwlock = RwLockDevice {
            device: Arc::new(
                RwLock::new(
                    test_dev.clone()
                )
            ),
        };

        let addr = "127.0.0.1:50052";

        let (signal_tx, signal_rx) = oneshot::channel();

        tokio::task::spawn(
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
            format!("{:?}",
                response
                    .unwrap()
                    .into_inner()
            ),
            format!(
                "DeviceStatus {{ id: \"{}\", name: \"test_device\", on: false, config: \"RwLock {{ data: SmartOutlet {{ description: \\\"test_outlet\\\", power: 0 }}, poisoned: false, .. }}\" }}",
                test_dev.clone().id()
            )
        );

        let response = client.switch(Request::new(Toggle { on: true })).await;

        assert_eq!(format!("{:?}", response.unwrap().into_inner()), "Empty");

        let response = client.get_status(Request::new(Empty {})).await;

        assert_eq!(
            format!("{:?}", response
                .unwrap()
                .into_inner()
            ),
            format!(
                "DeviceStatus {{ id: \"{}\", name: \"test_device\", on: true, config: \"RwLock {{ data: SmartOutlet {{ description: \\\"test_outlet\\\", power: 0 }}, poisoned: false, .. }}\" }}",
                test_dev.clone().id()
            )
        );

        let _ = signal_tx.send(());
    }

    #[tokio::test]
    async fn test_client_server_thermometer() -> Result<(), Error> {
        let test_thermometer = Arc::new(
            RwLock::new(
                SmartThermometer::new(
                    "test_thermometer".to_string(),
                    None
                )
            )
        );
        
        let test_dev = Device::new("test_device".to_string(), test_thermometer, None);

        let test_dev_rwlock = RwLockDevice {
            device: Arc::new(
                RwLock::new(
                    test_dev
                )
            ),
        };



        let server_addr = "127.0.0.1:50053";
        let client_addr = "127.0.0.1:50054";
        let cert_address = "localhost";

        let arr2send = vec![30_i8, 0_i8, -20_i8,];

        let (endpoint_server, server_cert) = make_server_endpoint(
            server_addr.parse().unwrap(),
            cert_address
        ).map_err(|e| anyhow!("failed to create server endpoint: {}", e))?;

        let test_dev_rwlock_test = test_dev_rwlock.clone();

        tokio::spawn(async move {
            // println!( "start" );

            let incoming_conn = endpoint_server
                .accept()
                .await
                .unwrap();
            let connection = incoming_conn
                .await
                .map_err(|e| anyhow!("failed to server connection: {}", e))
                .unwrap();

            println!(
                "[server] connection accepted: addr={}",
                &connection.remote_address()
            );

            let mut send = connection
                .open_uni()
                .await
                .map_err(|e| anyhow!("failed to connection SendStream: {}", e))
                .unwrap();

            for temp in arr2send.iter(){
                let _ = sleep(Duration::from_millis(1000)).await;

                send.write_all(&(temp.to_ne_bytes()))
                    .await
                    .map_err(|e| anyhow!("failed to send request: {}", e))
                    .unwrap();

                println!("send {}", &temp);

                let _ = sleep(Duration::from_millis(500)).await;

                let config = test_dev_rwlock_test
                    .read()
                    .unwrap()
                    .config()
                    .read()
                    .unwrap()
                    .to_string();
                
                assert_eq!(
                    format!("{:}", config),
                    format!("Description: test_thermometer,\nTemperature: {}", temp)
                );

            }

            let _ = sleep(Duration::from_millis(1000)).await;

            let _ = send
                .finish()
                .await
                .map_err(|e| anyhow!("failed to finish: {}", e))
                .unwrap();

            // println!("finish send");

            // Dropping all handles associated with a connection implicitly closes it
        });

        let _ = sleep(Duration::from_millis(1000)).await;

        let _ = test_dev_rwlock
            .listening(client_addr, server_addr, cert_address, &server_cert)
            .await;

        // let _ = test_dev_rwlock
        //     .write()
        //     .unwrap()
        //     .config()
        //     .write()
        //     .unwrap()
        //     .listening(
        //         client_addr,
        //         server_addr,
        //         cert_address, 
        //         &server_cert
        //     )
        //     .await;

        // let config = test_dev_rwlock_test
        //     .read()
        //     .unwrap()
        //     .config()
        //     .read()
        //     .unwrap()
        //     .to_string();

        // println!("config {:?}", config);


        // let endpoint_client = make_client_endpoint(
        //     client_addr.parse().unwrap(),
        //      &[&server_cert]
        //     )
        //     .map_err(|e| anyhow!("failed to create client endpoint: {}", e))?;

        // // connect to server
        // let outcoming_conn = endpoint_client
        //     .connect(
        //         server_addr.parse().unwrap(),
        //          cert_address
        //     )
        //     .unwrap();
        // let connection = outcoming_conn
        //     .await
        //     .map_err(|e| anyhow!("failed to client connection: {}", e))?;
        
        // println!("[client] connected: addr={}", connection.remote_address());

        // while let Ok(mut recv) = connection
        //     .accept_uni()
        //     .await
        //     .map_err(|e| anyhow!("failed to create client Uni listener: {}", e))
        //     {
        //     println!("start");
        //     'listener: loop {
        //         // Because it is a unidirectional stream, we can only receive not send back.
        //         let temp = &mut [255_u8; 1];
        //         let size = recv.read_exact(temp)
        //             .await
        //             .map_err(|e| anyhow!("failed to read: {}", e));
        //         println!("size {:?}", size);
        //         if size.is_err() { 
        //             println!("finish");
        //             break 'listener;
        //         }
        //         println!("recv {:?}", i8::from_ne_bytes(*temp));
        //         // self.temperature = i8::from_ne_bytes(*temp);
        //     }
        // }
        // // Make sure the server has a chance to clean up
        // endpoint_client.wait_idle().await;
        // println!("exit");

        Ok(())
    }

}
