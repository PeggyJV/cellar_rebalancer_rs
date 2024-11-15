//! Steward
//!
//! Application based on the [Abscissa] framework.
//!
//! [Abscissa]: https://github.com/iqlusioninc/abscissa

// Tip: Deny warnings with `RUSTFLAGS="-D warnings"` environment variable in CI

#![forbid(unsafe_code)]
#![warn(trivial_casts)]
#![allow(clippy::uninlined_format_args)]

pub mod application;
pub mod cellars;
pub mod commands;
pub mod config;
pub mod cork;
pub mod encode;
pub mod error;
pub mod gas;
pub mod prelude;
pub mod proposals;
pub mod pubsub;
pub mod server;
pub mod simulate;
pub mod somm_send;
pub mod status;
pub mod tenderly;
pub mod utils;

// Generated ABI definitions. This has to be manually updated when new contracts are added.
#[allow(clippy::all)]
pub mod abi {
    pub mod aave_v2_stablecoin {
        include!("gen/abi/aave_v2_stablecoin.rs");
    }

    pub mod cellar_v1 {
        include!("gen/abi/cellar_v1.rs");
    }

    pub mod cellar_v2 {
        include!("gen/abi/cellar_v2.rs");
    }

    pub mod cellar_v2_2 {
        include!("gen/abi/cellar_v2_2.rs");
    }

    pub mod cellar_v2_5 {
        include!("gen/abi/cellar_v2_5.rs");
    }

    pub mod adaptors {
        pub mod aave_v2_a_token_adaptor_v1 {
            include!("gen/abi/aave_a_token_adaptor_v1.rs");
        }

        pub mod aave_v2_a_token_adaptor_v2 {
            include!("gen/abi/aave_a_token_adaptor_v2.rs");
        }

        pub mod aave_v2_debt_token_adaptor_v1 {
            include!("gen/abi/aave_debt_token_adaptor_v1.rs");
        }

        pub mod aave_v2_debt_token_adaptor_v2 {
            include!("gen/abi/aave_debt_token_adaptor_v2.rs");
        }

        pub mod aave_v2_enable_asset_as_collateral_adaptor_v1 {
            include!("gen/abi/aave_v2_collateral_adaptor_v1.rs");
        }

        pub mod aave_v3_a_token_adaptor_v1 {
            include!("gen/abi/aave_v3_a_token_adaptor_v1.rs");
        }

        pub mod aave_v3_debt_token_adaptor_v1 {
            include!("gen/abi/aave_v3_debt_token_adaptor_v1.rs");
        }

        pub mod aura_erc4626_adaptor_v1 {
            include!("gen/abi/aura_erc4626_adaptor_v1.rs");
        }

        pub mod balancer_pool_adaptor_v1 {
            include!("gen/abi/balancer_pool_adaptor_v1.rs");
        }

        pub mod cellar_adaptor_v1 {
            include!("gen/abi/cellar_adaptor_v1.rs");
        }

        pub mod cellar_with_multi_asset_deposit_v1 {
            include!("gen/abi/cellar_with_multi_asset_deposit_v1.rs");
        }

        pub mod cellar_with_share_lock_period_v1 {
            include!("gen/abi/cellar_with_share_lock_period_v1.rs");
        }

        pub mod compound_c_token_adaptor_v2 {
            include!("gen/abi/compound_c_token_adaptor_v2.rs");
        }

        pub mod convex_curve_adaptor_v1 {
            include!("gen/abi/convex_curve_adaptor_v1.rs");
        }

        pub mod curve_adaptor_v1 {
            include!("gen/abi/curve_adaptor_v1.rs");
        }

        pub mod f_token_adaptor {
            include!("gen/abi/f_token_adaptor.rs");
        }

        pub mod collateral_f_token_adaptor_v1 {
            include!("gen/abi/collateral_f_token_adaptor_v1.rs");
        }

        pub mod debt_f_token_adaptor_v1 {
            include!("gen/abi/debt_f_token_adaptor_v1.rs");
        }

        pub mod erc4626_adaptor_v1 {
            include!("gen/abi/erc4626_adaptor_v1.rs");
        }

        pub mod fees_and_reserves_adaptor_v1 {
            include!("gen/abi/fees_and_reserves_adaptor_v1.rs");
        }

        pub mod legacy_cellar_adaptor_v1 {
            include!("gen/abi/legacy_cellar_adaptor_v1.rs");
        }

        pub mod morpho_aave_v2_a_token_adaptor_v1 {
            include!("gen/abi/morpho_aave_v2_a_token_adaptor_v1.rs");
        }

        pub mod morpho_aave_v2_debt_token_adaptor_v1 {
            include!("gen/abi/morpho_aave_v2_debt_token_adaptor_v1.rs");
        }

        pub mod morpho_aave_v3_a_token_collateral_adaptor_v1 {
            include!("gen/abi/morpho_aave_v3_a_token_collateral_adaptor_v1.rs");
        }

        pub mod morpho_aave_v3_a_token_p2p_adaptor_v1 {
            include!("gen/abi/morpho_aave_v3_a_token_p2p_adaptor_v1.rs");
        }

        pub mod morpho_aave_v3_debt_token_adaptor_v1 {
            include!("gen/abi/morpho_aave_v3_debt_token_adaptor_v1.rs");
        }

        pub mod morpho_blue_collateral_adaptor_v1 {
            include!("gen/abi/morpho_blue_collateral_adaptor_v1.rs");
        }

        pub mod morpho_blue_debt_adaptor_v1 {
            include!("gen/abi/morpho_blue_debt_adaptor_v1.rs");
        }

        pub mod morpho_blue_supply_adaptor_v1 {
            include!("gen/abi/morpho_blue_supply_adaptor_v1.rs");
        }

        pub mod oneinch_adaptor_v1 {
            include!("gen/abi/oneinch_adaptor_v1.rs");
        }

        pub mod pendle_adaptor_v1 {
            include!("gen/abi/pendle_adaptor_v1.rs");
        }

        pub mod staking_adaptor_v1 {
            include!("gen/abi/staking_adaptor_v1.rs");
        }

        pub mod swap_with_uniswap_adaptor_v1 {
            include!("gen/abi/swap_with_uniswap_adaptor_v1.rs");
        }

        pub mod uniswap_v3_adaptor_v2 {
            include!("gen/abi/uniswap_v3_adaptor_v2.rs");
        }

        pub mod vesting_simple_adaptor_v2 {
            include!("gen/abi/vesting_simple_adaptor_v2.rs");
        }

        pub mod zero_x_adaptor_v1 {
            include!("gen/abi/zero_x_adaptor_v1.rs");
        }

        pub mod boring_vault {
            pub mod manager_with_merkle_verification {
                include!("gen/abi/boring_vault/manager_with_merkle_verification.rs");
            }
        }
    }
}

#[allow(clippy::all)]
pub use steward_proto::proto;
