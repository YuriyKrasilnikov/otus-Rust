use crate::error::{ConnectError, ConnectResult, RecvError, SendError};
use std::io::{Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use thiserror::Error;

/// Represent client-side connection for CTP
pub struct CtpClient {
    stream: TcpStream,
}

impl CtpClient {
    /// Try to connect to specified address and perform handshake.
    pub fn connect<Addrs>(addrs: Addrs) -> ConnectResult<Self>
    where
        Addrs: ToSocketAddrs,
    {
        let stream = TcpStream::connect(addrs)?;
        Self::try_handshake(stream)
    }

    /// Send request to connected CTP server.
    pub fn send_request<R: AsRef<str>>(&mut self, req: R) -> RequestResult {
        crate::send_comand(req, &mut self.stream)?;
        let response = crate::recv_status(&mut self.stream)?;
        Ok(response)
    }

    fn try_handshake(mut stream: TcpStream) -> ConnectResult<Self> {
        stream.write_all(b"clnt")?;
        let mut buf = [0; 4];
        stream.read_exact(&mut buf)?;
        if &buf != b"serv" {
            let msg = format!("received: {:?}", buf);
            return Err(ConnectError::BadHandshake(msg));
        }
        Ok(Self { stream })
    }
}

pub type RequestResult = Result<String, RequestError>;

/// Error for request sending. It consists from two steps: sending and receiving data.
///
/// `SendError` caused by send data error.
/// `RecvError` caused by receive data error.
#[derive(Debug, Error)]
pub enum RequestError {
    #[error(transparent)]
    Send(#[from] SendError),
    #[error(transparent)]
    Recv(#[from] RecvError),
}