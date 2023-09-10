extern crate thiserror;

use crate::error::{RecvError, RecvResult, SendResult};
use std::io::{Read, Write};

pub mod client;
pub mod error;
pub mod server;

fn send_comand<Data: AsRef<str>, Writer: Write>(data: Data, mut writer: Writer) -> SendResult {
    let bytes = data.as_ref().as_bytes();
    let len = bytes.len() as u32;
    let len_bytes = len.to_be_bytes();
    writer.write_all(&len_bytes)?;
    writer.write_all(bytes)?;
    Ok(())
}

fn recv_status<Reader: Read>(mut reader: Reader) -> RecvResult {
    let mut buf = [0; 4];
    reader.read_exact(&mut buf)?;
    let len = u32::from_be_bytes(buf);

    let mut buf = vec![0; len as _];
    reader.read_exact(&mut buf)?;
    String::from_utf8(buf).map_err(|_| RecvError::BadEncoding)
}


#[cfg(test)]
mod tests {

    #[warn(unused_imports)]
    use std::thread;
    #[warn(unused_imports)]
    use std::error::Error;
    #[warn(unused_imports)]
    use self::client::CtpClient;
    #[warn(unused_imports)]
    use self::server::{CtpConnection, CtpServer};

    #[warn(unused_imports)]
    use super::*;

    fn create_client() -> Result<(), Box<dyn Error>>{
        println!("start create_client");
        let mut client = CtpClient::connect("127.0.0.1:55331")?;
        let response = client.send_request("Hello, server")?;
        assert_eq!(response, "Hello, client");
        println!("end create_client");
        Ok(())
    }

    fn process_connection(mut conn: CtpConnection) -> Result<(), Box<dyn Error>> {
        println!("start process_connection");
        let req = conn.recv_request()?;
        assert_eq!(req, "Hello, server");
        conn.send_response("Hello, client")?;
        println!("end process_connection");
        Ok(())
    }

    fn create_server() -> Result<(), Box<dyn Error>>{
        println!("start create_server");
        let server = CtpServer::bind("127.0.0.1:55331")?;
        println!("server created");
        for connection in server.incoming() {
            process_connection(connection?)?;
            println!("break connection");
            break
        }
        println!("end create_server");
        Ok(())
    }

    #[test]
    fn client_server(){
        let _ = thread::spawn(||{
            let _ = create_server();
        });
        let _ = create_client();
    }


}