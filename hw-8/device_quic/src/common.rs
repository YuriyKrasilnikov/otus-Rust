use quinn::{ClientConfig, Endpoint, ServerConfig};
use std::{error::Error, net::SocketAddr, sync::Arc};

/// Constructs a QUIC endpoint configured for use a client only.
///
/// ## Args
///
/// - server_certs: list of trusted certificates.
#[allow(unused)]
pub fn make_client_endpoint(
    bind_addr: SocketAddr,
    server_certs: &[&[u8]],
) -> Result<Endpoint, Box<dyn Error>> {
    let client_cfg = configure_client(server_certs)?;
    let mut endpoint = Endpoint::client(bind_addr)?;
    endpoint.set_default_client_config(client_cfg);
    Ok(endpoint)
}

/// Constructs a QUIC endpoint configured to listen for incoming connections on a certain address
/// and port.
///
/// ## Returns
///
/// - a stream of incoming QUIC connections
/// - server certificate serialized into DER format
#[allow(unused)]
pub fn make_server_endpoint(
    bind_addr: SocketAddr,
    cert_address: &str,
) -> Result<(Endpoint, Vec<u8>), Box<dyn Error>> {
    let (server_config, server_cert) = configure_server(cert_address)?;
    let endpoint = Endpoint::server(server_config, bind_addr)?;
    Ok((endpoint, server_cert))
}

/// Builds default quinn client config and trusts given certificates.
///
/// ## Args
///
/// - server_certs: a list of trusted certificates in DER format.
fn configure_client(server_certs: &[&[u8]]) -> Result<ClientConfig, Box<dyn Error>> {
    let mut certs = rustls::RootCertStore::empty();
    for cert in server_certs {
        certs.add(&rustls::Certificate(cert.to_vec()))?;
    }

    let client_config = ClientConfig::with_root_certificates(certs);
    Ok(client_config)
}

/// Returns default server configuration along with its certificate.
fn configure_server(cert_address: &str) -> Result<(ServerConfig, Vec<u8>), Box<dyn Error>> {
    let cert = rcgen::generate_simple_self_signed(vec![cert_address.into()]).unwrap();
    let cert_der = cert.serialize_der().unwrap();
    let priv_key = cert.serialize_private_key_der();
    let priv_key = rustls::PrivateKey(priv_key);
    let cert_chain = vec![rustls::Certificate(cert_der.clone())];

    let mut server_config = ServerConfig::with_single_cert(cert_chain, priv_key)?;
    let transport_config = Arc::get_mut(&mut server_config.transport).unwrap();
    transport_config.max_concurrent_uni_streams(0_u8.into());

    Ok((server_config, cert_der))
}

#[allow(unused)]
pub const ALPN_QUIC_HTTP: &[&[u8]] = &[b"hq-29"];

#[cfg(test)]
mod tests {
    #[warn(unused_imports)]
    use super::*;

    #[warn(unused_imports)]
    use tokio::time::{sleep, Duration};

    // Test Client Server
    #[tokio::test]
    async fn test_client_server() -> Result<(), Box<dyn std::error::Error>> {
        let server_addr = "127.0.0.1:5000".parse().unwrap();
        let cert_address = "localhost";
        let (endpoint, server_cert) = make_server_endpoint(server_addr, cert_address)?;

        tokio::spawn(async move {
            let incoming_conn = endpoint.accept().await.unwrap();
            let connection = incoming_conn.await.unwrap();

            // println!(
            //     "[server] connection accepted: addr={}",
            //     &connection.remote_address()
            // );
            assert_eq!("127.0.0.1:1234", connection.remote_address().to_string());

            while let Ok((mut send, mut recv)) = connection.accept_bi().await {
                // Because it is a bidirectional stream, we can both send and receive.
                let request = recv.read_to_end(50).await.unwrap();

                // println!("request: {:?}", String::from_utf8(request).unwrap());
                assert_eq!("request", String::from_utf8(request).unwrap());

                send.write_all(b"response").await.unwrap();
                send.finish().await.unwrap();
            }

            // Dropping all handles associated with a connection implicitly closes it
        });

        let _ = sleep(Duration::from_millis(1000)).await;

        let endpoint = make_client_endpoint("0.0.0.0:1234".parse().unwrap(), &[&server_cert])?;
        // connect to server
        let outcoming_conn = endpoint.connect(server_addr, cert_address).unwrap();
        let connection = outcoming_conn.await.unwrap();

        // println!("[client] connected: addr={}", connection.remote_address());
        assert_eq!("127.0.0.1:5000", connection.remote_address().to_string());

        let (mut send, mut recv) = connection.open_bi().await?;

        send.write_all(b"request").await?;
        send.finish().await?;

        let responce = recv.read_to_end(10).await?;
        // println!("responce: {:?}", String::from_utf8(responce).unwrap());
        assert_eq!("response", String::from_utf8(responce).unwrap());

        // Make sure the server has a chance to clean up
        endpoint.wait_idle().await;

        // let server_out = server.await.unwrap();
        // assert_eq!("127.0.0.1:1234", server_out.to_string());

        Ok(())
    }
}
