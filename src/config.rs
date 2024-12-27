//! Steward Config
//!
//! See instructions in `commands.rs` to specify the path to your
//! application's configuration file and/or command-line options
//! for specifying it.
use crate::{error::ErrorKind, prelude::APP, somm_send::MAX_GAS_PER_BLOCK};
use abscissa_core::Application;
use deep_space::{Address as CosmosAddress, PrivateKey as CosmosPrivateKey};
use ethers::signers::LocalWallet as EthWallet;
use gravity_bridge::cosmos_gravity;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use signatory::FsKeyStore;
use std::{net::SocketAddr, path::Path};

lazy_static! {
    static ref DELEGATE_KEY: CosmosPrivateKey = {
        let config = APP.config();
        let name = &config.keys.delegate_key;
        config
            .load_deep_space_key(name.clone())
            .expect("failed to load delegate key")
    };
    static ref DELEGATE_ADDRESS: CosmosAddress = {
        let config = APP.config();
        DELEGATE_KEY
            .to_address(&config.cosmos.prefix)
            .expect("failed to derive delegate address from key. make sure the cosmos.prefix field is set in your config.")
    };
}

const GRAVITY_ADDRESS: &str = "0x69592e6f9d21989a043646fe8225da2600e5a0f7";

pub(crate) fn get_delegate_address() -> &'static CosmosAddress {
    &DELEGATE_ADDRESS
}

pub(crate) fn get_delegate_key() -> &'static CosmosPrivateKey {
    &DELEGATE_KEY
}

/// Steward Configuration
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct StewardConfig {
    pub keystore: String,
    pub cosmos: CosmosSection,
    pub ethereum: EthereumSection,
    pub gravity: GravitySection,
    pub keys: KeysConfig,
    pub cork: CorkConfig,
    pub metrics: MetricsSection,
    pub server: ServerSection,
    pub simulate: SimulateSection,
    pub pubsub: PubsubConfig,
}

impl StewardConfig {
    fn load_secret_key(
        &self,
        name: String,
    ) -> Result<k256::elliptic_curve::SecretKey<k256::Secp256k1>, crate::error::Error> {
        let keystore = Path::new(&self.keystore);
        let keystore = match FsKeyStore::open(keystore) {
            Ok(keystore) => keystore,
            Err(signatory::Error::NotADirectory) => {
                return Err(ErrorKind::Config
                    .context("keystore path is not a directory")
                    .into());
            }
            Err(signatory::Error::Permissions) => {
                return Err(ErrorKind::Config
                    .context("insufficient permissions for keystore path")
                    .into());
            }
            Err(e) => {
                return Err(ErrorKind::Config.context(e).into());
            }
        };
        let name = name.parse().expect("Could not parse name");
        let key = keystore.load(&name).expect("Could not load key");
        key.to_pem().parse().map_err(|err| {
            ErrorKind::Config
                .context(format!("failed to parse key {:?}", err))
                .into()
        })
    }

    pub fn load_clarity_key(
        &self,
        name: String,
    ) -> Result<clarity::PrivateKey, crate::error::Error> {
        let key = self.load_secret_key(name)?.to_bytes();
        clarity::PrivateKey::from_slice(key.as_slice()).map_err(|err| {
            ErrorKind::Config
                .context(format!("failed to convert key {:?}", err))
                .into()
        })
    }

    pub fn load_deep_space_key(
        &self,
        name: String,
    ) -> Result<CosmosPrivateKey, crate::error::Error> {
        let key = self.load_secret_key(name)?.to_bytes();
        let key = deep_space::utils::bytes_to_hex_str(key.as_slice());
        key.parse().map_err(|err| {
            ErrorKind::Config
                .context(format!("failed to parse key {:?}", err))
                .into()
        })
    }

    pub fn load_gravity_deep_space_key(
        &self,
        name: String,
    ) -> Result<cosmos_gravity::crypto::PrivateKey, crate::error::Error> {
        let key = self.load_secret_key(name)?.to_bytes();
        let key = deep_space::utils::bytes_to_hex_str(key.as_slice());
        key.parse().map_err(|err| {
            ErrorKind::Config
                .context(format!("failed to parse key {:?}", err))
                .into()
        })
    }

    pub fn load_ethers_wallet(&self, name: String) -> Result<EthWallet, crate::error::Error> {
        Ok(EthWallet::from(self.load_secret_key(name)?))
    }
}

/// Default configuration settings.
///
/// Note: if your needs are as simple as below, you can
/// use `#[derive(Default)]` on StewardConfig instead.
impl Default for StewardConfig {
    fn default() -> Self {
        Self {
            keystore: String::new(),
            cosmos: CosmosSection::default(),
            ethereum: EthereumSection::default(),
            gravity: GravitySection::default(),
            keys: KeysConfig::default(),
            cork: CorkConfig::default(),
            metrics: MetricsSection::default(),
            server: ServerSection::default(),
            simulate: SimulateSection::default(),
            pubsub: PubsubConfig::default(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct ServerSection {
    pub address: String,
    pub port: u16,
    pub server_cert_path: String,
    pub server_key_path: String,
}

impl Default for ServerSection {
    fn default() -> Self {
        Self {
            address: "0.0.0.0".to_string(),
            port: 5734,
            server_cert_path: "".to_owned(),
            server_key_path: "".to_owned(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct KeysConfig {
    pub delegate_key: String,
}

impl Default for KeysConfig {
    fn default() -> Self {
        Self {
            delegate_key: "".to_owned(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct CorkConfig {
    /// Cache refresh period in seconds
    pub cache_refresh_period: u64,
    /// Proposal polling period in seconds
    pub proposal_poll_period: u64,
    /// Number of retries for failed scheduling for proposals
    pub max_scheduling_retries: u64,
}

impl Default for CorkConfig {
    fn default() -> Self {
        Self {
            cache_refresh_period: 60,
            proposal_poll_period: 300,
            max_scheduling_retries: 3,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct PubsubConfig {
    /// Cache refresh period in seconds
    pub cache_refresh_period: u64,
    /// List of publisher domains to reject
    pub publisher_domain_block_list: Vec<String>,
}

impl Default for PubsubConfig {
    fn default() -> Self {
        Self {
            cache_refresh_period: 3600,
            publisher_domain_block_list: Vec::default(),
        }
    }
}

/// EthereumSection for ethereum rpc and derivation path
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct EthereumSection {
    pub blocks_to_search: u64,
    pub gas_price_multiplier: f32,
    pub gas_multiplier: f32,
    pub key_derivation_path: String,
    pub rpc: String,
}

impl Default for EthereumSection {
    fn default() -> Self {
        Self {
            blocks_to_search: 5000,
            gas_price_multiplier: 1.0f32,
            gas_multiplier: 1.0f32,
            key_derivation_path: "m/44'/60'/0'/0/0".to_owned(),
            rpc: "http://localhost:8545".to_owned(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct GravitySection {
    pub contract: String,
    pub fees_denom: String,
}

impl Default for GravitySection {
    fn default() -> Self {
        Self {
            contract: "0x0000000000000000000000000000000000000000".to_owned(),
            fees_denom: "usomm".to_owned(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct CosmosSection {
    pub gas_adjustment: f64,
    pub grpc: String,
    pub key_derivation_path: String,
    pub prefix: String,
    pub msg_batch_size: u32,
    pub gas_limit_per_msg: u64,
    pub gas_price: GasPrice,
}

impl Default for CosmosSection {
    fn default() -> Self {
        Self {
            gas_adjustment: 1.1f64,
            grpc: "https://127.0.0.1:9090".to_owned(),
            key_derivation_path: "m/44'/118'/0'/0/0".to_owned(),
            prefix: "somm".to_owned(),
            msg_batch_size: 5,
            gas_limit_per_msg: MAX_GAS_PER_BLOCK,
            gas_price: GasPrice::default(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct GasPrice {
    pub amount: f64,
    pub denom: String,
}

impl Default for GasPrice {
    fn default() -> Self {
        Self {
            amount: 0.0,
            denom: "usomm".to_owned(),
        }
    }
}

impl GasPrice {
    pub fn as_tuple(&self) -> (f64, String) {
        (self.amount, self.denom.to_owned())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct MetricsSection {
    pub listen_addr: SocketAddr,
}

impl Default for MetricsSection {
    fn default() -> Self {
        Self {
            listen_addr: "127.0.0.1:3000".parse().unwrap(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct SimulateSection {
    pub use_tls: bool,
    pub network_id: String,
    pub tenderly_access_key: String,
    pub tenderly_project_name: String,
    pub tenderly_username: String,
    pub gravity_address: String,
    pub server_cert_path: String,
    pub server_key_path: String,
    pub client_ca_cert_path: String,
}

impl Default for SimulateSection {
    fn default() -> Self {
        Self {
            use_tls: false,
            network_id: "1".to_string(),
            tenderly_access_key: "".to_string(),
            tenderly_project_name: "".to_string(),
            tenderly_username: "".to_string(),
            gravity_address: GRAVITY_ADDRESS.to_string(),
            server_cert_path: "".to_string(),
            server_key_path: "".to_string(),
            client_ca_cert_path: "".to_string(),
        }
    }
}
