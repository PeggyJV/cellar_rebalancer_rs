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
pub mod error;
pub mod gas;
pub mod prelude;
pub mod server;
pub mod simulate;
pub mod somm_send;
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

        pub mod cellar_adaptor_v1 {
            include!("gen/abi/cellar_adaptor_v1.rs");
        }

        pub mod compound_c_token_adaptor_v2 {
            include!("gen/abi/compound_c_token_adaptor_v2.rs");
        }

        pub mod f_token_adaptor {
            include!("gen/abi/f_token_adaptor.rs");
        }

        pub mod fees_and_reserves_adaptor_v1 {
            include!("gen/abi/fees_and_reserves_adaptor_v1.rs");
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

        pub mod oneinch_adaptor_v1 {
            include!("gen/abi/oneinch_adaptor_v1.rs");
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
    }
}

#[allow(clippy::all)]
pub mod proto {
    include!("gen/proto/steward.v4.rs");
}
