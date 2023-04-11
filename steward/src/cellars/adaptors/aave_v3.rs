use abscissa_core::tracing::log::debug;
use ethers::{abi::AbiEncode, types::Bytes};
use steward_abi::{
    aave_v3_a_token_adaptor::AaveV3ATokenAdaptorCalls,
    aave_v3_debt_token_adaptor::{AaveV3DebtTokenAdaptorCalls, FlashLoanCall},
    cellar_v2_2::AdaptorCall as AbiAdaptorCall,
};
use steward_proto::steward::{
    aave_v3_debt_token_adaptor_v1::{
        self, adaptor_call_for_aave_v3_flashloan::CallData::*, AdaptorCallForAaveV3Flashloan,
    },
    aave_v3a_token_adaptor_v1, AaveV3DebtTokenAdaptorV1Calls, AaveV3aTokenAdaptorV1Calls,
};

use crate::{
    cellars::adaptors,
    error::Error,
    utils::{sp_call_error, sp_call_parse_address, string_to_u256},
};

pub(crate) fn aave_v3_a_token_adaptor_v1_call(
    params: AaveV3aTokenAdaptorV1Calls,
) -> Result<Vec<Bytes>, Error> {
    let mut calls = Vec::new();
    for c in params.calls {
        let function = c
            .function
            .ok_or_else(|| sp_call_error("function cannot be empty".to_string()))?;

        match function {
            aave_v3a_token_adaptor_v1::Function::RevokeApproval(p) => {
                let call = steward_abi::aave_v3_a_token_adaptor::RevokeApprovalCall {
                    asset: sp_call_parse_address(p.asset)?,
                    spender: sp_call_parse_address(p.spender)?,
                };
                calls.push(
                    AaveV3ATokenAdaptorCalls::RevokeApproval(call)
                        .encode()
                        .into(),
                )
            }
            aave_v3a_token_adaptor_v1::Function::DepositToAave(p) => {
                let call = steward_abi::aave_v3_a_token_adaptor::DepositToAaveCall {
                    token_to_deposit: sp_call_parse_address(p.token)?,
                    amount_to_deposit: string_to_u256(p.amount)?,
                };
                calls.push(
                    AaveV3ATokenAdaptorCalls::DepositToAave(call)
                        .encode()
                        .into(),
                )
            }
            aave_v3a_token_adaptor_v1::Function::WithdrawFromAave(p) => {
                let call = steward_abi::aave_v3_a_token_adaptor::WithdrawFromAaveCall {
                    token_to_withdraw: sp_call_parse_address(p.token)?,
                    amount_to_withdraw: string_to_u256(p.amount)?,
                };
                calls.push(
                    AaveV3ATokenAdaptorCalls::WithdrawFromAave(call)
                        .encode()
                        .into(),
                )
            }
            aave_v3a_token_adaptor_v1::Function::AdjustIsolationModeAssetAsCollateral(p) => {
                let call = steward_abi::aave_v3_a_token_adaptor::AdjustIsolationModeAssetAsCollateralCall {
                    asset: sp_call_parse_address(p.asset)?,
                    use_as_collateral: p.use_as_collateral,
                };
                calls.push(
                    AaveV3ATokenAdaptorCalls::AdjustIsolationModeAssetAsCollateral(call)
                        .encode()
                        .into(),
                )
            }
            aave_v3a_token_adaptor_v1::Function::ChangeEmode(p) => {
                let call = steward_abi::aave_v3_a_token_adaptor::ChangeEModeCall {
                    category_id: p.category_id as u8,
                };
                calls.push(AaveV3ATokenAdaptorCalls::ChangeEMode(call).encode().into())
            }
        }
    }

    Ok(calls)
}

pub(crate) fn aave_v3_debt_token_adaptor_v1_call(
    params: AaveV3DebtTokenAdaptorV1Calls,
) -> Result<Vec<Bytes>, Error> {
    let mut calls = Vec::new();
    for c in params.calls {
        let function = c
            .function
            .ok_or_else(|| sp_call_error("function cannot be empty".to_string()))?;

        match function {
            aave_v3_debt_token_adaptor_v1::Function::RevokeApproval(p) => {
                let call = steward_abi::aave_v3_debt_token_adaptor::RevokeApprovalCall {
                    asset: sp_call_parse_address(p.asset)?,
                    spender: sp_call_parse_address(p.spender)?,
                };
                calls.push(
                    AaveV3DebtTokenAdaptorCalls::RevokeApproval(call)
                        .encode()
                        .into(),
                )
            }
            aave_v3_debt_token_adaptor_v1::Function::BorrowFromAave(p) => {
                let call = steward_abi::aave_v3_debt_token_adaptor::BorrowFromAaveCall {
                    debt_token_to_borrow: sp_call_parse_address(p.token)?,
                    amount_to_borrow: string_to_u256(p.amount)?,
                };
                calls.push(
                    AaveV3DebtTokenAdaptorCalls::BorrowFromAave(call)
                        .encode()
                        .into(),
                )
            }
            aave_v3_debt_token_adaptor_v1::Function::RepayAaveDebt(p) => {
                let call = steward_abi::aave_v3_debt_token_adaptor::RepayAaveDebtCall {
                    token_to_repay: sp_call_parse_address(p.token)?,
                    amount_to_repay: string_to_u256(p.amount)?,
                };
                calls.push(
                    AaveV3DebtTokenAdaptorCalls::RepayAaveDebt(call)
                        .encode()
                        .into(),
                )
            }
            aave_v3_debt_token_adaptor_v1::Function::FlashLoan(p) => {
                let call = FlashLoanCall {
                    loan_token: p
                        .loan_tokens
                        .iter()
                        .map(|t| sp_call_parse_address(t.clone()))
                        .collect::<Result<Vec<_>, _>>()?,
                    loan_amount: p
                        .loan_amounts
                        .iter()
                        .map(|a| string_to_u256(a.clone()))
                        .collect::<Result<Vec<_>, _>>()?,
                    params: get_encoded_adaptor_call(p.params)?.encode().into(),
                };
                calls.push(AaveV3DebtTokenAdaptorCalls::FlashLoan(call).encode().into())
            }
        }
    }

    Ok(calls)
}

/// Encodes calls to the Adaptor contracts
fn get_encoded_adaptor_call(
    data: Vec<AdaptorCallForAaveV3Flashloan>,
) -> Result<Vec<AbiAdaptorCall>, Error> {
    let mut result: Vec<AbiAdaptorCall> = Vec::new();
    for d in data {
        debug!("adaptor call to {}", d.adaptor);
        let mut calls: Vec<Bytes> = Vec::new();
        let call_data = d
            .call_data
            .ok_or_else(|| sp_call_error("call data is empty".to_string()))?;

        match call_data {
            UniswapV3Calls(params) => {
                calls.extend(adaptors::uniswap_v3::uniswap_v3_adaptor_v1_call(params)?)
            }
            AaveATokenV1Calls(params) => {
                calls.extend(adaptors::aave_v2::aave_a_token_adaptor_v1_call(params)?)
            }
            AaveDebtTokenV1Calls(params) => {
                calls.extend(adaptors::aave_v2::aave_debt_token_adaptor_v1_call(params)?)
            }
            CompoundCTokenV1Calls(params) => {
                calls.extend(adaptors::compound::compound_c_token_v1_call(params)?)
            }
            AaveATokenV2Calls(params) => {
                calls.extend(adaptors::aave_v2::aave_a_token_adaptor_v2_call(params)?)
            }
            AaveDebtTokenV2Calls(params) => {
                calls.extend(adaptors::aave_v2::aave_debt_token_adaptor_v2_call(params)?)
            }
            AaveV3ATokenV1Calls(params) => {
                calls.extend(adaptors::aave_v3::aave_v3_a_token_adaptor_v1_call(params)?)
            }
            AaveV3DebtTokenV1Calls(params) => calls.extend(
                adaptors::aave_v3::aave_v3_debt_token_adaptor_v1_call(params)?,
            ),
            OneInchV1Calls(params) => {
                calls.extend(adaptors::oneinch::one_inch_adaptor_v1_call(params)?)
            }
            FeesAndReservesV1Calls(params) => calls
                .extend(adaptors::fees_and_reserves::fees_and_reserves_adaptor_v1_call(params)?),
            ZeroXV1Calls(params) => calls.extend(adaptors::zero_x::zero_x_adaptor_v1_call(params)?),
            SwapWithUniswapV1Calls(params) => calls.extend(
                adaptors::uniswap_v3::swap_with_uniswap_adaptor_v1_call(params)?,
            ),
            CompoundCTokenV2Calls(params) => {
                calls.extend(adaptors::compound::compound_c_token_v2_call(params)?)
            }
            VestingSimpleCalls(params) => calls.extend(
                adaptors::vesting_simple::vesting_simple_adaptor_v1_call(params)?,
            ),
            CellarCalls(params) => {
                calls.extend(adaptors::sommelier::cellar_adaptor_v1_call(params)?)
            }
        };

        result.push(AbiAdaptorCall {
            adaptor: sp_call_parse_address(d.adaptor)?,
            call_data: calls,
        })
    }

    Ok(result)
}
