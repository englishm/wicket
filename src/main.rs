mod cli;
use clap::Parser;
use cli::*;
use std::sync::Arc;

fn main() {
    let config = Config::parse();
    let server_crypto_config = rustls::ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(
            vec![rustls::Certificate(vec![0])],
            rustls::PrivateKey(vec![0]),
        )
        .unwrap();
    let udp_socket = std::net::UdpSocket::bind(config.listen).unwrap();
    let server_config = quinn::ServerConfig::with_crypto(Arc::new(server_crypto_config));
    let rt = quinn::default_runtime().unwrap();
    let endpoint_config = quinn::EndpointConfig::default();
    let _quic_listener =
        quinn::Endpoint::new(endpoint_config, Some(server_config), udp_socket, rt).unwrap();
}
