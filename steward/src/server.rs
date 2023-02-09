use std::net::SocketAddr;

use crate::{config::StewardConfig, error::Error};
use tonic::transport::{Certificate, Identity, Server, ServerTlsConfig};

pub const DEFAULT_CLIENT_CA: &[u8] = include_bytes!("../../tls/sevenseas_ca.crt");
// for gRPC reflection
pub const DESCRIPTOR: &[u8] = include_bytes!("../../steward_proto_rust/src/prost/descriptor.bin");

pub struct ServerConfig {
    pub tls_config: Option<ServerTlsConfig>,
    pub address: SocketAddr,
}

pub async fn load_server_config(
    config: &std::sync::Arc<StewardConfig>,
) -> Result<ServerConfig, Error> {
    let mut tls_config = None;
    if !config.simulate.enabled || config.simulate.use_tls {
        let cert = tokio::fs::read(&config.server.server_cert_path).await?;
        let key = tokio::fs::read(&config.server.server_key_path).await?;
        let server_identity = Identity::from_pem(cert, key);
        let client_ca = match &config.server.client_ca_cert_path {
            Some(path) => tokio::fs::read(path).await?,
            None => DEFAULT_CLIENT_CA.into(),
        };
        let client_ca_cert = Certificate::from_pem(client_ca);
        tls_config = Some(
            ServerTlsConfig::new()
                .identity(server_identity.clone())
                .client_ca_root(client_ca_cert),
        );
    }

    let port = &config.server.port;
    let address = &config.server.address;
    let address: SocketAddr = format!("{}:{}", address, port).parse()?;

    Ok(ServerConfig {
        tls_config,
        address,
    })
}

pub fn with_tls(builder: Server, tls_config: &ServerTlsConfig) -> Server {
    builder
        .tls_config(tls_config.to_owned())
        .unwrap_or_else(|err| {
            panic!("{:?}", err);
        })
}
