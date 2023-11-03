mod cli;
use clap::Parser;
use cli::*;
use std::sync::Arc;

const CERT: &[u8] = include_bytes!("../dev/localhost.crt");
const KEY: &[u8] = include_bytes!("../dev/localhost.key");

fn main() {
    let config = Config::parse();

    let cert = &rustls_pemfile::certs(&mut CERT.as_ref()).unwrap()[0];
    let key = &rustls_pemfile::pkcs8_private_keys(&mut KEY.as_ref()).unwrap()[0];

    let server_crypto_config = rustls::ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(
            vec![rustls::Certificate(cert.to_vec())], // TODO: load real cert
            rustls::PrivateKey(key.to_vec()),         // TODO: load real key
        )
        .unwrap();
    let udp_socket = std::net::UdpSocket::bind(config.listen).unwrap();
    let server_config = quinn::ServerConfig::with_crypto(Arc::new(server_crypto_config));
    let rt = quinn::default_runtime().unwrap();
    let endpoint_config = quinn::EndpointConfig::default();
    let _quic_listener =
        quinn::Endpoint::new(endpoint_config, Some(server_config), udp_socket, rt).unwrap();
}
