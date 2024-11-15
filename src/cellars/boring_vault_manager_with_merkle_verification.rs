use ethers::{abi::AbiEncode, contract::EthCall};
use steward_proto::proto::boring_vault_manager_with_merkle_verification::Function;

use crate::{
    abi::adaptors::boring_vault::manager_with_merkle_verification::{
        ManagerWithMerkleVerificationCalls, PauseCall, SetManageRootCall, UnpauseCall,
    },
    error::{Error, ErrorKind},
    utils::sp_call_parse_address,
};

use super::log_cellar_call;

const CELLAR_NAME: &str = "ManagerWithMerkleVerification";

pub fn get_encoded_call(function: Function, cellar_id: String) -> Result<Vec<u8>, Error> {
    get_call(function, cellar_id).map(|call| call.encode())
}

pub fn get_call(
    function: Function,
    cellar_id: String,
) -> Result<ManagerWithMerkleVerificationCalls, Error> {
    match function {
        Function::SetManageRoot(params) => {
            log_cellar_call(CELLAR_NAME, &SetManageRootCall::function_name(), &cellar_id);

            let strategist = sp_call_parse_address(params.strategist)?;
            let manage_root = hex::decode(params.manage_root)
                .map_err(|e| {
                    ErrorKind::SPCallError.context(format!("failed to decode manage_root: {e}"))
                })?
                .try_into()
                .map_err(|_| {
                    ErrorKind::SPCallError.context("manage_root must be 32 bytes".to_string())
                })?;

            let call = SetManageRootCall {
                strategist,
                manage_root,
            };

            Ok(ManagerWithMerkleVerificationCalls::SetManageRoot(call))
        }
        Function::Pause(_) => {
            log_cellar_call(CELLAR_NAME, &PauseCall::function_name(), &cellar_id);
            let call = PauseCall {};
            Ok(ManagerWithMerkleVerificationCalls::Pause(call))
        }
        Function::Unpause(_) => {
            log_cellar_call(CELLAR_NAME, &UnpauseCall::function_name(), &cellar_id);
            let call = UnpauseCall {};
            Ok(ManagerWithMerkleVerificationCalls::Unpause(call))
        }
    }
}
