use abscissa_core::tracing::log::{debug, error, info, warn};
use ethers::types::H160;
use gravity_bridge::gravity_proto::cosmos_sdk_proto::cosmos::base::abci::v1beta1::TxResponse;
use lazy_static::lazy_static;
use sha3::{Digest, Keccak256};
use somm_proto::{axelar_cork::AxelarCork, cork::Cork};
use tonic::{self, async_trait, Code, Request, Response, Status};

use crate::cellars::to_checksum_address;
use crate::server::handle_authorization;
use crate::{
    cellars::{
        self, aave_v2_stablecoin, boring_vault_manager_with_merkle_verification, cellar_v1,
        cellar_v2, cellar_v2_2, cellar_v2_5,
    },
    error::{
        Error,
        ErrorKind::{self, *},
    },
    proto::{self, schedule_request::CallData::*, ScheduleRequest, ScheduleResponse},
    somm_send,
};

pub mod cache;
pub mod client;
pub mod proposals;

lazy_static! {
    static ref STEWARD_VERSION: &'static str = env!("CARGO_PKG_VERSION");
}

pub(crate) const ETHEREUM_CHAIN_ID: u64 = 1;

pub struct CorkHandler;

#[async_trait]
impl proto::contract_call_service_server::ContractCallService for CorkHandler {
    async fn schedule(
        &self,
        request: Request<ScheduleRequest>,
    ) -> Result<Response<ScheduleResponse>, Status> {
        let certs = request.peer_certs().unwrap();
        let request = request.get_ref().to_owned();

        let chain_id = request.chain_id;
        if chain_id == 0 {
            return Err(Status::new(
                Code::InvalidArgument,
                "chain ID cannot be 0".to_string(),
            ));
        }

        let (cellar_id, cellar_id_bytes) = match to_checksum_address(&request.cellar_id.clone()) {
            Ok(id) => id,
            Err(err) => {
                debug!(
                    "requested cellar ID checksum parse failed: {}",
                    request.cellar_id
                );
                return Err(Status::new(Code::InvalidArgument, err.to_string()));
            }
        };
        if let Err(err) = cellars::validate_cellar_id(chain_id, &cellar_id).await {
            let message = format!("cellar ID validation failed: {}", err);
            match err.kind() {
                CacheError => return Err(Status::new(Code::Internal, message)),
                SPCallError => return Err(Status::new(Code::InvalidArgument, message)),
                UnapprovedCellar => return Err(Status::new(Code::InvalidArgument, message)),
                _ => return Err(Status::new(Code::Unknown, message)),
            }
        }

        let subscription_id = format!("{}:{}", chain_id, cellar_id);
        debug!("checking publisher authorization for {subscription_id}");
        handle_authorization(&subscription_id, certs).await?;

        // Build and send cork
        let height = request.block_height;
        let deadline = request.deadline;
        let encoded_call = match get_encoded_call(request) {
            Ok(c) => c,
            Err(err) => {
                warn!("failed to get encoded call for {}: {}", cellar_id, err);
                return Err(Status::new(Code::InvalidArgument, err.to_string()));
            }
        };

        debug!("hex encoded call: {:?}", hex::encode(&encoded_call));

        if chain_id == ETHEREUM_CHAIN_ID {
            handle_cork(&cellar_id, encoded_call.clone(), height).await?;
        } else {
            handle_axelar_cork(chain_id, &cellar_id, encoded_call.clone(), height, deadline)
                .await?;
        }

        let id = id_hash(height, chain_id, &cellar_id, &encoded_call);
        let invalidation_scope = invalidation_scope(cellar_id_bytes, &encoded_call);
        info!(
            "scheduled cork {} for cellar {} on chain {} at height {}",
            id, cellar_id, chain_id, height
        );

        Ok(Response::new(ScheduleResponse {
            id,
            chain_id,
            invalidation_scope,
        }))
    }
}

pub fn get_encoded_call(request: ScheduleRequest) -> Result<Vec<u8>, Error> {
    if request.call_data.is_none() {
        return Err(ErrorKind::Http.context("empty contract call data").into());
    }

    match request.call_data.unwrap() {
        AaveV2Stablecoin(call) => {
            if call.function.is_none() {
                return Err(ErrorKind::Http.context("empty function data").into());
            }

            aave_v2_stablecoin::get_encoded_call(call.function.unwrap(), request.cellar_id)
        }
        CellarV1(call) => {
            if call.function.is_none() {
                return Err(ErrorKind::Http.context("empty function data").into());
            }

            cellar_v1::get_encoded_call(call.function.unwrap(), request.cellar_id)
        }
        CellarV2(call) => {
            if call.function.is_none() {
                return Err(ErrorKind::Http.context("empty function data").into());
            }

            cellar_v2::get_encoded_call(call.function.unwrap(), request.cellar_id)
        }
        CellarV22(call) => {
            if call.call_type.is_none() {
                return Err(ErrorKind::Http.context("empty function data").into());
            }

            cellar_v2_2::get_encoded_call(call.call_type.unwrap(), request.cellar_id)
        }
        CellarV25(call) => {
            if call.call_type.is_none() {
                return Err(ErrorKind::Http.context("empty function data").into());
            }

            cellar_v2_5::get_encoded_call(call.call_type.unwrap(), request.cellar_id)
        }
        BoringVaultManagerWithMerkleVerification(call) => {
            if call.function.is_none() {
                return Err(ErrorKind::Http.context("empty function data").into());
            }

            boring_vault_manager_with_merkle_verification::get_encoded_call(
                call.function.unwrap(),
                request.cellar_id,
            )
        }
    }
}

async fn handle_cork(cellar_id: &str, encoded_call: Vec<u8>, height: u64) -> Result<(), Status> {
    match schedule_cork(cellar_id, encoded_call.clone(), height).await {
        Ok(res) => {
            if res.code != 0 {
                error!(
                    "failed to schedule cork for cellar {}. code {}, raw log: {}",
                    cellar_id, res.code, res.raw_log
                );
                return Err(Status::new(
                    Code::Internal,
                    "cork submission failed. this may be a steward configuration problem."
                        .to_string(),
                ));
            }

            debug!("cork response: {:?}", res);

            Ok(())
        }
        Err(err) => {
            error!("failed to schedule cork for cellar {}: {}", cellar_id, err);
            Err(Status::new(
                Code::Internal,
                format!("failed to send cork to sommelier: {}", err),
            ))
        }
    }
}

async fn handle_axelar_cork(
    chain_id: u64,
    cellar_id: &str,
    encoded_call: Vec<u8>,
    height: u64,
    deadline: u64,
) -> Result<(), Status> {
    match schedule_axelar_cork(chain_id, cellar_id, encoded_call.clone(), height, deadline).await {
        Ok(res) => {
            if res.code != 0 {
                error!(
                    "failed to schedule axelar cork for cellar {}, chain id {}. code {}, raw log: {}",
                    cellar_id, chain_id, res.code, res.raw_log
                );
                return Err(Status::new(
                    Code::Internal,
                    "axelar cork submission failed. this may be a steward configuration problem."
                        .to_string(),
                ));
            }

            debug!("axelar cork response: {:?}", res);

            Ok(())
        }
        Err(err) => {
            error!("failed to schedule cork for cellar {}: {}", cellar_id, err);
            Err(Status::new(
                Code::Internal,
                format!("failed to send cork to sommelier: {}", err),
            ))
        }
    }
}

pub async fn schedule_cork(
    contract: &str,
    encoded_call: Vec<u8>,
    height: u64,
) -> Result<TxResponse, Error> {
    debug!("establishing grpc connection to cork");
    let cork = Cork {
        encoded_contract_call: encoded_call,
        target_contract_address: contract.to_string(),
    };

    somm_send::schedule_cork(cork, height)
        .await
        .map_err(|e| e.into())
}

pub async fn schedule_axelar_cork(
    chain_id: u64,
    contract: &str,
    encoded_call: Vec<u8>,
    height: u64,
    deadline: u64,
) -> Result<TxResponse, Error> {
    debug!("establishing grpc connection to axelarcork");
    let cork = AxelarCork {
        encoded_contract_call: encoded_call,
        target_contract_address: contract.to_string(),
        chain_id,
        deadline,
    };

    somm_send::schedule_axelar_cork(cork, height)
        .await
        .map_err(|e| e.into())
}

pub fn id_hash(
    block_height: u64,
    chain_id: u64,
    contract_address: &str,
    encoded_call: &[u8],
) -> String {
    let mut hasher = Keccak256::new();
    let address = contract_address
        .parse::<H160>()
        .expect("failed to parse cellar ID. it should have been validated before now.");
    let input = [
        (block_height).to_be_bytes().as_slice(),
        (chain_id).to_be_bytes().as_slice(),
        address.as_bytes(),
        encoded_call,
    ]
    .concat();
    hasher.update(input);

    format!("{:x}", hasher.finalize())
}

// returns the hex encoded invalidation scope of the contract call. this calculation mirrors
// what is done by the cork module when submitting the contract call to gravity.
pub fn invalidation_scope(target_contract_address_bytes: Vec<u8>, encoded_call: &[u8]) -> String {
    let mut hasher = Keccak256::new();
    let input = [&target_contract_address_bytes, encoded_call].concat();

    hasher.update(input);

    format!("{:x}", hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id_hash() {
        let expected =
            "4f954c2a399b67bfc523f08b40c7f94854f6270b0c67fce8f04811d085dc0691".to_string();

        let encoded = "9955a9d400000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000";
        let call = hex::decode(encoded).unwrap();
        let chain_id = 1;
        let height = 35;
        let cellar_id = "0x0165878A594ca255338adfa4d48449f69242Eb8F".to_string();
        let actual = id_hash(height, chain_id, &cellar_id, &call);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_invalidation_scope() {
        let expected = "47c8800f7dd3fa4b8c7a08a30bdcf2c206574760663d30f1fbc78b5f632e2209";
        let actual = invalidation_scope("bleep".as_bytes().to_vec(), "bloop".as_bytes());

        assert_eq!(expected, actual);
    }
}
