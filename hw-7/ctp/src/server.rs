use crate::error::{ConnectError, ConnectResult, RecvResult, SendResult};
use std::io;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream, ToSocketAddrs};
use thiserror::Error;

/// Represent CTP server, that can accept incoming connections.
pub struct CtpServer {
    tcp: TcpListener,
}

impl CtpServer {
    /// Binds server to specefied socket.
    pub fn bind<Addrs>(addrs: Addrs) -> BindResult
    where
        Addrs: ToSocketAddrs,
    {
        let tcp = TcpListener::bind(addrs)?;
        Ok(Self { tcp })
    }

    /// Blocking iterator for incoming connections.
    pub fn incoming(&self) -> impl Iterator<Item = ConnectResult<CtpConnection>> + '_ {
        self.tcp.incoming().map(|s| match s {
            Ok(s) => Self::try_handshake(s),
            Err(e) => Err(ConnectError::Io(e)),
        })
    }

    fn try_handshake(mut stream: TcpStream) -> ConnectResult<CtpConnection> {
        let mut buf = [0; 4];
        stream.read_exact(&mut buf)?;
        if &buf != b"clnt" {
            let msg = format!("received: {:?}", buf);
            return Err(ConnectError::BadHandshake(msg));
        }
        stream.write_all(b"serv")?;
        Ok(CtpConnection { stream })
    }
}

pub type BindResult = Result<CtpServer, BindError>;

/// Bind to socket error
#[derive(Debug, Error)]
pub enum BindError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
}

/// Represent connection from client.
///
/// Allows to receive requests and send responses.
pub struct CtpConnection {
    stream: TcpStream,
}

impl CtpConnection {
    /// Send response to client
    pub fn send_response<Resp: AsRef<str>>(&mut self, response: Resp) -> SendResult {
        crate::send_comand(response, &mut self.stream)
    }

    /// Receive requests from client
    pub fn recv_request(&mut self) -> RecvResult {
        crate::recv_status(&mut self.stream)
    }

    /// Address of connected client
    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        self.stream.peer_addr()
    }
}