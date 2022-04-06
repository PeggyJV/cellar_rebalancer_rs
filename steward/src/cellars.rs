use crate::{
    cork::{cache::{self, is_approved}},
    error::Error,
    gas::CellarGas,
    utils::get_eth_provider,
};
use abscissa_core::tracing::warn;
use ethers::prelude::*;
use std::result::Result;

pub(crate) mod aave_v2_stablecoin;
pub(crate) mod uniswapv3;

pub async fn get_gas_price() -> Result<U256, Error> {
    if std::env::var("ETHERSCAN_API_KEY").is_ok() {
        match CellarGas::etherscan_standard().await {
            Ok(gas) => return Ok(gas),
            Err(err) => {
                warn!("failed to retrieve gas estimate from etherscan: {}", err);
            }
        }
    }

    let provider = get_eth_provider().await?;

    provider.get_gas_price().await.map_err(|r| r.into())
}

pub async fn validate_cellar_id(cellar_id: &str) -> Result<(), String> {
    if let Err(err) = cellar_id.parse::<H160>()  {
        return Err(format!("invalid ethereum address: {}", err));
    }

    if !is_approved(cellar_id).await {
        if let Err(err) = cache::refresh_approved_cellars().await {
            return Err(format!("failed to refresh approved cellar cache while processing SubmitCork request: {}", err));
        }

        if !is_approved(cellar_id).await {
            return Err(format!("cellar ID not approved by governance: {}", cellar_id.to_lowercase()));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use assay::assay;

    #[assay]
    async fn invalid_cellar_id_format_errors() {
        let cellar_id = "thisaintright";
        let result = validate_cellar_id(cellar_id).await;

        assert!(result.is_err())
    }

    #[assay]
    async fn valid_cellar_id_works() {
        let cellar_id = "0x0000000000000000000000000000000000000000";
        let result = validate_cellar_id(cellar_id).await;

        assert!(result.is_ok());
    }
}
