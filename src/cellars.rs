use crate::{
    cork::cache::{self, is_approved},
    error::{Error, ErrorKind},
    utils::{sp_call_error, string_to_u256},
};
use abscissa_core::tracing::info;
use ethers::prelude::*;
use lazy_static::lazy_static;
use std::result::Result;

pub type CachePriceRouterArgs = (bool, u32, Option<String>);

pub(crate) mod aave_v2_stablecoin;
pub(crate) mod adaptors;
pub(crate) mod boring_vault_manager_with_merkle_verification;
pub(crate) mod cellar_v1;
pub(crate) mod cellar_v2;
pub(crate) mod cellar_v2_2;
pub(crate) mod cellar_v2_5;

// constants
// addresses are normalized by removing the 0x prefix and converting to lowercase for reliable comparison

// oracles
//
//
// TurboSteth
pub const TURBOSTETH_ORACLE1: (U256, &str) = (
    U256([24, 0, 0, 0]),
    "762e003b3ab6042a358619e78cba9624beb69b8e",
);
pub const ALLOWED_TURBOSTETH_PRICE_ORACLES: [(U256, &str); 1] = [TURBOSTETH_ORACLE1];

//TurboGHO
pub const TURBOGHO_ORACLE1: (U256, &str) = (
    U256([25, 0, 0, 0]),
    "8e692b7ae230827bc2db30ffa6f3be8ed1a76fc1",
);

pub const ALLOWED_TURBOGHO_PRICE_ORACLES: [(U256, &str); 1] = [TURBOGHO_ORACLE1];

// TurboSweth

pub const TURBOSWETH_ORACLE1: (U256, &str) = (
    U256([3, 0, 0, 0]),
    "72249f0199eacf6230def33a31e80cf76de78f67",
);
pub const TURBOSWETH_ORACLE2: (U256, &str) = (
    U256([5, 0, 0, 0]),
    "c47278b65443ce71cf47e8455bb343f2db11b70e",
);
pub const TURBOSWETH_ORACLE3: (U256, &str) = (
    U256([5, 0, 0, 0]),
    "26cde3f5db92ea91c84c838e664fe42dec1b6747",
);
pub const TURBOSWETH_ORACLE4: (U256, &str) = (
    U256([5, 0, 0, 0]),
    "cb265cac371970e51bec685930e1340fd919fae3",
);
pub const TURBOSWETH_ORACLE5: (U256, &str) = (
    U256([5, 0, 0, 0]),
    "7acdb8096e51b2730387977bad340b9efde61342",
);

pub const TURBOSWETH_ORACLE6: (U256, &str) = (
    U256([26, 0, 0, 0]),
    "0acdb8096e51b2730387977bad340b9efde61342",
);

pub const TEST_RYUSD_ORACLE: (U256, &str) = (
    U256([4, 0, 0, 0]),
    "ddf603866d6d8d207c6200552655df1ebde5a641",
);

pub const ALLOWED_TEST_RYUSD_PRICE_ORACLES: [(U256, &str); 1] = [TEST_RYUSD_ORACLE];

pub const ALLOWED_TURBOSWETH_PRICE_ORACLES: [(U256, &str); 6] = [
    TURBOSWETH_ORACLE1,
    TURBOSWETH_ORACLE2,
    TURBOSWETH_ORACLE3,
    TURBOSWETH_ORACLE4,
    TURBOSWETH_ORACLE5,
    TURBOSWETH_ORACLE6,
];

pub const TURBOSOMM_ORACLE1: (U256, &str) = (
    U256([8, 0, 0, 0]),
    "30510876b377941f23d7322845de0ca734da59e0",
);
pub const TURBOSOMM_ORACLE2: (U256, &str) = (
    U256([8, 0, 0, 0]),
    "84785287f0c9c282462da7927aaed9773b32d9cb",
);

pub const ALLOWED_TURBOSOMM_PRICE_ORACLES: [(U256, &str); 2] =
    [TURBOSOMM_ORACLE1, TURBOSOMM_ORACLE2];

// price routers

// since we're wrapping price router addresses in as Option<T>, it vastly simplifies comparisons for them to be
// owned values (String) vs. a borrowed &str since we have to mutate the passed in values to normalize them.
lazy_static! {
    pub static ref PRICEROUTER1: String = String::from("a1a0bc3d59e4ee5840c9530e49bdc2d1f88aaf92");
    pub static ref PRICEROUTER2: String = String::from("8e46f30b09fdfae6c97db27fecf3304f86dd88c2");

    // mapping of cellar address to allowable arguments for a cachePriceRouter call.
    // args map to params as (checkTotalAssets, allowableRange, expectedPriceRouterAddress). The third
    // param is only present for 2.5 cellars so it's an Option<&str>.
    pub static ref ALLOWED_CACHE_PRICE_ROUTER: [(&'static str, CachePriceRouterArgs); 4] = [
        (CELLAR_RYBTC, (true, 500, None)),
        (CELLAR_RYETH, (true, 500, None)),
        (CELLAR_TURBO_STETH, (false, 500, Some(PRICEROUTER1.clone()))),
        (CELLAR_TURBO_STETH, (true, 500, Some(PRICEROUTER2.clone()))),
    ];
}

// permissions

pub const ALLOWED_V2_0_SETUP_ADAPTORS: [(&str, &str); 0] = [];
pub const ALLOWED_V2_2_CATALOGUE_ADAPTORS: [(&str, &str); 3] = [
    (CELLAR_RYETH, ADAPTOR_AAVE_V3_A_TOKEN_V1_LIDO),
    (CELLAR_RYETH, ADAPTOR_AAVE_V3_DEBT_TOKEN_V1_LIDO),
    (CELLAR_RYETH, ADAPTOR_PENDLE_ADAPTOR_V1),
];
pub const ALLOWED_V2_5_CATALOGUE_ADAPTORS: [(&str, &str); 2] = [
    (CELLAR_TURBO_STETH, ADAPTOR_AAVE_V3_A_TOKEN_V1_LIDO),
    (CELLAR_TURBO_STETH, ADAPTOR_AAVE_V3_DEBT_TOKEN_V1_LIDO),
];

// due to position size limits in v2.0, positions must be added and removed from the limited list
// and thus approved positions need to be allowed to be re-added, hence this large list
pub const ALLOWED_V2_0_POSITIONS: [(&str, u32); 20] = [
    (CELLAR_RYUSD, 1),
    (CELLAR_RYUSD, 2),
    (CELLAR_RYUSD, 3),
    (CELLAR_RYUSD, 13),
    (CELLAR_RYUSD, 14),
    (CELLAR_RYUSD, 15),
    (CELLAR_RYUSD, 16),
    (CELLAR_RYUSD, 17),
    (CELLAR_RYUSD, 18),
    (CELLAR_RYUSD, 19),
    (CELLAR_RYUSD, 20),
    (CELLAR_RYUSD, 21),
    (CELLAR_RYUSD, 22),
    (CELLAR_RYUSD, 23),
    (CELLAR_RYUSD, 24),
    (CELLAR_RYUSD, 25),
    (CELLAR_RYUSD, 26),
    (CELLAR_RYUSD, 27),
    (CELLAR_RYUSD, 28),
    (CELLAR_RYUSD, 29),
];
pub const ALLOWED_V2_2_CATALOGUE_POSITIONS: [(&str, u32); 24] = [
    (CELLAR_RYETH, 202),
    (CELLAR_RYETH, 203),
    (CELLAR_RYETH, 204),
    (CELLAR_RYETH, 205),
    (CELLAR_RYETH, 206),
    (CELLAR_RYETH, 207),
    (CELLAR_RYETH, 208),
    (CELLAR_RYETH, 209),
    (CELLAR_RYETH, 210),
    (CELLAR_RYETH, 211),
    (CELLAR_RYETH, 212),
    (CELLAR_RYETH, 213),
    (CELLAR_RYETH, 214),
    (CELLAR_RYETH, 215),
    (CELLAR_RYETH, 216),
    (CELLAR_RYETH, 217),
    (CELLAR_RYETH, 218),
    (CELLAR_RYETH, 219),
    (CELLAR_RYETH, 220),
    (CELLAR_RYETH, 221),
    (CELLAR_RYETH, 222),
    (CELLAR_RYETH, 223),
    (CELLAR_RYBTC, 224),
    (CELLAR_RYBTC, 225),
];
pub const ALLOWED_V2_5_CATALOGUE_POSITIONS: [(&str, u32); 6] = [
    (CELLAR_TURBO_STETH, 7000),
    (CELLAR_TURBO_STETH, 7001),
    (CELLAR_TURBO_STETH, 7500),
    (CELLAR_TURBO_STETH, 7501),
    (CELLAR_ETH_GROWTH, 33333),
    (CELLAR_ETH_GROWTH, 33334),
];

pub const BLOCKED_ADAPTORS: [&str; 3] = [
    ADAPTOR_UNIV3_V1,
    ADAPTOR_VESTING_SIMPLE_V1,
    ADAPTOR_COMPOUND_C_TOKEN_V1,
];

pub const BLOCKED_V2_0_POSITIONS: [u32; 9] = [4, 5, 6, 7, 8, 9, 10, 11, 12];
pub const BLOCKED_V2_2_POSITIONS: [u32; 0] = [];
pub const BLOCKED_V2_5_POSITIONS: [u32; 0] = [];

pub struct CellarArchPermissions {
    allowed_adaptors: &'static [(&'static str, &'static str)],
    allowed_positions: &'static [(&'static str, u32)],
    blocked_positions: &'static [u32],
}

pub const V2_0_PERMISSIONS: CellarArchPermissions = CellarArchPermissions {
    allowed_adaptors: &ALLOWED_V2_0_SETUP_ADAPTORS,
    allowed_positions: &ALLOWED_V2_0_POSITIONS,
    blocked_positions: &BLOCKED_V2_0_POSITIONS,
};
pub const V2_2_PERMISSIONS: CellarArchPermissions = CellarArchPermissions {
    allowed_adaptors: &ALLOWED_V2_2_CATALOGUE_ADAPTORS,
    allowed_positions: &ALLOWED_V2_2_CATALOGUE_POSITIONS,
    blocked_positions: &BLOCKED_V2_2_POSITIONS,
};
pub const V2_5_PERMISSIONS: CellarArchPermissions = CellarArchPermissions {
    allowed_adaptors: &ALLOWED_V2_5_CATALOGUE_ADAPTORS,
    allowed_positions: &ALLOWED_V2_5_CATALOGUE_POSITIONS,
    blocked_positions: &BLOCKED_V2_5_POSITIONS,
};

// cellars

pub const CELLAR_MORPHO_ETH_MAXIMIZER: &str = "cf4b531b4cde95bd35d71926e09b2b54c564f5b6";
pub const CELLAR_RY1INCH: &str = "c7b69e15d86c5c1581dacce3cacaf5b68cd6596f";
pub const CELLAR_RYBTC: &str = "0274a704a6d9129f90a62ddc6f6024b33ecdad36";
pub const CELLAR_RYENS: &str = "18ea937aba6053bc232d9ae2c42abe7a8a2be440";
pub const CELLAR_RYETH: &str = "b5b29320d2dde5ba5bafa1ebcd270052070483ec";
pub const CELLAR_RYLINK: &str = "4068bdd217a45f8f668ef19f1e3a1f043e4c4934";
pub const CELLAR_RYSNX: &str = "cbf2250f33c4161e18d4a2fa47464520af5216b5";
pub const CELLAR_RYUNI: &str = "6a6af5393dc23d7e3db28d28ef422db7c40932b6";
pub const CELLAR_RYUSD: &str = "97e6e0a40a3d02f12d1cec30ebfbae04e37c119e";
pub const CELLAR_TURBO_SWETH: &str = "d33dad974b938744dac81fe00ac67cb5aa13958e";
pub const CELLAR_TURBO_GHO: &str = "0c190ded9be5f512bd72827bdad4003e9cc7975c";
pub const CELLAR_TURBO_STETH: &str = "fd6db5011b171b05e1ea3b92f9eacaeeb055e971";
pub const CELLAR_TURBO_SOMM: &str = "5195222f69c5821f8095ec565e71e18ab6a2298f";
pub const CELLAR_TURBO_EETH_DEPLOYMENT_1: &str = "9a7b4980c6f0fcaa50cd5f288ad7038f434c692e";
pub const CELLAR_TURBO_EETH_DEPLOYMENT_2: &str = "dadc82e26b3739750e036dfd9defd3ed459b877a";
pub const CELLAR_ETH_GROWTH: &str = "6c51041a91c91c86f3f08a72cb4d3f67f1208897";
pub const CELLAR_TEST_RYUSD: &str = "01a4a3e1e730d245f210eebc6aee54f2381cac63";

// deprecated adaptors

pub const ADAPTOR_UNIV3_V1: &str = "7c4262f83e6775d6ff6fe8d9ab268611ed9d13ee";
pub const ADAPTOR_VESTING_SIMPLE_V1: &str = "1eaa1a100a460f46a2032f0402bc01fe89faab60";
pub const ADAPTOR_COMPOUND_C_TOKEN_V1: &str = "26dba82495f6189dde7648ae88bead46c402f078";

// adaptors

pub const ADAPTOR_AAVE_V3_A_TOKEN_V1: &str = "76cef5606c8b6ba38fe2e3c639e1659afa530b47";
pub const ADAPTOR_AAVE_V3_A_TOKEN_V1_LIDO: &str = "67448b5689012f99db211b516e9fd480d8edfbf3";
pub const ADAPTOR_AAVE_V3_DEBT_TOKEN_V1_LIDO: &str = "7fade03231a694fc12b4ad6098a0f8aa9f732335";
pub const ADAPTOR_AURA_ERC4626_ADAPTOR_V1: &str = "298d97494c5374e796368bcf15f0290771f6ae99";
pub const ADAPTOR_BALANCER_POOL_V1: &str = "2750348a897059c45683d33a1742a3989454f7d6";
pub const ADAPTOR_CELLAR_V2: &str = "3b5ca5de4d808cd793d3a7b3a731d3e67e707b27";
pub const ADAPTOR_COLLATERAL_F_TOKEN_V1: &str = "0055cf6a99eba1405d100c7dfaa88a35521a0037";
pub const ADAPTOR_CONVEX_CURVE_ADAPTOR_V1: &str = "98c44ff447c62364e3750c5e2ef8acc38391a8b0";
pub const ADAPTOR_CURVE_ADAPTOR_V1: &str = "94e28529f73dad189cd0bf9d83a06572d4bfb26a";
pub const ADAPTOR_DEBT_F_TOKEN_V1: &str = "50d8f70a5da95021dab86579db4751a863c1b87c";
pub const ADAPTOR_ERC4626_V1: &str = "411a4e55a4867610279ef35cba4da91a7753a93e";
pub const ADAPTOR_LEGACY_CELLAR_V1: &str = "1e22adf9e63ef8f2a3626841ddddd19683e31068";
pub const ADAPTOR_LIDO_STAKING_V1: &str = "0a64c091d428c7430fe1f47d53f7c8edb1285bce";
pub const ADAPTOR_MORPHO_AAVE_V2_A_TOKEN_V1: &str = "1a4cb53edb8c65c3df6aa9d88c1ab4cf35312b73";
pub const ADAPTOR_MORPHO_AAVE_V2_DEBT_TOKEN_V1: &str = "407d5489f201013ee6a6ca20fccb05047c548138";
pub const ADAPTOR_MORPHO_AAVE_V3_A_TOKEN_COLLATERAL_V1: &str =
    "b46e8a03b1aafffb50f281397c57b5b87080363e";
pub const ADAPTOR_MORPHO_AAVE_V3_DEBT_TOKEN_V1: &str = "25a61f771af9a38c10ddd93c2bbab39a88926fa9";
pub const ADAPTOR_MORPHO_AAVE_V3_P2P_V1: &str = "4fe068caad05b82bf3f86e1f7d1a7b8bbf516111";
pub const ADAPTOR_PENDLE_ADAPTOR_V1: &str = "3ba55a9151cb67ce15bb3ed2d7a6894989626a8a";
pub const ADAPTOR_UNIV3_V3_DEPLOYMENT_1: &str = "92611574ec9bc13c6137917481dab7bb7b173c9b";
pub const ADAPTOR_UNIV3_V3_DEPLOYMENT_2: &str = "c74ffa211a8148949a77ec1070df7013c8d5ce92";
pub const ADAPTOR_VESTING_SIMPLE_V1_1_DEPLOYMENT1: &str =
    "3b98ba00f981342664969e609fb88280704ac479";
pub const ADAPTOR_VESTING_SIMPLE_V1_1_DEPLOYMENT2: &str =
    "8a95bbabb0039480f6dd90fe856c1e0c3d575aa1";

// utils

pub fn log_cellar_call(cellar_name: &str, function_name: &str, cellar_id: &str) {
    info!(
        "encoding {}.{} call for cellar {}",
        cellar_name, function_name, cellar_id
    );
}

pub fn log_governance_cellar_call(
    proposal_id: u64,
    cellar_name: &str,
    function_name: &str,
    cellar_id: &str,
) {
    info!(
        "[Proposal {}]: encoding {}.{} call for cellar {}",
        proposal_id, cellar_name, function_name, cellar_id
    );
}

// since a string prefixed with or without 0x is parsable, ensure the string comparison is valid
pub fn normalize_address(address: String) -> String {
    let lowercase_address = address.to_lowercase();
    return address
        .to_lowercase()
        .strip_prefix("0x")
        .unwrap_or(&lowercase_address)
        .to_string();
}

// validation logic

pub fn validate_new_adaptor(
    cellar_id: &str,
    adaptor_id: &str,
    permissions: &CellarArchPermissions,
) -> Result<(), Error> {
    let adaptor_id = normalize_address(adaptor_id.to_string());
    check_blocked_adaptor(&adaptor_id)?;

    let cellar_id = normalize_address(cellar_id.to_string());
    if !permissions
        .allowed_adaptors
        .contains(&(&cellar_id, &adaptor_id))
    {
        return Err(sp_call_error(format!(
            "new adaptor {adaptor_id} not allowed to be added for cellar {cellar_id}"
        )));
    }

    Ok(())
}

pub fn validate_new_position(
    cellar_id: &str,
    position: u32,
    permissions: &CellarArchPermissions,
) -> Result<(), Error> {
    check_blocked_position(&position, permissions)?;

    let cellar_id = normalize_address(cellar_id.to_string());
    if !permissions
        .allowed_positions
        .contains(&(&cellar_id, position))
    {
        return Err(sp_call_error(format!(
            "new position {position} not allowed to be added for cellar {cellar_id}"
        )));
    }

    Ok(())
}

pub fn validate_oracle(
    cellar_id: &str,
    registry_id_in: &str,
    oracle_in: &str,
) -> Result<(), Error> {
    let cellar_id_normalized = normalize_address(cellar_id.to_string());
    let oracle_in = normalize_address(oracle_in.to_string());
    let registry_id_in = string_to_u256(registry_id_in.to_string())?;
    if cellar_id_normalized.eq(CELLAR_TURBO_SWETH)
        && ALLOWED_TURBOSWETH_PRICE_ORACLES.contains(&(registry_id_in, oracle_in.as_str()))
    {
        return Ok(());
    }
    if cellar_id_normalized.eq(CELLAR_TURBO_SOMM)
        && ALLOWED_TURBOSOMM_PRICE_ORACLES.contains(&(registry_id_in, oracle_in.as_str()))
    {
        return Ok(());
    }

    if cellar_id_normalized.eq(CELLAR_TURBO_STETH)
        && ALLOWED_TURBOSTETH_PRICE_ORACLES.contains(&(registry_id_in, oracle_in.as_str()))
    {
        return Ok(());
    }

    if cellar_id_normalized.eq(CELLAR_TURBO_GHO)
        && ALLOWED_TURBOGHO_PRICE_ORACLES.contains(&(registry_id_in, oracle_in.as_str()))
    {
        return Ok(());
    }

    if cellar_id_normalized.eq(CELLAR_TEST_RYUSD)
        && ALLOWED_TEST_RYUSD_PRICE_ORACLES.contains(&(registry_id_in, oracle_in.as_str()))
    {
        return Ok(());
    }

    Err(sp_call_error("unauthorized oracle update".to_string()))
}

pub fn validate_cache_price_router(
    cellar_id: &str,
    check_total_assets_value: bool,
    allowable_range_value: u32,
    expected_price_router: Option<String>,
) -> Result<(), Error> {
    let cellar_id_normalized = normalize_address(cellar_id.to_string());
    let expected_price_router = expected_price_router.map(normalize_address);

    if !ALLOWED_CACHE_PRICE_ROUTER
        .iter()
        .any(|(cellar_id, permissions)| {
            cellar_id_normalized.eq(cellar_id)
                && permissions.0 == check_total_assets_value
                && permissions.1 >= allowable_range_value
                && permissions.2 == expected_price_router
        })
    {
        return Err(sp_call_error("call not authorized for cellar".to_string()));
    }

    Ok(())
}

pub fn check_blocked_adaptor(adaptor_id: &str) -> Result<(), Error> {
    let adaptor_id = normalize_address(adaptor_id.to_string());
    if BLOCKED_ADAPTORS.contains(&adaptor_id.as_str()) {
        return Err(sp_call_error(format!("adaptor {adaptor_id} is blocked")));
    }

    Ok(())
}

pub fn check_blocked_position(
    position: &u32,
    permissions: &CellarArchPermissions,
) -> Result<(), Error> {
    if permissions.blocked_positions.contains(position) {
        return Err(sp_call_error(format!("position {position} is blocked")));
    }

    Ok(())
}

pub fn is_evm_address(address: &str) -> bool {
    address.parse::<Address>().is_ok()
}

/// Checks that a cellar ID is a valid EVM address and that it is approved by governance. If it is not found in the
/// approved cellar cache initially, we force a cache refresh and check again in case the cellar was approved on-chain
/// since the last automatic refresh.
pub async fn validate_cellar_id(chain_id: u64, cellar_id: &str) -> Result<(), Error> {
    if !is_evm_address(cellar_id) {
        return Err(ErrorKind::InvalidEVMAddress
            .context(format!(
                "invalid cellar ID format {}: must be valid EVM address",
                cellar_id
            ))
            .into());
    }

    let (cellar_id, _) = to_checksum_address(cellar_id)?;

    if !is_approved(chain_id, &cellar_id) {
        if let Err(err) = cache::refresh_approved_cellars().await {
            return Err(ErrorKind::CacheError
                .context(format!("failed to refresh approved cellar cache: {}", err))
                .into());
        }

        if !is_approved(chain_id, &cellar_id) {
            return Err(ErrorKind::UnapprovedCellar
                .context(format!(
                    "cellar ID {} is not approved by governance",
                    cellar_id
                ))
                .into());
        }
    }

    Ok(())
}

pub(crate) fn to_checksum_address(address: &str) -> Result<(String, Vec<u8>), Error> {
    let address = match address.parse::<Address>() {
        Ok(a) => a,
        Err(e) => {
            return Err(ErrorKind::InvalidEVMAddress
                .context(format!("invalid EVM address format ({}): {:?}", address, e))
                .into())
        }
    };
    let address_bytes = address.as_bytes().to_vec();
    let address = ethers::utils::to_checksum(&address, None);

    Ok((address, address_bytes))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_cellar_id_format_errors() {
        let cellar_id = "thisaintright";

        assert!(!is_evm_address(cellar_id));
    }

    #[test]
    fn valid_cellar_id_format_works() {
        let cellar_id = "0x0000000000000000000000000000000000000000";

        assert!(is_evm_address(cellar_id));
    }

    #[test]
    fn test_address_normalization() {
        let blocked1 = String::from("0x7C4262f83e6775D6ff6fE8d9ab268611Ed9d13Ee");
        let blocked2 = String::from("0X7c4262f83e6775d6ff6fe8d9ab268611ed9d13ee");
        let blocked3 = String::from("7C4262f83e6775D6ff6fE8d9ab268611Ed9d13Ee");
        let blocked4 = String::from("7c4262f83e6775d6ff6fe8d9ab268611ed9d13ee");

        assert!(BLOCKED_ADAPTORS.contains(&normalize_address(blocked1.clone()).as_str()));
        assert!(BLOCKED_ADAPTORS.contains(&normalize_address(blocked2).as_str()));
        assert!(BLOCKED_ADAPTORS.contains(&normalize_address(blocked3).as_str()));
        assert!(BLOCKED_ADAPTORS.contains(&normalize_address(blocked4).as_str()));

        // idempotent
        let once = normalize_address(blocked1);
        let twice = normalize_address(once.clone());
        assert_eq!(&once, &twice);
    }

    #[test]
    fn test_check_blocked_position() {
        let error_prefix = "SP call error: ".to_string();

        // allows unblocked position ID
        let unblocked_pos = 25;
        assert!(check_blocked_position(&unblocked_pos, &V2_0_PERMISSIONS).is_ok());
        assert!(check_blocked_position(&unblocked_pos, &V2_2_PERMISSIONS).is_ok());
        assert!(check_blocked_position(&unblocked_pos, &V2_5_PERMISSIONS).is_ok());

        // rejects blocked position ID
        let v2_0_blocked_pos = 4;
        let res = check_blocked_position(&v2_0_blocked_pos, &V2_0_PERMISSIONS);
        let expected_err =
            error_prefix.clone() + &format!("position {v2_0_blocked_pos} is blocked");
        assert!(res.is_err());
        assert_eq!(expected_err, res.unwrap_err().to_string());

        // this position is only blocked on V2.0
        assert!(check_blocked_position(&v2_0_blocked_pos, &V2_2_PERMISSIONS).is_ok());
        assert!(check_blocked_position(&v2_0_blocked_pos, &V2_5_PERMISSIONS).is_ok());
    }

    #[test]
    fn test_check_blocked_adaptor() {
        let error_prefix = "SP call error: ".to_string();

        // allows unblocked adaptor ID
        let unblocked_adaptor = ADAPTOR_MORPHO_AAVE_V2_A_TOKEN_V1;
        assert!(check_blocked_adaptor(&unblocked_adaptor).is_ok());

        // rejects blocked adaptor ID
        let blocked_adaptor = ADAPTOR_UNIV3_V1;
        let res = check_blocked_adaptor(&blocked_adaptor);
        let expected_err = error_prefix.clone() + &format!("adaptor {blocked_adaptor} is blocked");
        assert!(res.is_err());
        assert_eq!(expected_err, res.unwrap_err().to_string());
    }

    #[test]
    fn test_validate_new_adaptor() {
        let error_prefix = "SP call error: ".to_string();

        // allows approved cellar/adaptor ID pairs
        let (v2_0_cellar_id, v2_0_approved_adaptor_id) = (CELLAR_RYUSD, ADAPTOR_CELLAR_V2);
        // assert!(
        //     validate_new_adaptor(v2_0_cellar_id, v2_0_approved_adaptor_id, &V2_0_PERMISSIONS)
        //         .is_ok()
        // );

        // rejects blocked adaptor ID
        let blocked_adaptor_id = ADAPTOR_UNIV3_V1;
        let res = validate_new_adaptor(v2_0_cellar_id, blocked_adaptor_id, &V2_0_PERMISSIONS);
        let expected_err =
            error_prefix.clone() + &format!("adaptor {blocked_adaptor_id} is blocked");
        assert!(res.is_err());
        assert_eq!(expected_err, res.unwrap_err().to_string());

        // rejects unapproved cellar/adaptor ID pair
        let unapproved_adaptor_id = ADAPTOR_UNIV3_V3_DEPLOYMENT_1;
        let res = validate_new_adaptor(v2_0_cellar_id, unapproved_adaptor_id, &V2_0_PERMISSIONS);
        let expected_err = error_prefix.clone()
            + &format!(
                "new adaptor {unapproved_adaptor_id} not allowed to be added for cellar {v2_0_cellar_id}"
            );
        assert!(res.is_err());
        assert_eq!(expected_err, res.unwrap_err().to_string());

        let unapproved_cellar = "0000000000000000000000000000000000000000";
        let res = validate_new_adaptor(
            unapproved_cellar,
            v2_0_approved_adaptor_id,
            &V2_0_PERMISSIONS,
        );
        let expected_err = error_prefix
            + &format!("new adaptor {v2_0_approved_adaptor_id} not allowed to be added for cellar {unapproved_cellar}");
        assert!(res.is_err());
        assert_eq!(expected_err, res.unwrap_err().to_string());
    }

    #[test]
    fn test_validate_new_position() {
        let error_prefix = "SP call error: ".to_string();

        // allows approved cellar/position ID pairs
        let (v2_0_cellar_id, v2_0_approved_pos) = (CELLAR_RYUSD, 1);
        assert!(
            validate_new_position(v2_0_cellar_id, v2_0_approved_pos, &V2_0_PERMISSIONS).is_ok()
        );

        // rejects blocked position ID
        let blocked_pos = 4;
        let res = validate_new_position(v2_0_cellar_id, blocked_pos, &V2_0_PERMISSIONS);
        let expected_err = error_prefix.clone() + &format!("position {blocked_pos} is blocked");
        assert!(res.is_err());
        assert_eq!(expected_err, res.unwrap_err().to_string());

        // rejects unapproved cellar/position ID pair
        let unapproved_pos = 153;
        let res = validate_new_position(v2_0_cellar_id, unapproved_pos, &V2_0_PERMISSIONS);
        let expected_err = error_prefix.clone()
            + &format!(
                "new position {unapproved_pos} not allowed to be added for cellar {v2_0_cellar_id}"
            );
        assert!(res.is_err());
        assert_eq!(expected_err, res.unwrap_err().to_string());

        let unapproved_cellar = "0000000000000000000000000000000000000000";
        let res = validate_new_position(unapproved_cellar, v2_0_approved_pos, &V2_0_PERMISSIONS);
        let expected_err = error_prefix
            + &format!(
                "new position {v2_0_approved_pos} not allowed to be added for cellar {unapproved_cellar}"
            );
        assert!(res.is_err());
        assert_eq!(expected_err, res.unwrap_err().to_string());
    }

    #[test]
    fn test_u256_sanity() {
        assert_eq!(U256([5, 0, 0, 0]), U256::from(5));
    }

    #[test]
    fn test_validate_oracle() {
        assert!(validate_oracle(
            CELLAR_RYBTC,
            &TURBOSWETH_ORACLE1.0.to_string(),
            TURBOSWETH_ORACLE1.1
        )
        .is_err());
        assert!(validate_oracle(
            CELLAR_RYBTC,
            &TURBOSOMM_ORACLE1.0.to_string(),
            TURBOSOMM_ORACLE1.1
        )
        .is_err());
        assert!(validate_oracle(
            CELLAR_TURBO_SWETH,
            &U256::from(6).to_string(),
            TURBOSWETH_ORACLE2.1
        )
        .is_err());
        assert!(validate_oracle(
            CELLAR_TURBO_SOMM,
            &U256::from(6).to_string(),
            TURBOSOMM_ORACLE2.1
        )
        .is_err());
        assert!(validate_oracle(
            CELLAR_TURBO_SWETH,
            &TURBOSWETH_ORACLE1.0.to_string(),
            TURBOSWETH_ORACLE1.1
        )
        .is_ok());
        assert!(validate_oracle(
            CELLAR_TURBO_SWETH,
            &TURBOSWETH_ORACLE2.0.to_string(),
            TURBOSWETH_ORACLE2.1
        )
        .is_ok());
        assert!(validate_oracle(
            CELLAR_TURBO_SWETH,
            &TURBOSWETH_ORACLE3.0.to_string(),
            TURBOSWETH_ORACLE3.1
        )
        .is_ok());
        assert!(validate_oracle(
            CELLAR_TURBO_SOMM,
            &TURBOSOMM_ORACLE1.0.to_string(),
            TURBOSOMM_ORACLE1.1
        )
        .is_ok());
        assert!(validate_oracle(
            CELLAR_TURBO_SOMM,
            &TURBOSOMM_ORACLE2.0.to_string(),
            TURBOSOMM_ORACLE2.1
        )
        .is_ok());
    }

    #[test]
    fn test_validate_cache_price_router() {
        // valid
        assert!(validate_cache_price_router(
            CELLAR_TURBO_STETH,
            true,
            400,
            Some(PRICEROUTER2.clone())
        )
        .is_ok());
        assert!(validate_cache_price_router(
            CELLAR_TURBO_STETH,
            true,
            500,
            Some(PRICEROUTER2.clone().to_uppercase())
        )
        .is_ok());

        // invalid
        assert!(validate_cache_price_router(
            CELLAR_TURBO_STETH,
            true,
            500,
            Some("notreal".to_string())
        )
        .is_err());
        assert!(validate_cache_price_router(
            CELLAR_TURBO_SWETH,
            true,
            500,
            Some(PRICEROUTER2.clone())
        )
        .is_err());
        assert!(validate_cache_price_router(CELLAR_RYETH, false, 500, None).is_err());
        assert!(validate_cache_price_router(CELLAR_RYBTC, true, 600, None).is_err());
    }
}
