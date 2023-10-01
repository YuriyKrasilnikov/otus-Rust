use std::fmt;

use std::option::Option;

use async_trait::async_trait;
use anyhow::{anyhow, Result};

use super::SmartDevices;

use device_quic::common::make_client_endpoint;

// Термометр

#[derive(Debug, Clone, PartialEq)]
pub struct SmartThermometer {
    description: String,
    temperature: i8,
}

#[async_trait]
impl SmartDevices for SmartThermometer {
    async fn listening(&mut self, client_addr: &str, server_addr: &str, cert_address: &str, server_cert: &Vec<u8>) -> Result<(), anyhow::Error> {
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
                println!("recv {:?}", i8::from_ne_bytes(*tempe));
                self.temperature = i8::from_ne_bytes(*tempe);
            }
        }
        // Make sure the server has a chance to clean up
        endpoint_client.wait_idle().await;
        // println!("exit");
        Ok(())
    }
}

impl fmt::Display for SmartThermometer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Description: {},\nTemperature: {}",
            self.description, self.temperature
        )
    }
}

#[allow(dead_code)]
impl SmartThermometer {
    pub(crate) fn new(description: String, temperature: Option<i8>) -> Self {
        SmartThermometer {
            description,
            temperature: temperature.unwrap_or(0),
        }
    }
    pub(crate) fn description(&self) -> &str {
        &self.description
    }
    pub(crate) fn temperature(&self) -> &i8 {
        &self.temperature
    }
}

#[cfg(test)]
mod tests {
    #[warn(unused_imports)]
    use super::*;

    #[test]
    fn new() {
        assert_eq!(
            SmartThermometer::new("test".to_string(), None),
            SmartThermometer {
                description: "test".to_string(),
                temperature: 0,
            }
        );
    }

    #[test]
    fn display() {
        assert_eq!(
            SmartThermometer::new("test".to_string(), None).to_string(),
            "Description: test,\nTemperature: 0"
        );
    }
}
