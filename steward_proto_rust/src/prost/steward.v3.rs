///
/// Represents a function call to the Aave V2 Stablecoin cellar
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct AaveV2Stablecoin {
    /// The function you wish to execute on the target cellar
    #[prost(
        oneof = "aave_v2_stablecoin::Function",
        tags = "1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12"
    )]
    pub function: ::core::option::Option<aave_v2_stablecoin::Function>,
}
/// Nested message and enum types in `AaveV2Stablecoin`.
pub mod aave_v2_stablecoin {
    ///
    /// Accrue yield, platform fees, and performance fees..
    ///
    /// Represents function `accrue()`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct Accrue {}
    ///
    /// Claim rewards from Aave and begin cooldown period to unstake them.
    ///
    /// Represents function `claimAndUnstake()`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct ClaimAndUnstake {}
    ///
    /// Pushes total assets into the current Aave lending position.
    ///
    /// Represents function `enterPosition()`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct EnterPosition {}
    ///
    /// Pushes assets into the current Aave lending position.
    ///
    /// Represents function `enterPosition(uint256 assets)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct EnterPositionWithAssets {
        /// amount of assets to enter into the current position
        #[prost(string, tag = "1")]
        pub assets: ::prost::alloc::string::String,
    }
    ///
    /// Pulls total assets from the current Aave lending position.
    ///
    /// Represents function `enterPosition()`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct ExitPosition {}
    ///
    /// Pulls assets from the current Aave lending position.
    ///
    /// Represents function `exitPosition(uint256 assets)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct ExitPositionWithAssets {
        /// amount of assets to exit from the current position
        #[prost(string, tag = "1")]
        pub assets: ::prost::alloc::string::String,
    }
    ///
    /// Rebalances current assets into a new asset position.
    ///
    /// Represents function `rebalance(address newLendingToken, uint256 minNewLendingTokenAmount)`
    ///
    /// This function is based on the Curve Pool Registry exchange_multiple() function:
    /// https://github.com/curvefi/curve-pool-registry/blob/16a8664952cf61d7fed06acca79ad5ac696f4b20/contracts/Swaps.vy#L461-L489
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct Rebalance {
        /// array of [initial token, pool, token, pool, token, ...] that specifies the swap route on Curve.
        #[prost(string, repeated, tag = "1")]
        pub route: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// An array of up to 4 swap params. Attempting more than four swaps will fail.
        #[prost(message, repeated, tag = "2")]
        pub swap_params: ::prost::alloc::vec::Vec<rebalance::SwapParams>,
        /// Minimum acceptable assets to be received from the swap (slippage parameter).  Must be parsable as an unsigned 256-bit integer.
        #[prost(string, tag = "3")]
        pub min_assets_out: ::prost::alloc::string::String,
    }
    /// Nested message and enum types in `Rebalance`.
    pub mod rebalance {
        ///
        /// Represents parameters for a single swap. Each swap needs the indeces in Rebalance.route of the in/out token addresses and the swap type. See the Curve contract linked above for more detail.
        #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
        pub struct SwapParams {
            /// Index in the `route` array of the swap's input token address
            #[prost(uint64, tag = "1")]
            pub in_index: u64,
            /// Index in the `route` array of the swap's output token address
            #[prost(uint64, tag = "2")]
            pub out_index: u64,
            /// 1 - stableswap `exchange`
            /// 2 - stableswap `exchange_underlying`
            /// 3 - cryptoswap `exchange`
            /// 4 - cryptoswap `exchange_underlying`
            /// 5 - Polygon factory metapools `exchange_underlying`
            /// See the Curve Pool Registry exchange_multiple() function for more information.
            #[prost(uint64, tag = "3")]
            pub swap_type: u64,
        }
    }
    ///
    /// Reinvest rewards back into cellar's current position. Must be called within 2 day unstake period 10 days after `claimAndUnstake` was run.
    ///
    /// Represents function `reinvest(uint256 minAssetsOut)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct Reinvest {
        /// Minimum acceptable assets to be received from the swap (slippage parameter).  Must be parsable as an unsigned 256-bit integer.
        #[prost(string, tag = "1")]
        pub min_assets_out: ::prost::alloc::string::String,
    }
    ///
    /// Set the accrual period over which yield is distributed.
    ///
    /// Represents function `setAccrualPeriod(uint32 newAccrualPeriod)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct SetAccrualPeriod {
        #[prost(uint32, tag = "1")]
        pub new_accrual_period: u32,
    }
    ///
    /// Set the per-wallet deposit limit. Uses the same decimals as the current asset.
    ///
    /// Represents function `setDepositLimit(uint256 limit)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct SetDepositLimit {
        /// Amount of assets to set as the new limit. Must be parsable as an unsigned 256-bit integer.
        #[prost(string, tag = "1")]
        pub limit: ::prost::alloc::string::String,
    }
    ///
    /// Set the maximum liquidity that cellar can manage. Uses the same decimals as the current asset.
    ///
    /// Represents function `setLiquidityLimit(uint256 limit)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct SetLiquidityLimit {
        /// Amount of assets to set as the new limit
        #[prost(string, tag = "1")]
        pub limit: ::prost::alloc::string::String,
    }
    ///
    /// Transfer accrued fees to the Sommelier Chain to distribute.
    ///
    /// Represents function `sendFees()`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct SendFees {}
    /// The function you wish to execute on the target cellar
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Oneof)]
    pub enum Function {
        /// Represents function `accruePlatformFees()`
        #[prost(message, tag = "1")]
        Accrue(Accrue),
        /// Represents function `claimAndUnstake()`
        #[prost(message, tag = "2")]
        ClaimAndUnstake(ClaimAndUnstake),
        /// Represents function `enterPosition()`
        #[prost(message, tag = "3")]
        EnterPosition(EnterPosition),
        /// Represents function `enterPosition(uint256 assets)`
        #[prost(message, tag = "4")]
        EnterPositionWithAssets(EnterPositionWithAssets),
        /// Represents function `exitPosition()`
        #[prost(message, tag = "5")]
        ExitPosition(ExitPosition),
        /// Represents function `exitPosition(uint256 assets)`
        #[prost(message, tag = "6")]
        ExitPositionWithAssets(ExitPositionWithAssets),
        /// Represents function `rebalance(address newLendingToken, uint256 minNewLendingTokenAmount)`
        #[prost(message, tag = "7")]
        Rebalance(Rebalance),
        /// Represents function `reinvest(uint256 minAssetsOut)`
        #[prost(message, tag = "8")]
        Reinvest(Reinvest),
        /// Represents function `setAccrualPeriod(uint32 newAccrualPeriod)`
        #[prost(message, tag = "9")]
        SetAccrualPeriod(SetAccrualPeriod),
        /// Represents function `setDepositLimit(uint256 limit)`
        #[prost(message, tag = "10")]
        SetDepositLimit(SetDepositLimit),
        /// Represents function `setLiquidityLimit(uint256 limit)`
        #[prost(message, tag = "11")]
        SetLiquidityLimit(SetLiquidityLimit),
        /// Represents function `transferFees()`
        #[prost(message, tag = "12")]
        SendFees(SendFees),
    }
}
///
/// Represents swap parameters for UniswapV2
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct UniV2SwapParams {
    /// Array of addresses dictating what swap path to follow
    #[prost(string, repeated, tag = "1")]
    pub path: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Amount of the first asset in the path to swap
    #[prost(string, tag = "2")]
    pub amount: ::prost::alloc::string::String,
    /// The minimum amount of the last asset in the path to receive
    #[prost(string, tag = "3")]
    pub amount_out_min: ::prost::alloc::string::String,
}
///
/// Represents swap parameters for UniswapV3
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct UniV3SwapParams {
    /// Array of addresses dictating what swap path to follow
    #[prost(string, repeated, tag = "1")]
    pub path: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Array of pool fees dictating what swap pools to use
    #[prost(uint32, repeated, tag = "2")]
    pub pool_fees: ::prost::alloc::vec::Vec<u32>,
    /// Amount of the first asset in the path to swap
    #[prost(string, tag = "3")]
    pub amount: ::prost::alloc::string::String,
    /// The minimum amount of the last asset in the path to receive
    #[prost(string, tag = "4")]
    pub amount_out_min: ::prost::alloc::string::String,
}
///
/// Represents swap parameters for an exchange
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct SwapParams {
    #[prost(oneof = "swap_params::Params", tags = "1, 2")]
    pub params: ::core::option::Option<swap_params::Params>,
}
/// Nested message and enum types in `SwapParams`.
pub mod swap_params {
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Oneof)]
    pub enum Params {
        /// Params for a Uniswap V2 swap
        #[prost(message, tag = "1")]
        Univ2Params(super::UniV2SwapParams),
        /// Params for a Uniswap V3 swap
        #[prost(message, tag = "2")]
        Univ3Params(super::UniV3SwapParams),
    }
}
///
/// Represents oracle swap parameters for UniswapV2
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct UniV2OracleSwapParams {
    /// Array of addresses dictating what swap path to follow
    #[prost(string, repeated, tag = "1")]
    pub path: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
///
/// Represents oracle swap parameters for UniswapV3
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct UniV3OracleSwapParams {
    /// Array of addresses dictating what swap path to follow
    #[prost(string, repeated, tag = "1")]
    pub path: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Array of pool fees dictating what swap pools to use
    #[prost(uint32, repeated, tag = "2")]
    pub pool_fees: ::prost::alloc::vec::Vec<u32>,
}
///
/// Represents swap params for BaseAdaptor.oracleSwap()
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct OracleSwapParams {
    #[prost(oneof = "oracle_swap_params::Params", tags = "1, 2")]
    pub params: ::core::option::Option<oracle_swap_params::Params>,
}
/// Nested message and enum types in `OracleSwapParams`.
pub mod oracle_swap_params {
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Oneof)]
    pub enum Params {
        #[prost(message, tag = "1")]
        Univ2Params(super::UniV2OracleSwapParams),
        #[prost(message, tag = "2")]
        Univ3Params(super::UniV3OracleSwapParams),
    }
}
///
/// Represents parameters for a Morpho Blue market
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct MarketParams {
    /// The address of the loan token
    #[prost(string, tag = "1")]
    pub loan_token: ::prost::alloc::string::String,
    /// The address of the collateral token
    #[prost(string, tag = "2")]
    pub collateral_token: ::prost::alloc::string::String,
    /// The address of the oracle
    #[prost(string, tag = "3")]
    pub oracle: ::prost::alloc::string::String,
    /// The address of the interest rate model
    #[prost(string, tag = "4")]
    pub irm: ::prost::alloc::string::String,
    /// The loan-to-value ratio
    #[prost(string, tag = "5")]
    pub lltv: ::prost::alloc::string::String,
}
///
/// Exchange selector
#[derive(
    serde::Deserialize,
    serde::Serialize,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    ::prost::Enumeration,
)]
#[repr(i32)]
pub enum Exchange {
    Unspecified = 0,
    /// Represents Uniswap V2
    Univ2 = 1,
    /// Represents Uniswap V3
    Univ3 = 2,
}
///
/// Helper function that allows swaps using the Swap Router
///
/// Represents function `swap(ERC20 assetIn, ERC20 assetOut, uint256 amountIn, SwapRouter.Exchange exchange, bytes memory params)`
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct Swap {
    /// Asset to swap from
    #[prost(string, tag = "1")]
    pub asset_in: ::prost::alloc::string::String,
    /// Asset to swap to
    #[prost(string, tag = "2")]
    pub asset_out: ::prost::alloc::string::String,
    /// Amount to swap
    #[prost(string, tag = "3")]
    pub amount_in: ::prost::alloc::string::String,
    /// The exchange to make the swap on
    #[prost(enumeration = "Exchange", tag = "4")]
    pub exchange: i32,
    /// The parameters for the swap
    #[prost(message, optional, tag = "5")]
    pub params: ::core::option::Option<SwapParams>,
}
///
/// Helper function to make safe "blind" Uniswap Swaps by comparing value in vs value out of the swap.
///
/// Represents function `oracleSwap(ERC20 assetIn, ERC20 assetOut, uint256 amountIn, SwapRouter.Exchange exchange, bytes memory params, uint64 slippage)`
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct OracleSwap {
    /// Asset to swap from
    #[prost(string, tag = "1")]
    pub asset_in: ::prost::alloc::string::String,
    /// Asset to swap to
    #[prost(string, tag = "2")]
    pub asset_out: ::prost::alloc::string::String,
    /// Amount to swap
    #[prost(string, tag = "3")]
    pub amount_in: ::prost::alloc::string::String,
    /// The exchange to make the swap on
    #[prost(enumeration = "Exchange", tag = "4")]
    pub exchange: i32,
    /// The parameters for the swap
    #[prost(message, optional, tag = "5")]
    pub params: ::core::option::Option<OracleSwapParams>,
    /// The slippage allowed for the swap
    #[prost(uint64, tag = "6")]
    pub slippage: u64,
}
///
/// Allows strategists to zero out an approval for a given `asset`.
///
/// Represents function `revokeApproval(ERC20 asset, address spender)`
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct RevokeApproval {
    /// ERC20 Asset to revoke spender's approval for
    #[prost(string, tag = "1")]
    pub asset: ::prost::alloc::string::String,
    /// The spender to revoke approval of asset for
    #[prost(string, tag = "2")]
    pub spender: ::prost::alloc::string::String,
}
/// Represents call data for the Uniswap V3 adaptor
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct SwapWithUniswapAdaptorV1 {
    ///**** BASE ADAPTOR FUNCTIONS ****
    #[prost(oneof = "swap_with_uniswap_adaptor_v1::Function", tags = "1, 2, 3")]
    pub function: ::core::option::Option<swap_with_uniswap_adaptor_v1::Function>,
}
/// Nested message and enum types in `SwapWithUniswapAdaptorV1`.
pub mod swap_with_uniswap_adaptor_v1 {
    ///
    /// Perform a swap using Uniswap V2.
    ///
    /// Represents function `swapWithUniV2(address[] path, uint256 amount, uint256 amountOutMin)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct SwapWithUniV2 {
        #[prost(string, repeated, tag = "1")]
        pub path: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        #[prost(string, tag = "2")]
        pub amount: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub amount_out_min: ::prost::alloc::string::String,
    }
    ///
    /// Perform a swap using Uniswap V3.
    ///
    /// Represents function `Represents function `swapWithUniV3(address[] path, uint24[] poolFees, uint256 amount, uint256 amountOutMin)``
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct SwapWithUniV3 {
        #[prost(string, repeated, tag = "1")]
        pub path: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        #[prost(uint32, repeated, tag = "2")]
        pub pool_fees: ::prost::alloc::vec::Vec<u32>,
        #[prost(string, tag = "3")]
        pub amount: ::prost::alloc::string::String,
        #[prost(string, tag = "4")]
        pub amount_out_min: ::prost::alloc::string::String,
    }
    ///**** BASE ADAPTOR FUNCTIONS ****
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Oneof)]
    pub enum Function {
        /// Represents function `revokeApproval(ERC20 asset, address spender)`
        #[prost(message, tag = "1")]
        RevokeApproval(super::RevokeApproval),
        //**** ADAPTOR-SPECIFIC FUNCTIONS ****
        /// Represents function `swapWithUniV2(address[] path, uint256 amount, uint256 amountOutMin)`
        #[prost(message, tag = "2")]
        SwapWithUniV2(SwapWithUniV2),
        /// Represents function `swapWithUniV3(address[] path, uint24[] poolFees, uint256 amount, uint256 amountOutMin)`
        #[prost(message, tag = "3")]
        SwapWithUniV3(SwapWithUniV3),
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct SwapWithUniswapAdaptorV1Calls {
    #[prost(message, repeated, tag = "1")]
    pub calls: ::prost::alloc::vec::Vec<SwapWithUniswapAdaptorV1>,
}
/// Represents call data for the Uniswap V3 adaptor
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct UniswapV3AdaptorV1 {
    ///**** BASE ADAPTOR FUNCTIONS ****
    #[prost(
        oneof = "uniswap_v3_adaptor_v1::Function",
        tags = "1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11"
    )]
    pub function: ::core::option::Option<uniswap_v3_adaptor_v1::Function>,
}
/// Nested message and enum types in `UniswapV3AdaptorV1`.
pub mod uniswap_v3_adaptor_v1 {
    ///
    /// Allows strategist to open up arbritray Uniswap V3 positions.
    ///
    /// Represents function openPosition(ERC20 token0, ERC20 token1, uint24 poolFee, uint256 amount0, uint256 amount1, uint256 min0, uint256 min1, int24 tickLower, int24 tickUpper)
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct OpenPosition {
        #[prost(string, tag = "1")]
        pub token_0: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub token_1: ::prost::alloc::string::String,
        #[prost(uint32, tag = "3")]
        pub pool_fee: u32,
        #[prost(string, tag = "4")]
        pub amount_0: ::prost::alloc::string::String,
        #[prost(string, tag = "5")]
        pub amount_1: ::prost::alloc::string::String,
        #[prost(string, tag = "6")]
        pub min_0: ::prost::alloc::string::String,
        #[prost(string, tag = "7")]
        pub min_1: ::prost::alloc::string::String,
        #[prost(int32, tag = "8")]
        pub tick_lower: i32,
        #[prost(int32, tag = "9")]
        pub tick_upper: i32,
    }
    ///
    /// Allows strategist to close Uniswap V3 positions.
    ///
    /// Represents function `closePosition(uint256 tokenId, uint256 min0, uint256 min1)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct ClosePosition {
        #[prost(string, tag = "1")]
        pub token_id: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub min_0: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub min_1: ::prost::alloc::string::String,
    }
    ///
    /// Allows strategist to add to existing Uniswap V3 positions.
    ///
    /// Represents function `addToPosition(uint256 tokenId, uint256 amount0, uint256 amount1, uint256 min0, uint256 min1)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct AddToPosition {
        #[prost(string, tag = "1")]
        pub token_id: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub amount_0: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub amount_1: ::prost::alloc::string::String,
        #[prost(string, tag = "4")]
        pub min_0: ::prost::alloc::string::String,
        #[prost(string, tag = "5")]
        pub min_1: ::prost::alloc::string::String,
    }
    ///
    /// Allows strategist to take from existing Uniswap V3 positions.
    ///
    /// Represents function `takeFromPosition(uint256 tokenId, uint128 liquidity, uint256 min0, uint256 min1, bool takeFees)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct TakeFromPosition {
        #[prost(string, tag = "1")]
        pub token_id: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub liquidity: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub min_0: ::prost::alloc::string::String,
        #[prost(string, tag = "4")]
        pub min_1: ::prost::alloc::string::String,
        #[prost(bool, tag = "5")]
        pub take_fees: bool,
    }
    ///
    /// Allows strategist to collect fees from existing Uniswap V3 positions.
    ///
    /// Represents function `collectFees(uint256 tokenId, uint128 amount0, uint128 amount1)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct CollectFees {
        #[prost(string, tag = "1")]
        pub token_id: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub amount_0: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub amount_1: ::prost::alloc::string::String,
    }
    ///
    /// Allows strategist to purge zero liquidity LP positions from tracker.
    ///
    /// Represents function `purgeAllZeroLiquidityPositions(ERC20 token0, ERC20 token1)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct PurgeAllZeroLiquidityPositions {
        #[prost(string, tag = "1")]
        pub token_0: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub token_1: ::prost::alloc::string::String,
    }
    ///
    /// Allows strategist to purge a single zero liquidity LP position from tracker.
    ///
    /// Represents function `purgeSinglePosition(uint256 tokenId)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct PurgeSinglePosition {
        #[prost(string, tag = "1")]
        pub token_id: ::prost::alloc::string::String,
    }
    ///
    /// Allows strategist to remove tracked positions that are not owned by the cellar.
    ///
    /// Represents function `removeUnOwnedPositionFromTracker(uint256 tokenId, ERC20 token0, ERC20 token1)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct RemoveUnownedPositionFromTracker {
        #[prost(string, tag = "1")]
        pub token_id: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub token_0: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub token_1: ::prost::alloc::string::String,
    }
    ///**** BASE ADAPTOR FUNCTIONS ****
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Oneof)]
    pub enum Function {
        /// Represents function `swap(ERC20 assetIn, ERC20 assetOut, uint256 amountIn, SwapRouter.Exchange exchange, bytes memory params)`
        #[prost(message, tag = "1")]
        Swap(super::Swap),
        /// Represents function `oracleSwap(ERC20 assetIn, ERC20 assetOut, uint256 amountIn, SwapRouter.Exchange exchange, bytes memory params, uint64 slippage)`
        #[prost(message, tag = "2")]
        OracleSwap(super::OracleSwap),
        /// Represents function `revokeApproval(ERC20 asset, address spender)`
        #[prost(message, tag = "3")]
        RevokeApproval(super::RevokeApproval),
        //**** ADAPTOR-SPECIFIC FUNCTIONS ****
        /// Represents function `openPosition(ERC20 token0, ERC20 token1, uint24 poolFee, uint256 amount0, uint256 amount1, uint256 min0, uint256 min1, int24 tickLower, int24 tickUpper)`
        #[prost(message, tag = "4")]
        OpenPosition(OpenPosition),
        /// Represents function `closePosition(uint256 positionId, uint256 min0, uint256 min1)`
        #[prost(message, tag = "5")]
        ClosePosition(ClosePosition),
        /// Represents function `addToPosition(uint256 positionId, uint256 amount0, uint256 amount1, uint256 min0, uint256 min1)`
        #[prost(message, tag = "6")]
        AddToPosition(AddToPosition),
        /// Represents function `takeFromPosition(uint256 positionId, uint128 liquidity, uint256 min0, uint256 min1, bool collectFees)`
        #[prost(message, tag = "7")]
        TakeFromPosition(TakeFromPosition),
        /// Represents function `collectFees(uint256 positionId, uint128 amount0, uint128 amount1)`
        #[prost(message, tag = "8")]
        CollectFees(CollectFees),
        /// Represents function `purgeAllZeroLiquidityPositions(ERC20 token0, ERC20 token1)`
        #[prost(message, tag = "9")]
        PurgeAllZeroLiquidityPositions(PurgeAllZeroLiquidityPositions),
        /// Represents function `purgeSinglePosition(uint256 tokenId)`
        #[prost(message, tag = "10")]
        PurgeSinglePosition(PurgeSinglePosition),
        /// Represents function `removeUnOwnedPositionFromTracker(uint256 tokenId, ERC20 token0, ERC20 token1)`
        #[prost(message, tag = "11")]
        RemoveUnownedPositionFromTracker(RemoveUnownedPositionFromTracker),
    }
}
/// Represents call data for the Uniswap V3 adaptor
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct UniswapV3AdaptorV2 {
    ///**** BASE ADAPTOR FUNCTIONS ****
    #[prost(
        oneof = "uniswap_v3_adaptor_v2::Function",
        tags = "1, 2, 3, 4, 5, 6, 7, 8, 9"
    )]
    pub function: ::core::option::Option<uniswap_v3_adaptor_v2::Function>,
}
/// Nested message and enum types in `UniswapV3AdaptorV2`.
pub mod uniswap_v3_adaptor_v2 {
    ///
    /// Allows strategist to open up arbritray Uniswap V3 positions.
    ///
    /// Represents function openPosition(ERC20 token0, ERC20 token1, uint24 poolFee, uint256 amount0, uint256 amount1, uint256 min0, uint256 min1, int24 tickLower, int24 tickUpper)
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct OpenPosition {
        #[prost(string, tag = "1")]
        pub token_0: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub token_1: ::prost::alloc::string::String,
        #[prost(uint32, tag = "3")]
        pub pool_fee: u32,
        #[prost(string, tag = "4")]
        pub amount_0: ::prost::alloc::string::String,
        #[prost(string, tag = "5")]
        pub amount_1: ::prost::alloc::string::String,
        #[prost(string, tag = "6")]
        pub min_0: ::prost::alloc::string::String,
        #[prost(string, tag = "7")]
        pub min_1: ::prost::alloc::string::String,
        #[prost(int32, tag = "8")]
        pub tick_lower: i32,
        #[prost(int32, tag = "9")]
        pub tick_upper: i32,
    }
    ///
    /// Allows strategist to close Uniswap V3 positions.
    ///
    /// Represents function `closePosition(uint256 tokenId, uint256 min0, uint256 min1)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct ClosePosition {
        #[prost(string, tag = "1")]
        pub token_id: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub min_0: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub min_1: ::prost::alloc::string::String,
    }
    ///
    /// Allows strategist to add to existing Uniswap V3 positions.
    ///
    /// Represents function `addToPosition(uint256 tokenId, uint256 amount0, uint256 amount1, uint256 min0, uint256 min1)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct AddToPosition {
        #[prost(string, tag = "1")]
        pub token_id: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub amount_0: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub amount_1: ::prost::alloc::string::String,
        #[prost(string, tag = "4")]
        pub min_0: ::prost::alloc::string::String,
        #[prost(string, tag = "5")]
        pub min_1: ::prost::alloc::string::String,
    }
    ///
    /// Allows strategist to take from existing Uniswap V3 positions.
    ///
    /// Represents function `takeFromPosition(uint256 tokenId, uint128 liquidity, uint256 min0, uint256 min1, bool takeFees)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct TakeFromPosition {
        #[prost(string, tag = "1")]
        pub token_id: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub liquidity: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub min_0: ::prost::alloc::string::String,
        #[prost(string, tag = "4")]
        pub min_1: ::prost::alloc::string::String,
        #[prost(bool, tag = "5")]
        pub take_fees: bool,
    }
    ///
    /// Allows strategist to collect fees from existing Uniswap V3 positions.
    ///
    /// Represents function `collectFees(uint256 tokenId, uint128 amount0, uint128 amount1)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct CollectFees {
        #[prost(string, tag = "1")]
        pub token_id: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub amount_0: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub amount_1: ::prost::alloc::string::String,
    }
    ///
    /// Allows strategist to purge zero liquidity LP positions from tracker.
    ///
    /// Represents function `purgeAllZeroLiquidityPositions(ERC20 token0, ERC20 token1)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct PurgeAllZeroLiquidityPositions {
        #[prost(string, tag = "1")]
        pub token_0: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub token_1: ::prost::alloc::string::String,
    }
    ///
    /// Allows strategist to purge a single zero liquidity LP position from tracker.
    ///
    /// Represents function `purgeSinglePosition(uint256 tokenId)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct PurgeSinglePosition {
        #[prost(string, tag = "1")]
        pub token_id: ::prost::alloc::string::String,
    }
    ///
    /// Allows strategist to remove tracked positions that are not owned by the cellar.
    ///
    /// Represents function `removeUnOwnedPositionFromTracker(uint256 tokenId, ERC20 token0, ERC20 token1)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct RemoveUnownedPositionFromTracker {
        #[prost(string, tag = "1")]
        pub token_id: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub token_0: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub token_1: ::prost::alloc::string::String,
    }
    ///**** BASE ADAPTOR FUNCTIONS ****
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Oneof)]
    pub enum Function {
        /// Represents function `revokeApproval(ERC20 asset, address spender)`
        #[prost(message, tag = "1")]
        RevokeApproval(super::RevokeApproval),
        //**** ADAPTOR-SPECIFIC FUNCTIONS ****
        /// Represents function `openPosition(ERC20 token0, ERC20 token1, uint24 poolFee, uint256 amount0, uint256 amount1, uint256 min0, uint256 min1, int24 tickLower, int24 tickUpper)`
        #[prost(message, tag = "2")]
        OpenPosition(OpenPosition),
        /// Represents function `closePosition(uint256 positionId, uint256 min0, uint256 min1)`
        #[prost(message, tag = "3")]
        ClosePosition(ClosePosition),
        /// Represents function `addToPosition(uint256 positionId, uint256 amount0, uint256 amount1, uint256 min0, uint256 min1)`
        #[prost(message, tag = "4")]
        AddToPosition(AddToPosition),
        /// Represents function `takeFromPosition(uint256 positionId, uint128 liquidity, uint256 min0, uint256 min1, bool collectFees)`
        #[prost(message, tag = "5")]
        TakeFromPosition(TakeFromPosition),
        /// Represents function `collectFees(uint256 positionId, uint128 amount0, uint128 amount1)`
        #[prost(message, tag = "6")]
        CollectFees(CollectFees),
        /// Represents function `purgeAllZeroLiquidityPositions(ERC20 token0, ERC20 token1)`
        #[prost(message, tag = "7")]
        PurgeAllZeroLiquidityPositions(PurgeAllZeroLiquidityPositions),
        /// Represents function `purgeSinglePosition(uint256 tokenId)`
        #[prost(message, tag = "8")]
        PurgeSinglePosition(PurgeSinglePosition),
        /// Represents function `removeUnOwnedPositionFromTracker(uint256 tokenId, ERC20 token0, ERC20 token1)`
        #[prost(message, tag = "9")]
        RemoveUnownedPositionFromTracker(RemoveUnownedPositionFromTracker),
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct UniswapV3AdaptorV1Calls {
    #[prost(message, repeated, tag = "1")]
    pub calls: ::prost::alloc::vec::Vec<UniswapV3AdaptorV1>,
}
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct UniswapV3AdaptorV2Calls {
    #[prost(message, repeated, tag = "1")]
    pub calls: ::prost::alloc::vec::Vec<UniswapV3AdaptorV2>,
}
///
/// Allows Morpho A Token cellars to claim Morpho Rewards
///
/// Represents function `claim(uint256 claimable, bytes32[] memory proof)`
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct Claim {
    /// The amount of the asset to withdraw.
    #[prost(string, tag = "1")]
    pub claimable: ::prost::alloc::string::String,
    /// Proof of claim
    #[prost(bytes = "vec", repeated, tag = "2")]
    pub proof: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// Represents call data for the Morpho Blue Debt adaptor.
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct MorphoBlueDebtAdaptorV1 {
    ///**** BASE ADAPTOR FUNCTIONS ****
    #[prost(oneof = "morpho_blue_debt_adaptor_v1::Function", tags = "1, 2, 3")]
    pub function: ::core::option::Option<morpho_blue_debt_adaptor_v1::Function>,
}
/// Nested message and enum types in `MorphoBlueDebtAdaptorV1`.
pub mod morpho_blue_debt_adaptor_v1 {
    ///
    /// Allows strategists borrow a specific amount of an asset on Morpho Blue
    ///
    /// Represents function `borrowFromMorphoBlue(MarketParams memory _market, uint256 _amountToBorrow)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct BorrowFromMorphoBlue {
        /// Identifier of a Morpho Blue Market
        #[prost(message, optional, tag = "1")]
        pub market: ::core::option::Option<super::MarketParams>,
        /// The amount of the debt token to borrow
        #[prost(string, tag = "2")]
        pub amount_to_borrow: ::prost::alloc::string::String,
    }
    ///
    /// Allows strategists to repay loan debt on Morph Blue Lending Market. Make sure to call addInterest() beforehand to ensure we are repaying what is required.
    ///
    /// Represents function `repayMorphoBlueDebt(MarketParams memory _market, uint256 _debtTokenRepayAmount)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct RepayMorphoBlueDebt {
        /// Identifier of a Morpho Blue Market
        #[prost(message, optional, tag = "1")]
        pub market: ::core::option::Option<super::MarketParams>,
        /// The amount of the debt token to repay
        #[prost(string, tag = "2")]
        pub debt_token_repay_amount: ::prost::alloc::string::String,
    }
    ///**** BASE ADAPTOR FUNCTIONS ****
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Oneof)]
    pub enum Function {
        /// Represents function `revokeApproval(ERC20 asset, address spender)`
        #[prost(message, tag = "1")]
        RevokeApproval(super::RevokeApproval),
        //**** ADAPTOR-SPECIFIC FUNCTIONS ****
        /// Represents function `borrowFromMorphoBlue(MarketParams memory _market, uint256 _amountToBorrow)`
        #[prost(message, tag = "2")]
        BorrowFromMorphoBlue(BorrowFromMorphoBlue),
        /// Represents function `repayMorphoBlueDebt(MarketParams memory _market, uint256 _debtTokenRepayAmount)`
        #[prost(message, tag = "3")]
        RepayMorphoBlueDebt(RepayMorphoBlueDebt),
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct MorphoBlueDebtAdaptorV1Calls {
    #[prost(message, repeated, tag = "1")]
    pub calls: ::prost::alloc::vec::Vec<MorphoBlueDebtAdaptorV1>,
}
/// Represents call data for the Morpho Aave V2 AToken adaptor.
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct MorphoAaveV2aTokenAdaptorV1 {
    ///**** BASE ADAPTOR FUNCTIONS ****
    #[prost(
        oneof = "morpho_aave_v2a_token_adaptor_v1::Function",
        tags = "1, 2, 3, 4"
    )]
    pub function: ::core::option::Option<morpho_aave_v2a_token_adaptor_v1::Function>,
}
/// Nested message and enum types in `MorphoAaveV2ATokenAdaptorV1`.
pub mod morpho_aave_v2a_token_adaptor_v1 {
    ///
    /// Allows strategists to lend assets on Morpho.
    ///
    /// Represents function `depositToAaveV2Morpho(IAaveToken aToken, uint256 amountToDeposit)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct DepositToAaveV2Morpho {
        /// The address of the Aave V2 aToken to deposit to.
        #[prost(string, tag = "1")]
        pub a_token: ::prost::alloc::string::String,
        /// The amount of the asset to deposit.
        #[prost(string, tag = "2")]
        pub amount_to_deposit: ::prost::alloc::string::String,
    }
    ///
    /// Allows strategists to withdraw assets from Morpho.
    ///
    /// Represents function `withdrawFromAaveV2Morpho(IAaveToken aToken, uint256 amountToWithdraw)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct WithdrawFromAaveV2Morpho {
        /// The address of the Aave V2 aToken to withdraw from.
        #[prost(string, tag = "1")]
        pub a_token: ::prost::alloc::string::String,
        /// The amount of the asset to withdraw.
        #[prost(string, tag = "2")]
        pub amount_to_withdraw: ::prost::alloc::string::String,
    }
    ///**** BASE ADAPTOR FUNCTIONS ****
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Oneof)]
    pub enum Function {
        /// Represents function `revokeApproval(ERC20 asset, address spender)`
        #[prost(message, tag = "1")]
        RevokeApproval(super::RevokeApproval),
        //**** ADAPTOR-SPECIFIC FUNCTIONS ****
        /// Represents function `depositToAaveV2Morpho(IAaveToken aToken, uint256 amountToDeposit)`
        #[prost(message, tag = "2")]
        DepositToAaveV2Morpho(DepositToAaveV2Morpho),
        /// Represents function `withdrawFromAaveV2Morpho(IAaveToken aToken, uint256 amountToWithdraw)`
        #[prost(message, tag = "3")]
        WithdrawFromAaveV2Morpho(WithdrawFromAaveV2Morpho),
        /// Represents function `claim(uint256 claimable, bytes32[] memory proof)`
        #[prost(message, tag = "4")]
        Claim(super::Claim),
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct MorphoAaveV2aTokenAdaptorV1Calls {
    #[prost(message, repeated, tag = "1")]
    pub calls: ::prost::alloc::vec::Vec<MorphoAaveV2aTokenAdaptorV1>,
}
/// Represents call data for the Morpho Aave V3 AToken Collateral adaptor.
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct MorphoAaveV3aTokenCollateralAdaptorV1 {
    ///**** BASE ADAPTOR FUNCTIONS ****
    #[prost(
        oneof = "morpho_aave_v3a_token_collateral_adaptor_v1::Function",
        tags = "1, 2, 3, 4"
    )]
    pub function: ::core::option::Option<morpho_aave_v3a_token_collateral_adaptor_v1::Function>,
}
/// Nested message and enum types in `MorphoAaveV3ATokenCollateralAdaptorV1`.
pub mod morpho_aave_v3a_token_collateral_adaptor_v1 {
    ///
    /// Allows strategists to lend assets on Morpho
    ///
    /// Represents function `depositToAaveV3Morpho(ERC20 tokenToDeposit, uint256 amountToDeposit)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct DepositToAaveV3Morpho {
        /// The address of the token to deposit
        #[prost(string, tag = "1")]
        pub token_to_deposit: ::prost::alloc::string::String,
        /// The amount of tokens to deposit
        #[prost(string, tag = "2")]
        pub amount_to_deposit: ::prost::alloc::string::String,
    }
    ///
    /// Allows strategists to withdraw assets from Morpho
    ///
    /// Represents function `withdrawFromAaveV3Morpho(ERC20 tokenToWithdraw, uint256 amountToWithdraw)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct WithdrawFromAaveV3Morpho {
        /// The address of the token to withdraw
        #[prost(string, tag = "1")]
        pub token_to_withdraw: ::prost::alloc::string::String,
        /// The amount of tokens to withdraw
        #[prost(string, tag = "2")]
        pub amount_to_withdraw: ::prost::alloc::string::String,
    }
    ///**** BASE ADAPTOR FUNCTIONS ****
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Oneof)]
    pub enum Function {
        /// Represents function `revokeApproval(ERC20 asset, address spender)`
        #[prost(message, tag = "1")]
        RevokeApproval(super::RevokeApproval),
        //**** ADAPTOR-SPECIFIC FUNCTIONS ****
        /// Represents function `depositToAaveV3Morpho(ERC20 tokenToDeposit, uint256 amountToDeposit)`
        #[prost(message, tag = "2")]
        DepositToAaveV3Morpho(DepositToAaveV3Morpho),
        /// Represents function `withdrawFromAaveV3Morpho(ERC20 tokenToWithdraw, uint256 amountToWithdraw)`
        #[prost(message, tag = "3")]
        WithdrawFromAaveV3Morpho(WithdrawFromAaveV3Morpho),
        /// Represents function `claim(uint256 claimable, bytes32[] memory proof)`
        #[prost(message, tag = "4")]
        Claim(super::Claim),
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct MorphoAaveV3aTokenCollateralAdaptorV1Calls {
    #[prost(message, repeated, tag = "1")]
    pub calls: ::prost::alloc::vec::Vec<MorphoAaveV3aTokenCollateralAdaptorV1>,
}
/// Represents call data for the Morpho Aave V3 A Token P2P adaptor.
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct MorphoAaveV3aTokenP2pAdaptorV1 {
    ///**** BASE ADAPTOR FUNCTIONS ****
    #[prost(
        oneof = "morpho_aave_v3a_token_p2p_adaptor_v1::Function",
        tags = "1, 2, 3, 4"
    )]
    pub function: ::core::option::Option<morpho_aave_v3a_token_p2p_adaptor_v1::Function>,
}
/// Nested message and enum types in `MorphoAaveV3ATokenP2PAdaptorV1`.
pub mod morpho_aave_v3a_token_p2p_adaptor_v1 {
    ///
    /// Allows strategists to lend assets on Morpho
    ///
    /// Represents function `depositToAaveV3Morpho(ERC20 tokenToDeposit, uint256 amountToDeposit, uint256 maxIterations)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct DepositToAaveV3Morpho {
        /// The address of the token to deposit
        #[prost(string, tag = "1")]
        pub token_to_deposit: ::prost::alloc::string::String,
        /// The amount of tokens to deposit
        #[prost(string, tag = "2")]
        pub amount_to_deposit: ::prost::alloc::string::String,
        /// The maximum number of iterations to run
        #[prost(string, tag = "3")]
        pub max_iterations: ::prost::alloc::string::String,
    }
    ///
    /// Allows strategists to withdraw assets from Morpho
    ///
    /// Represents function `withdrawFromAaveV3Morpho(ERC20 tokenToWithdraw, uint256 amountToWithdraw, uint256 maxIterations)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct WithdrawFromAaveV3Morpho {
        /// The address of the token to withdraw
        #[prost(string, tag = "1")]
        pub token_to_withdraw: ::prost::alloc::string::String,
        /// The amount of tokens to withdraw
        #[prost(string, tag = "2")]
        pub amount_to_withdraw: ::prost::alloc::string::String,
        /// The maximum number of iterations to run
        #[prost(string, tag = "3")]
        pub max_iterations: ::prost::alloc::string::String,
    }
    ///**** BASE ADAPTOR FUNCTIONS ****
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Oneof)]
    pub enum Function {
        /// Represents function `revokeApproval(ERC20 asset, address spender)`
        #[prost(message, tag = "1")]
        RevokeApproval(super::RevokeApproval),
        //**** ADAPTOR-SPECIFIC FUNCTIONS ****
        /// Represents function `depositToAaveV3Morpho(ERC20 tokenToDeposit, uint256 amountToDeposit, uint256 maxIterations)`
        #[prost(message, tag = "2")]
        DepositToAaveV3Morpho(DepositToAaveV3Morpho),
        /// Represents function `withdrawFromAaveV3Morpho(ERC20 tokenToWithdraw, uint256 amountToWithdraw, uint256 maxIterations)`
        #[prost(message, tag = "3")]
        WithdrawFromAaveV3Morpho(WithdrawFromAaveV3Morpho),
        /// Represents function `claim(uint256 claimable, bytes32[] memory proof)`
        #[prost(message, tag = "4")]
        Claim(super::Claim),
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct MorphoAaveV3aTokenP2pAdaptorV1Calls {
    #[prost(message, repeated, tag = "1")]
    pub calls: ::prost::alloc::vec::Vec<MorphoAaveV3aTokenP2pAdaptorV1>,
}
/// Represents call data for the Morpho Aave V3 Debt Token adaptor.
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct MorphoAaveV3DebtTokenAdaptorV1 {
    ///**** BASE ADAPTOR FUNCTIONS ****
    #[prost(
        oneof = "morpho_aave_v3_debt_token_adaptor_v1::Function",
        tags = "1, 2, 3"
    )]
    pub function: ::core::option::Option<morpho_aave_v3_debt_token_adaptor_v1::Function>,
}
/// Nested message and enum types in `MorphoAaveV3DebtTokenAdaptorV1`.
pub mod morpho_aave_v3_debt_token_adaptor_v1 {
    ///
    /// Allows strategists to borrow assets from Morpho
    ///
    /// Represents function `borrowFromAaveV3Morpho(address underlying, uint256 amountToBorrow, uint256 maxIterations)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct BorrowFromAaveV3Morpho {
        /// The underlying asset to borrow
        #[prost(string, tag = "1")]
        pub underlying: ::prost::alloc::string::String,
        /// The amount of the underlying asset to borrow
        #[prost(string, tag = "2")]
        pub amount_to_borrow: ::prost::alloc::string::String,
        /// The maximum number of iterations to perform
        #[prost(string, tag = "3")]
        pub max_iterations: ::prost::alloc::string::String,
    }
    ///
    /// Allows strategists to repay loan debt on Morpho
    ///
    /// Represents function `repayAaveV3MorphoDebt(ERC20 tokenToRepay, uint256 amountToRepay)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct RepayAaveV3MorphoDebt {
        /// The token to repay
        #[prost(string, tag = "1")]
        pub token_to_repay: ::prost::alloc::string::String,
        /// The amount of the token to repay
        #[prost(string, tag = "2")]
        pub amount_to_repay: ::prost::alloc::string::String,
    }
    ///**** BASE ADAPTOR FUNCTIONS ****
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Oneof)]
    pub enum Function {
        /// Represents function `revokeApproval(ERC20 asset, address spender)`
        #[prost(message, tag = "1")]
        RevokeApproval(super::RevokeApproval),
        //**** ADAPTOR-SPECIFIC FUNCTIONS ****
        /// Represents function `borrowFromAaveV3Morpho(address underlying, uint256 amountToBorrow, uint256 maxIterations)`
        #[prost(message, tag = "2")]
        BorrowFromAaveV3Morpho(BorrowFromAaveV3Morpho),
        /// Represents function `repayAaveV3MorphoDebt(ERC20 tokenToRepay, uint256 amountToRepay)`
        #[prost(message, tag = "3")]
        RepayAaveV3MorphoDebt(RepayAaveV3MorphoDebt),
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct MorphoAaveV3DebtTokenAdaptorV1Calls {
    #[prost(message, repeated, tag = "1")]
    pub calls: ::prost::alloc::vec::Vec<MorphoAaveV3DebtTokenAdaptorV1>,
}
/// Represents call data for the Morpho Blue Collateral adaptor.
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct MorphoBlueCollateralAdaptorV1 {
    ///**** BASE ADAPTOR FUNCTIONS ****
    #[prost(
        oneof = "morpho_blue_collateral_adaptor_v1::Function",
        tags = "1, 2, 3"
    )]
    pub function: ::core::option::Option<morpho_blue_collateral_adaptor_v1::Function>,
}
/// Nested message and enum types in `MorphoBlueCollateralAdaptorV1`.
pub mod morpho_blue_collateral_adaptor_v1 {
    ///
    /// Allows strategists to add collateral to the respective cellar position on specified MB Market, enabling borrowing.
    ///
    /// Represents function `addCollateral(MarketParams memory _market, uint256 _collateralToDeposit)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct AddCollateral {
        /// Identifier of a Morpho Blue Market
        #[prost(message, optional, tag = "1")]
        pub market: ::core::option::Option<super::MarketParams>,
        /// The amount of collateral to add
        #[prost(string, tag = "2")]
        pub collateral_to_deposit: ::prost::alloc::string::String,
    }
    ///
    /// Allows strategists to remove collateral from the respective cellar position on specified MB Market.
    ///
    /// Represents function `removeCollateral(MarketParams memory _market, uint256 _collateralAmount)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct RemoveCollateral {
        /// Identifier of a Morpho Blue Market
        #[prost(message, optional, tag = "1")]
        pub market: ::core::option::Option<super::MarketParams>,
        /// The amount of collateral to remove
        #[prost(string, tag = "2")]
        pub collateral_amount: ::prost::alloc::string::String,
    }
    ///**** BASE ADAPTOR FUNCTIONS ****
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Oneof)]
    pub enum Function {
        /// Represents function `revokeApproval(ERC20 asset, address spender)`
        #[prost(message, tag = "1")]
        RevokeApproval(super::RevokeApproval),
        //**** ADAPTOR-SPECIFIC FUNCTIONS ****
        /// Represents function `addCollateral(MarketParams memory _market, uint256 _collateralToDeposit)`
        #[prost(message, tag = "2")]
        AddCollateral(AddCollateral),
        /// Represents function `removeCollateral(MarketParams memory _market, uint256 _collateralAmount)`
        #[prost(message, tag = "3")]
        RemoveCollateral(RemoveCollateral),
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct MorphoBlueCollateralAdaptorV1Calls {
    #[prost(message, repeated, tag = "1")]
    pub calls: ::prost::alloc::vec::Vec<MorphoBlueCollateralAdaptorV1>,
}
/// Represents call data for the Morpho Blue Supply adaptor.
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct MorphoBlueSupplyAdaptorV1 {
    ///**** BASE ADAPTOR FUNCTIONS ****
    #[prost(oneof = "morpho_blue_supply_adaptor_v1::Function", tags = "1, 2, 3")]
    pub function: ::core::option::Option<morpho_blue_supply_adaptor_v1::Function>,
}
/// Nested message and enum types in `MorphoBlueSupplyAdaptorV1`.
pub mod morpho_blue_supply_adaptor_v1 {
    ///
    /// Allows strategists to lend a specific amount for an asset on Morpho Blue
    ///
    /// Represents function `lendToMorphoBlue(MarketParams memory _market, uint256 _assets)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct LendToMorphoBlue {
        /// Identifier of a Morpho Blue Market
        #[prost(message, optional, tag = "1")]
        pub market: ::core::option::Option<super::MarketParams>,
        /// The amount of the loan token to lend
        #[prost(string, tag = "2")]
        pub assets: ::prost::alloc::string::String,
    }
    ///
    /// Allows strategists to withdraw the underlying asset plus interest
    ///
    /// Represents function `withdrawFromMorphoBlue(MarketParams memory _market, uint256 _assets)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct WithdrawFromMorphoBlue {
        /// Identifier of a Morpho Blue Market
        #[prost(message, optional, tag = "1")]
        pub market: ::core::option::Option<super::MarketParams>,
        /// The amount of the loan token to lend
        #[prost(string, tag = "2")]
        pub assets: ::prost::alloc::string::String,
    }
    ///**** BASE ADAPTOR FUNCTIONS ****
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Oneof)]
    pub enum Function {
        /// Represents function `revokeApproval(ERC20 asset, address spender)`
        #[prost(message, tag = "1")]
        RevokeApproval(super::RevokeApproval),
        //**** ADAPTOR-SPECIFIC FUNCTIONS ****
        /// Represents function `lendToMorphoBlue(MarketParams memory _market, uint256 _assets)`
        #[prost(message, tag = "2")]
        LendToMorphoBlue(LendToMorphoBlue),
        /// Represents function `withdrawFromMorphoBlue(MarketParams memory _market, uint256 _assets)`
        #[prost(message, tag = "3")]
        WithdrawFromMorphoBlue(WithdrawFromMorphoBlue),
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct MorphoBlueSupplyAdaptorV1Calls {
    #[prost(message, repeated, tag = "1")]
    pub calls: ::prost::alloc::vec::Vec<MorphoBlueSupplyAdaptorV1>,
}
/// Represents call data for the Morpho Aave V2 Debt Token adaptor.
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct MorphoAaveV2DebtTokenAdaptorV1 {
    ///**** BASE ADAPTOR FUNCTIONS ****
    #[prost(
        oneof = "morpho_aave_v2_debt_token_adaptor_v1::Function",
        tags = "1, 2, 3"
    )]
    pub function: ::core::option::Option<morpho_aave_v2_debt_token_adaptor_v1::Function>,
}
/// Nested message and enum types in `MorphoAaveV2DebtTokenAdaptorV1`.
pub mod morpho_aave_v2_debt_token_adaptor_v1 {
    ///
    /// Allows strategists to borrow assets from Aave.
    ///
    /// Represents function `borrowFromAaveV2Morpho(address aToken, uint256 amountToBorrow)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct BorrowFromAaveV2Morpho {
        /// The address of the Aave V2 aToken to borrow.
        #[prost(string, tag = "1")]
        pub a_token: ::prost::alloc::string::String,
        /// The amount of the asset to borrow.
        #[prost(string, tag = "2")]
        pub amount_to_borrow: ::prost::alloc::string::String,
    }
    ///
    /// Allows strategists to repay loan debt on Aave.
    ///
    /// Represents function `repayAaveV2MorphoDebt(IAaveToken aToken, uint256 amountToRepay)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct RepayAaveV2MorphoDebt {
        /// The address of the Aave V2 aToken to repay.
        #[prost(string, tag = "1")]
        pub a_token: ::prost::alloc::string::String,
        /// The amount of the asset to repay.
        #[prost(string, tag = "2")]
        pub amount_to_repay: ::prost::alloc::string::String,
    }
    ///**** BASE ADAPTOR FUNCTIONS ****
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Oneof)]
    pub enum Function {
        /// Represents function `revokeApproval(ERC20 asset, address spender)`
        #[prost(message, tag = "1")]
        RevokeApproval(super::RevokeApproval),
        //**** ADAPTOR-SPECIFIC FUNCTIONS ****
        /// Represents function `borrowFromAaveV2Morpho(address aToken, uint256 amountToBorrow)`
        #[prost(message, tag = "2")]
        BorrowFromAaveV2Morpho(BorrowFromAaveV2Morpho),
        /// Represents function `repayAaveV2MorphoDebt(IAaveToken aToken, uint256 amountToRepay)`
        #[prost(message, tag = "3")]
        RepayAaveV2MorphoDebt(RepayAaveV2MorphoDebt),
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct MorphoAaveV2DebtTokenAdaptorV1Calls {
    #[prost(message, repeated, tag = "1")]
    pub calls: ::prost::alloc::vec::Vec<MorphoAaveV2DebtTokenAdaptorV1>,
}
/// Represents call data for the Frax Collateral F Token adaptor.
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct CollateralFTokenAdaptorV1 {
    ///**** BASE ADAPTOR FUNCTIONS ****
    #[prost(oneof = "collateral_f_token_adaptor_v1::Function", tags = "1, 2, 3")]
    pub function: ::core::option::Option<collateral_f_token_adaptor_v1::Function>,
}
/// Nested message and enum types in `CollateralFTokenAdaptorV1`.
pub mod collateral_f_token_adaptor_v1 {
    ///
    /// Allows strategists to add collateral to the respective cellar position on FraxLend, enabling borrowing.
    ///
    /// Represents function `addCollateral(IFToken _fraxlendPair, uint256 _collateralToDeposit)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct AddCollateral {
        /// The FraxLend pair to add collateral to.
        #[prost(string, tag = "1")]
        pub fraxlend_pair: ::prost::alloc::string::String,
        /// The amount of collateral to add to the cellar position.
        #[prost(string, tag = "2")]
        pub collateral_to_deposit: ::prost::alloc::string::String,
    }
    ///
    /// Allows strategists to remove collateral from the respective cellar position on FraxLend.
    ///
    /// Represents function `removeCollateral(uint256 _collateralAmount, IFToken _fraxlendPair)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct RemoveCollateral {
        /// The amount of collateral to remove from the cellar position.
        #[prost(string, tag = "1")]
        pub collateral_amount: ::prost::alloc::string::String,
        /// The FraxLend pair to remove collateral from.
        #[prost(string, tag = "2")]
        pub fraxlend_pair: ::prost::alloc::string::String,
    }
    ///**** BASE ADAPTOR FUNCTIONS ****
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Oneof)]
    pub enum Function {
        /// Represents function `revokeApproval(ERC20 asset, address spender)`
        #[prost(message, tag = "1")]
        RevokeApproval(super::RevokeApproval),
        //**** ADAPTOR-SPECIFIC FUNCTIONS ****
        /// Represents function `addCollateral(IFToken _fraxlendPair, uint256 _collateralToDeposit)`
        #[prost(message, tag = "2")]
        AddCollateral(AddCollateral),
        /// Represents function `removeCollateral(uint256 _collateralAmount, IFToken _fraxlendPair)`
        #[prost(message, tag = "3")]
        RemoveCollateral(RemoveCollateral),
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct CollateralFTokenAdaptorV1Calls {
    #[prost(message, repeated, tag = "1")]
    pub calls: ::prost::alloc::vec::Vec<CollateralFTokenAdaptorV1>,
}
/// Represents call data for the Frax adaptor.
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct DebtFTokenAdaptorV1 {
    ///**** BASE ADAPTOR FUNCTIONS ****
    #[prost(oneof = "debt_f_token_adaptor_v1::Function", tags = "1, 2, 3, 4")]
    pub function: ::core::option::Option<debt_f_token_adaptor_v1::Function>,
}
/// Nested message and enum types in `DebtFTokenAdaptorV1`.
pub mod debt_f_token_adaptor_v1 {
    ///
    /// Allows a strategist to borrow assets from Fraxlend
    ///
    /// Represents `function borrowFromFraxlend(IFToken fraxlendPair, uint256 amountToBorrow)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct BorrowFromFraxlend {
        /// The address of the Frax Pair to borrow from.
        #[prost(string, tag = "1")]
        pub fraxlend_pair: ::prost::alloc::string::String,
        /// The amount of the asset to borrow.
        #[prost(string, tag = "2")]
        pub amount_to_borrow: ::prost::alloc::string::String,
    }
    ///
    /// Allows strategists to repay loan debt on Fraxlend Pair.
    /// Make sure to call addInterest() beforehand to ensure we are repaying what is required.    
    ///
    /// Represents `function repayFraxlendDebt(IFToken _fraxlendPair, uint256 _debtTokenRepayAmount)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct RepayFraxlendDebt {
        /// The address of the Frax Pair to repay debt on.
        #[prost(string, tag = "1")]
        pub fraxlend_pair: ::prost::alloc::string::String,
        /// The amount of the debt token to repay.
        #[prost(string, tag = "2")]
        pub debt_token_repay_amount: ::prost::alloc::string::String,
    }
    ///
    /// Allows a strategist to call `addInterest` on a Frax Pair they are using
    ///
    /// Represents `function callAddInterest(IFToken _fraxlendPair)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct CallAddInterest {
        /// The address of the pair to call addInterest on.
        #[prost(string, tag = "1")]
        pub fraxlend_pair: ::prost::alloc::string::String,
    }
    ///**** BASE ADAPTOR FUNCTIONS ****
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Oneof)]
    pub enum Function {
        /// Represents function `revokeApproval(ERC20 asset, address spender)`
        #[prost(message, tag = "1")]
        RevokeApproval(super::RevokeApproval),
        //**** ADAPTOR-SPECIFIC FUNCTIONS ****
        /// Represents function `borrowFromFraxlend(IFToken fraxlendPair, uint256 amountToBorrow)`
        #[prost(message, tag = "2")]
        BorrowFromFraxlend(BorrowFromFraxlend),
        /// Represents function `repayFraxlendDebt(IFToken _fraxlendPair, uint256 _debtTokenRepayAmount)`
        #[prost(message, tag = "3")]
        RepayFraxlendDebt(RepayFraxlendDebt),
        /// Represents function `callAddInterest(IFToken _fraxlendPair)`
        #[prost(message, tag = "4")]
        CallAddInterest(CallAddInterest),
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct DebtFTokenAdaptorV1Calls {
    #[prost(message, repeated, tag = "1")]
    pub calls: ::prost::alloc::vec::Vec<DebtFTokenAdaptorV1>,
}
/// Represents call data for the Frax adaptor.
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct FTokenAdaptorV1 {
    ///**** BASE ADAPTOR FUNCTIONS ****
    #[prost(oneof = "f_token_adaptor_v1::Function", tags = "1, 2, 3, 4, 5")]
    pub function: ::core::option::Option<f_token_adaptor_v1::Function>,
}
/// Nested message and enum types in `FTokenAdaptorV1`.
pub mod f_token_adaptor_v1 {
    ///
    /// Allows strategists to lend FRAX on FraxLend
    ///
    /// Represents `function lendFrax(IFToken fToken, uint256 amountToDeposit)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct LendFrax {
        /// The address of the fToken to lend.
        #[prost(string, tag = "1")]
        pub f_token: ::prost::alloc::string::String,
        /// The amount of the fToken to lend.
        #[prost(string, tag = "2")]
        pub amount_to_deposit: ::prost::alloc::string::String,
    }
    ///
    /// Allows strategists to redeem FRAX shares on FraxLend
    ///
    /// Represents `function redeemFraxShare(IFToken fToken, uint256 amountToRedeem)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct RedeemFraxShare {
        /// The address of the fToken to redeem.
        #[prost(string, tag = "1")]
        pub f_token: ::prost::alloc::string::String,
        /// The amount of the fToken to redeem.
        #[prost(string, tag = "2")]
        pub amount_to_redeem: ::prost::alloc::string::String,
    }
    ///
    /// Allows strategists to withdraw FRAX from FraxLend
    ///
    /// Represents `function withdrawFrax(IFToken fToken, uint256 amountToWithdraw)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct WithdrawFrax {
        /// The address of the fToken to withdraw.
        #[prost(string, tag = "1")]
        pub f_token: ::prost::alloc::string::String,
        /// The amount of the fToken to withdraw.
        #[prost(string, tag = "2")]
        pub amount_to_withdraw: ::prost::alloc::string::String,
    }
    ///
    /// Allows a strategist to call `addInterest` on a Frax Pair they are using
    ///
    /// Represents `function callAddInterest(IFToken fToken)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct CallAddInterest {
        /// The address of the fToken to call `addInterest` on.
        #[prost(string, tag = "1")]
        pub f_token: ::prost::alloc::string::String,
    }
    ///**** BASE ADAPTOR FUNCTIONS ****
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Oneof)]
    pub enum Function {
        /// Represents function `revokeApproval(ERC20 asset, address spender)`
        #[prost(message, tag = "1")]
        RevokeApproval(super::RevokeApproval),
        //**** ADAPTOR-SPECIFIC FUNCTIONS ****
        /// Represents function `lendFrax(IFToken fToken, uint256 amountToDeposit)`
        #[prost(message, tag = "2")]
        LendFrax(LendFrax),
        /// Represents function `redeemFraxShare(IFToken fToken, uint256 amountToRedeem)`
        #[prost(message, tag = "3")]
        RedeemFraxShare(RedeemFraxShare),
        /// Represents function `withdrawFrax(IFToken fToken, uint256 amountToWithdraw)`
        #[prost(message, tag = "4")]
        WithdrawFrax(WithdrawFrax),
        /// Represents function `callAddInterest(IFToken fToken)`
        #[prost(message, tag = "5")]
        CallAddInterest(CallAddInterest),
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct FTokenAdaptorV1Calls {
    #[prost(message, repeated, tag = "1")]
    pub calls: ::prost::alloc::vec::Vec<FTokenAdaptorV1>,
}
/// Represents call data for the Curve adaptor V1
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct CurveAdaptorV1 {
    ///**** BASE ADAPTOR FUNCTIONS ****
    #[prost(oneof = "curve_adaptor_v1::Function", tags = "1, 2, 3, 4, 5, 6, 7, 8")]
    pub function: ::core::option::Option<curve_adaptor_v1::Function>,
}
/// Nested message and enum types in `CurveAdaptorV1`.
pub mod curve_adaptor_v1 {
    ///
    /// Allows strategist to add liquidity to Curve pairs that do NOT use the native asset.
    ///
    /// Represents function `addLiquidity(address pool, ERC20 lpToken, uint256[] orderedUnderlyingTokenAmounts, uint256 minLPAmount, CurveGauge gauge, bytes4 selector)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct AddLiquidity {
        /// Address of the Curve pool
        #[prost(string, tag = "1")]
        pub pool: ::prost::alloc::string::String,
        /// Address of the LP token
        #[prost(string, tag = "2")]
        pub lp_token: ::prost::alloc::string::String,
        /// Minimum amount of each underlying token to receive
        #[prost(string, repeated, tag = "3")]
        pub ordered_underlying_token_amounts:
            ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Minimum amount of LP tokens to receive
        #[prost(string, tag = "4")]
        pub min_lp_amount: ::prost::alloc::string::String,
        /// Address of the Curve gauge
        #[prost(string, tag = "5")]
        pub gauge: ::prost::alloc::string::String,
        /// Selector of the function to call
        #[prost(string, tag = "6")]
        pub selector: ::prost::alloc::string::String,
    }
    ///
    /// Allows strategist to add liquidity to Curve pairs that use the native asset.
    ///
    /// Represents function `addLiquidityETH(address pool, ERC20 lpToken, uint256[] orderedUnderlyingTokenAmounts, uint256 minLPAmount, bool useUnderlying, CurveGauge gauge, bytes4 selector)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct AddLiquidityEth {
        /// Address of the Curve pool
        #[prost(string, tag = "1")]
        pub pool: ::prost::alloc::string::String,
        /// Address of the LP token
        #[prost(string, tag = "2")]
        pub lp_token: ::prost::alloc::string::String,
        /// Minimum amount of each underlying token to receive
        #[prost(string, repeated, tag = "3")]
        pub ordered_underlying_token_amounts:
            ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Minimum amount of LP tokens to receive
        #[prost(string, tag = "4")]
        pub min_lp_amount: ::prost::alloc::string::String,
        /// Whether to use the underlying asset or the wrapped asset
        #[prost(bool, tag = "5")]
        pub use_underlying: bool,
        /// Address of the Curve gauge
        #[prost(string, tag = "6")]
        pub gauge: ::prost::alloc::string::String,
        /// Selector of the function to call
        #[prost(string, tag = "7")]
        pub selector: ::prost::alloc::string::String,
    }
    ///
    /// Allows strategist to remove liquidity from Curve pairs that do NOT use the native asset.
    ///
    /// Represents function `removeLiquidity(address pool, ERC20 lpToken, uint256 lpTokenAmount, uint256[] orderedMinimumUnderlyingTokenAmountsOut, CurveGauge gauge, bytes4 selector)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct RemoveLiquidity {
        /// Address of the Curve pool
        #[prost(string, tag = "1")]
        pub pool: ::prost::alloc::string::String,
        /// Address of the LP token
        #[prost(string, tag = "2")]
        pub lp_token: ::prost::alloc::string::String,
        /// Amount of LP tokens to remove
        #[prost(string, tag = "3")]
        pub lp_token_amount: ::prost::alloc::string::String,
        /// Minimum amount of each underlying token to receive
        #[prost(string, repeated, tag = "4")]
        pub ordered_minimum_underlying_token_amounts_out:
            ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Address of the Curve gauge
        #[prost(string, tag = "5")]
        pub gauge: ::prost::alloc::string::String,
        /// Selector of the function to call
        #[prost(string, tag = "6")]
        pub selector: ::prost::alloc::string::String,
    }
    ///
    /// Allows strategist to remove liquidity from Curve pairs that use the native asset.
    ///
    /// Represents function `removeLiquidityETH(address pool, ERC20 lpToken, uint256 lpTokenAmount, uint256[] orderedMinimumUnderlyingTokenAmountsOut, bool useUnderlying, CurveGauge gauge, bytes4 selector)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct RemoveLiquidityEth {
        /// Address of the Curve pool
        #[prost(string, tag = "1")]
        pub pool: ::prost::alloc::string::String,
        /// Address of the LP token
        #[prost(string, tag = "2")]
        pub lp_token: ::prost::alloc::string::String,
        /// Amount of LP tokens to remove
        #[prost(string, tag = "3")]
        pub lp_token_amount: ::prost::alloc::string::String,
        /// Minimum amount of each underlying token to receive
        #[prost(string, repeated, tag = "4")]
        pub ordered_minimum_underlying_token_amounts_out:
            ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Whether to use the underlying asset or the wrapped asset
        #[prost(bool, tag = "5")]
        pub use_underlying: bool,
        /// Address of the Curve gauge
        #[prost(string, tag = "6")]
        pub gauge: ::prost::alloc::string::String,
        /// Selector of the function to call
        #[prost(string, tag = "7")]
        pub selector: ::prost::alloc::string::String,
    }
    ///
    /// Allows strategist to stake Curve LP tokens in their gauge.
    ///
    /// Represents function `stakeInGauge(ERC20 lpToken, CurveGauge gauge, uint256 amount, CurvePool pool, bytes4 selector)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct StakeInGauge {
        /// Address of the LP token
        #[prost(string, tag = "1")]
        pub lp_token: ::prost::alloc::string::String,
        /// Address of the Curve gauge
        #[prost(string, tag = "2")]
        pub gauge: ::prost::alloc::string::String,
        /// Amount of LP tokens to stake
        #[prost(string, tag = "3")]
        pub amount: ::prost::alloc::string::String,
        /// Address of the Curve pool
        #[prost(string, tag = "4")]
        pub pool: ::prost::alloc::string::String,
        /// Selector of the function to call
        #[prost(string, tag = "5")]
        pub selector: ::prost::alloc::string::String,
    }
    ///
    /// Allows strategist to unstake Curve LP tokens from their gauge.
    ///
    /// Represents function `unstakeFromGauge(CurveGauge gauge, uint256 amount)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct UnstakeFromGauge {
        /// Address of the Curve gauge
        #[prost(string, tag = "1")]
        pub gauge: ::prost::alloc::string::String,
        /// Amount of LP tokens to unstake
        #[prost(string, tag = "2")]
        pub amount: ::prost::alloc::string::String,
    }
    ///
    /// Allows strategist to claim rewards from a gauge.
    ///
    /// Represents function `claimRewards(CurveGauge gauge)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct ClaimRewards {
        /// Address of the Curve gauge
        #[prost(string, tag = "1")]
        pub gauge: ::prost::alloc::string::String,
    }
    ///**** BASE ADAPTOR FUNCTIONS ****
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Oneof)]
    pub enum Function {
        /// Represents function `revokeApproval(ERC20 asset, address spender)`
        #[prost(message, tag = "1")]
        RevokeApproval(super::RevokeApproval),
        //**** ADAPTOR-SPECIFIC FUNCTIONS ****
        /// Represents function `addLiquidity(address pool, ERC20 lpToken, uint256[] orderedUnderlyingTokenAmounts, uint256 minLPAmount, CurveGauge gauge, bytes4 selector)`
        #[prost(message, tag = "2")]
        AddLiquidity(AddLiquidity),
        /// Represents function `addLiquidityETH(address pool, ERC20 lpToken, uint256[] orderedMinimumUnderlyingTokenAmountsOut, uint256 minLPAmount, bool useUnderlying, CurveGauge gauge, bytes4 selector)`
        #[prost(message, tag = "3")]
        AddLiquidityEth(AddLiquidityEth),
        /// Represents function `removeLiquidity(address pool, ERC20 lpToken, uint256 lpTokenAmount, uint256[] orderedMinimumUnderlyingTokenAmountsOut, CurveGauge gauge, bytes4 selector)`
        #[prost(message, tag = "4")]
        RemoveLiquidity(RemoveLiquidity),
        /// Represents function `removeLiquidityETH(address pool, ERC20 lpToken, uint256 lpTokenAmount, uint256[] orderedMinimumUnderlyingTokenAmountsOut, bool useUnderlying, CurveGauge gauge, bytes4 selector)`
        #[prost(message, tag = "5")]
        RemoveLiquidityEth(RemoveLiquidityEth),
        /// Represents function `stakeInGauge(ERC20 lpToken, CurveGauge gauge, uint256 amount, CurvePool pool, bytes4 selector)`
        #[prost(message, tag = "6")]
        StakeInGauge(StakeInGauge),
        /// Represents function `unstakeFromGauge(CurveGauge gauge, uint256 amount)`
        #[prost(message, tag = "7")]
        UnstakeFromGauge(UnstakeFromGauge),
        /// Represents function `claimRewards(CurveGauge gauge)`
        #[prost(message, tag = "8")]
        ClaimRewards(ClaimRewards),
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct CurveAdaptorV1Calls {
    #[prost(message, repeated, tag = "1")]
    pub calls: ::prost::alloc::vec::Vec<CurveAdaptorV1>,
}
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct CellarAdaptorV1 {
    ///**** BASE ADAPTOR FUNCTIONS ****
    #[prost(oneof = "cellar_adaptor_v1::Function", tags = "1, 2, 3")]
    pub function: ::core::option::Option<cellar_adaptor_v1::Function>,
}
/// Nested message and enum types in `CellarAdaptorV1`.
pub mod cellar_adaptor_v1 {
    ///
    /// Allows strategists to deposit into Cellar positions.
    ///
    /// Represents function `depositToCellar(Cellar cellar, uint256 assets)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct DepositToCellar {
        #[prost(string, tag = "1")]
        pub cellar: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub assets: ::prost::alloc::string::String,
    }
    ///
    /// Allows strategists to withdraw from Cellar positions.
    ///
    /// Represents function `withdrawFromCellar(Cellar cellar, uint256 assets)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct WithdrawFromCellar {
        #[prost(string, tag = "1")]
        pub cellar: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub assets: ::prost::alloc::string::String,
    }
    ///**** BASE ADAPTOR FUNCTIONS ****
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Oneof)]
    pub enum Function {
        /// Represents function `revokeApproval(ERC20 asset, address spender)`
        #[prost(message, tag = "1")]
        RevokeApproval(super::RevokeApproval),
        //**** ADAPTOR-SPECIFIC FUNCTIONS ****
        /// Represents function `depositToCellar(Cellar cellar, uint256 assets)`
        #[prost(message, tag = "2")]
        DepositToCellar(DepositToCellar),
        /// Represents function `withdrawFromCellar(Cellar cellar, uint256 assets)`
        #[prost(message, tag = "3")]
        WithdrawFromCellar(WithdrawFromCellar),
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct CellarAdaptorV1Calls {
    #[prost(message, repeated, tag = "1")]
    pub calls: ::prost::alloc::vec::Vec<CellarAdaptorV1>,
}
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct LegacyCellarAdaptorV1 {
    ///**** BASE ADAPTOR FUNCTIONS ****
    #[prost(oneof = "legacy_cellar_adaptor_v1::Function", tags = "1, 2, 3")]
    pub function: ::core::option::Option<legacy_cellar_adaptor_v1::Function>,
}
/// Nested message and enum types in `LegacyCellarAdaptorV1`.
pub mod legacy_cellar_adaptor_v1 {
    ///
    /// Allows strategists to deposit into Cellar positions.
    ///
    /// Represents function `depositToCellar(Cellar cellar, uint256 assets, address oracle)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct DepositToCellar {
        #[prost(string, tag = "1")]
        pub cellar: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub assets: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub oracle: ::prost::alloc::string::String,
    }
    ///
    /// Allows strategists to withdraw from Cellar positions.
    ///
    /// Represents function `withdrawFromCellar(Cellar cellar, uint256 assets, address oracle)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct WithdrawFromCellar {
        #[prost(string, tag = "1")]
        pub cellar: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub assets: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub oracle: ::prost::alloc::string::String,
    }
    ///**** BASE ADAPTOR FUNCTIONS ****
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Oneof)]
    pub enum Function {
        /// Represents function `revokeApproval(ERC20 asset, address spender)`
        #[prost(message, tag = "1")]
        RevokeApproval(super::RevokeApproval),
        //**** ADAPTOR-SPECIFIC FUNCTIONS ****
        /// Represents function `depositToCellar(Cellar cellar, uint256 assets, address oracle)`
        #[prost(message, tag = "2")]
        DepositToCellar(DepositToCellar),
        /// Represents function `withdrawFromCellar(Cellar cellar, uint256 assets, address oracle)`
        #[prost(message, tag = "3")]
        WithdrawFromCellar(WithdrawFromCellar),
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct LegacyCellarAdaptorV1Calls {
    #[prost(message, repeated, tag = "1")]
    pub calls: ::prost::alloc::vec::Vec<LegacyCellarAdaptorV1>,
}
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct ZeroXAdaptorV1 {
    ///**** BASE ADAPTOR FUNCTIONS ****
    #[prost(oneof = "zero_x_adaptor_v1::Function", tags = "1, 2")]
    pub function: ::core::option::Option<zero_x_adaptor_v1::Function>,
}
/// Nested message and enum types in `ZeroXAdaptorV1`.
pub mod zero_x_adaptor_v1 {
    ///
    /// Allows strategists to make ERC20 swaps using 0x.
    ///
    /// Represents function `swapWith0x(ERC20 tokenIn, ERC20 tokenOut, uint256 amount, bytes memory swapCallData)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct SwapWith0x {
        #[prost(string, tag = "1")]
        pub token_in: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub token_out: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub amount: ::prost::alloc::string::String,
        #[prost(bytes = "vec", tag = "4")]
        pub swap_call_data: ::prost::alloc::vec::Vec<u8>,
    }
    ///**** BASE ADAPTOR FUNCTIONS ****
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Oneof)]
    pub enum Function {
        /// Represents function `revokeApproval(ERC20 asset, address spender)`
        #[prost(message, tag = "1")]
        RevokeApproval(super::RevokeApproval),
        //**** ADAPTOR-SPECIFIC FUNCTIONS ****
        /// Represents function `swapWith0x(ERC20 tokenIn, ERC20 tokenOut, uint256 amount, bytes memory swapCallData)`
        #[prost(message, tag = "2")]
        SwapWith0x(SwapWith0x),
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct ZeroXAdaptorV1Calls {
    #[prost(message, repeated, tag = "1")]
    pub calls: ::prost::alloc::vec::Vec<ZeroXAdaptorV1>,
}
/// Represents call data for the Convex Curve adaptor V1
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct ConvexCurveAdaptorV1 {
    ///**** BASE ADAPTOR FUNCTIONS ****
    #[prost(oneof = "convex_curve_adaptor_v1::Function", tags = "1, 2, 3, 4")]
    pub function: ::core::option::Option<convex_curve_adaptor_v1::Function>,
}
/// Nested message and enum types in `ConvexCurveAdaptorV1`.
pub mod convex_curve_adaptor_v1 {
    ///
    /// Allows strategists to deposit and stake LPTs into Convex markets via the respective Convex market Booster contract
    ///
    /// Represents function `depositLPTInConvexAndStake(uint256 _pid, address baseRewardPool, ERC20 _lpt, CurvePool _pool, bytes4 _selector, uint256 _amount)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct DepositLptInConvexAndStake {
        #[prost(string, tag = "1")]
        pub pid: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub base_reward_pool: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub lpt: ::prost::alloc::string::String,
        #[prost(string, tag = "4")]
        pub pool: ::prost::alloc::string::String,
        #[prost(string, tag = "5")]
        pub selector: ::prost::alloc::string::String,
        #[prost(string, tag = "6")]
        pub amount_to_deposit: ::prost::alloc::string::String,
    }
    ///
    /// Allows strategists to withdraw from Convex markets via Booster contract w/ or w/o claiming rewards
    ///
    /// Represents function `withdrawFromBaseRewardPoolAsLPTaddress(_baseRewardPool, uint256 _amount, bool _claim)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct WithdrawFromBaseRewardPoolAsLpt {
        #[prost(string, tag = "1")]
        pub base_reward_pool: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub amount_to_withdraw: ::prost::alloc::string::String,
        #[prost(bool, tag = "3")]
        pub claim: bool,
    }
    ///
    /// Allows strategists to get rewards for an Convex Booster without withdrawing/unwrapping from Convex market
    ///
    /// Represents function `getRewards(address _baseRewardPool, bool _claimExtras)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct GetRewards {
        #[prost(string, tag = "1")]
        pub base_reward_pool: ::prost::alloc::string::String,
        #[prost(bool, tag = "2")]
        pub claim_extras: bool,
    }
    ///**** BASE ADAPTOR FUNCTIONS ****
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Oneof)]
    pub enum Function {
        /// Represents function `revokeApproval(ERC20 asset, address spender)`
        #[prost(message, tag = "1")]
        RevokeApproval(super::RevokeApproval),
        //**** ADAPTOR-SPECIFIC FUNCTIONS ****
        /// Represents function `depositLPTInConvexAndStake(uint256 _pid, address baseRewardPool, ERC20 _lpt, CurvePool _pool, bytes4 _selector, uint256 _amount)`
        #[prost(message, tag = "2")]
        DepositLptInConvexAndStake(DepositLptInConvexAndStake),
        /// Represents function `withdrawFromBaseRewardPoolAsLPTaddress(_baseRewardPool, uint256 _amount, bool _claim)`
        #[prost(message, tag = "3")]
        WithdrawFromBaseRewardPoolAsLpt(WithdrawFromBaseRewardPoolAsLpt),
        /// Represents function `getRewards(address _baseRewardPool, bool _claimExtras)`
        #[prost(message, tag = "4")]
        GetRewards(GetRewards),
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct ConvexCurveAdaptorV1Calls {
    #[prost(message, repeated, tag = "1")]
    pub calls: ::prost::alloc::vec::Vec<ConvexCurveAdaptorV1>,
}
/// Represents call data for the Vesting Simple adaptor
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct VestingSimpleAdaptorV2 {
    ///**** BASE ADAPTOR FUNCTIONS ****
    #[prost(oneof = "vesting_simple_adaptor_v2::Function", tags = "1, 2, 3, 4, 5")]
    pub function: ::core::option::Option<vesting_simple_adaptor_v2::Function>,
}
/// Nested message and enum types in `VestingSimpleAdaptorV2`.
pub mod vesting_simple_adaptor_v2 {
    ///
    /// Allows strategists to deposit tokens to the vesting contract. By passing a max uint256 for amountToDeposit, the cellar will
    /// deposit its entire balance (appropriate in most cases).
    ///
    /// Represents function `depositToVesting(VestingSimple vestingContract, uint256 amountToDeposit)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct DepositToVesting {
        #[prost(string, tag = "1")]
        pub vesting_contract: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub amount: ::prost::alloc::string::String,
    }
    ///
    /// Allows strategists to deposit tokens to the vesting contract. By passing a max uint256 for amountToDeposit, the cellar will
    /// deposit its entire balance (appropriate in most cases).
    ///
    /// Represents function `withdrawFromVesting(VestingSimple vestingContract, uint256 depositId, uint256 amountToWithdraw)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct WithdrawFromVesting {
        #[prost(string, tag = "1")]
        pub vesting_contract: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub deposit_id: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub amount: ::prost::alloc::string::String,
    }
    ///
    /// Withdraw a single deposit from vesting. This will not affect the cellar's TVL because any deposit must already have vested, and
    /// will be reported in balanceOf. Will revert if not enough tokens are available based on amountToWithdraw.
    ///
    /// Represents function `withdrawAnyFromVesting(VestingSimple vestingContract, uint256 amountToWithdraw)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct WithdrawAnyFromVesting {
        #[prost(string, tag = "1")]
        pub vesting_contract: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub amount: ::prost::alloc::string::String,
    }
    ///
    /// Withdraw a certain amount of tokens from vesting, from any deposit. This will not affect the cellar's TVL because any deposit must
    /// already have vested, and will be reported in balanceOf. Will revert if not enough tokens are available based on amountToWithdraw.
    ///
    /// Represents function `withdrawAllFromVesting(VestingSimple vestingContract)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct WithdrawAllFromVesting {
        #[prost(string, tag = "1")]
        pub vesting_contract: ::prost::alloc::string::String,
    }
    ///**** BASE ADAPTOR FUNCTIONS ****
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Oneof)]
    pub enum Function {
        /// Represents function `revokeApproval(ERC20 asset, address spender)`
        #[prost(message, tag = "1")]
        RevokeApproval(super::RevokeApproval),
        //**** ADAPTOR-SPECIFIC FUNCTIONS ****
        /// Represents function `depositToVesting(VestingSimple vestingContract, uint256 amountToDeposit)`
        #[prost(message, tag = "2")]
        DepositToVesting(DepositToVesting),
        /// Represents function `withdrawFromVesting(VestingSimple vestingContract, uint256 depositId, uint256 amountToWithdraw)`
        #[prost(message, tag = "3")]
        WithdrawFromVesting(WithdrawFromVesting),
        /// Represents function `withdrawAnyFromVesting(VestingSimple vestingContract, uint256 amountToWithdraw)`
        #[prost(message, tag = "4")]
        WithdrawAnyFromVesting(WithdrawAnyFromVesting),
        /// Represents function `withdrawAllFromVesting(VestingSimple vestingContract)`
        #[prost(message, tag = "5")]
        WithdrawAllFromVesting(WithdrawAllFromVesting),
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct VestingSimpleAdaptorV2Calls {
    #[prost(message, repeated, tag = "1")]
    pub calls: ::prost::alloc::vec::Vec<VestingSimpleAdaptorV2>,
}
/// Represents call data for the Aave Debt Token adaptor V1, used for borrowing and repaying debt on Aave.
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct AaveDebtTokenAdaptorV1 {
    ///**** BASE ADAPTOR FUNCTIONS ****
    #[prost(
        oneof = "aave_debt_token_adaptor_v1::Function",
        tags = "1, 2, 3, 4, 5, 6"
    )]
    pub function: ::core::option::Option<aave_debt_token_adaptor_v1::Function>,
}
/// Nested message and enum types in `AaveDebtTokenAdaptorV1`.
pub mod aave_debt_token_adaptor_v1 {
    ///
    /// Allows strategists to borrow assets from Aave.
    ///
    /// Represents function `depositToAave(ERC20 tokenToDeposit, uint256 amountToDeposit)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct BorrowFromAave {
        /// The address of the ERC20 token to borrow
        #[prost(string, tag = "1")]
        pub token: ::prost::alloc::string::String,
        /// The amount to borrow
        #[prost(string, tag = "2")]
        pub amount: ::prost::alloc::string::String,
    }
    ///
    /// Allows strategists to repay loan debt on Aave.
    ///
    /// Represents function `repayAaveDebt(ERC20 tokenToRepay, uint256 amountToRepay)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct RepayAaveDebt {
        /// The address of the ERC20 token to repay
        #[prost(string, tag = "1")]
        pub token: ::prost::alloc::string::String,
        /// The amount to repay
        #[prost(string, tag = "2")]
        pub amount: ::prost::alloc::string::String,
    }
    ///
    /// Allows strategists to swap assets and repay loans in one call.
    ///
    /// Represents function `swapAndRepay(ERC20 tokenIn, ERC20 tokenToRepay, uint256 amountIn, SwapRouter.Exchange exchange, bytes memory params)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct SwapAndRepay {
        /// The address of the token to swap from
        #[prost(string, tag = "1")]
        pub token_in: ::prost::alloc::string::String,
        /// The address of the token to swap to and repay with
        #[prost(string, tag = "2")]
        pub token_to_repay: ::prost::alloc::string::String,
        /// The amount to swap
        #[prost(string, tag = "3")]
        pub amount_in: ::prost::alloc::string::String,
        /// The exchange to make the swap on
        #[prost(enumeration = "super::Exchange", tag = "4")]
        pub exchange: i32,
        /// The parameters for the swap
        #[prost(message, optional, tag = "5")]
        pub params: ::core::option::Option<super::SwapParams>,
    }
    ///**** BASE ADAPTOR FUNCTIONS ****
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Oneof)]
    pub enum Function {
        /// Represents function `swap(ERC20 assetIn, ERC20 assetOut, uint256 amountIn, SwapRouter.Exchange exchange, bytes memory params)`
        #[prost(message, tag = "1")]
        Swap(super::Swap),
        /// Represents function `oracleSwap(ERC20 assetIn, ERC20 assetOut, uint256 amountIn, SwapRouter.Exchange exchange, bytes memory params, uint64 slippage)`
        #[prost(message, tag = "2")]
        OracleSwap(super::OracleSwap),
        /// Represents function `revokeApproval(ERC20 asset, address spender)`
        #[prost(message, tag = "3")]
        RevokeApproval(super::RevokeApproval),
        //**** ADAPTOR-SPECIFIC FUNCTIONS ****
        /// Represents function `borrowFromAave(ERC20 debtTokenToBorrow, uint256 amountToBorrow)`
        #[prost(message, tag = "4")]
        BorrowFromAave(BorrowFromAave),
        /// Represents function `repayAaveDebt(ERC20 tokenToRepay, uint256 amountToRepay)`
        #[prost(message, tag = "5")]
        RepayAaveDebt(RepayAaveDebt),
        /// Represents function `swapAndRepay(ERC20 tokenIn, ERC20 tokenToRepay, uint256 amountIn, SwapRouter.Exchange exchange, bytes memory params)`
        #[prost(message, tag = "6")]
        SwapAndRepay(SwapAndRepay),
    }
}
/// Represents call data for the Aave Debt Token adaptor V2, used for borrowing and repaying debt on Aave.
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct AaveDebtTokenAdaptorV2 {
    ///**** BASE ADAPTOR FUNCTIONS ****
    #[prost(oneof = "aave_debt_token_adaptor_v2::Function", tags = "1, 2, 3")]
    pub function: ::core::option::Option<aave_debt_token_adaptor_v2::Function>,
}
/// Nested message and enum types in `AaveDebtTokenAdaptorV2`.
pub mod aave_debt_token_adaptor_v2 {
    ///
    /// Allows strategists to borrow assets from Aave.
    ///
    /// Represents function `depositToAave(ERC20 tokenToDeposit, uint256 amountToDeposit)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct BorrowFromAave {
        /// The address of the ERC20 token to borrow
        #[prost(string, tag = "1")]
        pub token: ::prost::alloc::string::String,
        /// The amount to borrow
        #[prost(string, tag = "2")]
        pub amount: ::prost::alloc::string::String,
    }
    ///
    /// Allows strategists to repay loan debt on Aave.
    ///
    /// Represents function `repayAaveDebt(ERC20 tokenToRepay, uint256 amountToRepay)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct RepayAaveDebt {
        /// The address of the ERC20 token to repay
        #[prost(string, tag = "1")]
        pub token: ::prost::alloc::string::String,
        /// The amount to repay
        #[prost(string, tag = "2")]
        pub amount: ::prost::alloc::string::String,
    }
    ///**** BASE ADAPTOR FUNCTIONS ****
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Oneof)]
    pub enum Function {
        /// Represents function `revokeApproval(ERC20 asset, address spender)`
        #[prost(message, tag = "1")]
        RevokeApproval(super::RevokeApproval),
        //**** ADAPTOR-SPECIFIC FUNCTIONS ****
        /// Represents function `borrowFromAave(ERC20 debtTokenToBorrow, uint256 amountToBorrow)`
        #[prost(message, tag = "2")]
        BorrowFromAave(BorrowFromAave),
        /// Represents function `repayAaveDebt(ERC20 tokenToRepay, uint256 amountToRepay)`
        #[prost(message, tag = "3")]
        RepayAaveDebt(RepayAaveDebt),
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct AaveDebtTokenAdaptorV1Calls {
    #[prost(message, repeated, tag = "1")]
    pub calls: ::prost::alloc::vec::Vec<AaveDebtTokenAdaptorV1>,
}
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct AaveDebtTokenAdaptorV2Calls {
    #[prost(message, repeated, tag = "1")]
    pub calls: ::prost::alloc::vec::Vec<AaveDebtTokenAdaptorV2>,
}
/// Represents call data for the Aave Debt Token adaptor, used for borrowing and repaying debt on Aave.
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct AaveV3DebtTokenAdaptorV1 {
    ///**** BASE ADAPTOR FUNCTIONS ****
    #[prost(oneof = "aave_v3_debt_token_adaptor_v1::Function", tags = "1, 2, 3, 4")]
    pub function: ::core::option::Option<aave_v3_debt_token_adaptor_v1::Function>,
}
/// Nested message and enum types in `AaveV3DebtTokenAdaptorV1`.
pub mod aave_v3_debt_token_adaptor_v1 {
    ///
    /// Allows strategists to borrow assets from Aave.
    ///
    /// Represents function `depositToAave(ERC20 tokenToDeposit, uint256 amountToDeposit)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct BorrowFromAave {
        /// The address of the ERC20 token to borrow
        #[prost(string, tag = "1")]
        pub token: ::prost::alloc::string::String,
        /// The amount to borrow
        #[prost(string, tag = "2")]
        pub amount: ::prost::alloc::string::String,
    }
    ///
    /// Allows strategists to repay loan debt on Aave.
    ///
    /// Represents function `repayAaveDebt(ERC20 tokenToRepay, uint256 amountToRepay)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct RepayAaveDebt {
        /// The address of the ERC20 token to repay
        #[prost(string, tag = "1")]
        pub token: ::prost::alloc::string::String,
        /// The amount to repay
        #[prost(string, tag = "2")]
        pub amount: ::prost::alloc::string::String,
    }
    ///
    /// Allows strategist to use aTokens to repay debt tokens with the same underlying.
    ///
    /// Represents function `repayWithATokens(ERC20 underlying, uint256 amount)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct RepayWithATokens {
        /// The address of the underlying ERC20 token to repay
        #[prost(string, tag = "1")]
        pub underlying_token: ::prost::alloc::string::String,
        /// The amount to repay
        #[prost(string, tag = "2")]
        pub amount: ::prost::alloc::string::String,
    }
    ///**** BASE ADAPTOR FUNCTIONS ****
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Oneof)]
    pub enum Function {
        /// Represents function `revokeApproval(ERC20 asset, address spender)`
        #[prost(message, tag = "1")]
        RevokeApproval(super::RevokeApproval),
        //**** ADAPTOR-SPECIFIC FUNCTIONS ****
        /// Represents function `borrowFromAave(ERC20 debtTokenToBorrow, uint256 amountToBorrow)`
        #[prost(message, tag = "2")]
        BorrowFromAave(BorrowFromAave),
        /// Represents function `repayAaveDebt(ERC20 tokenToRepay, uint256 amountToRepay)`
        #[prost(message, tag = "3")]
        RepayAaveDebt(RepayAaveDebt),
        /// Represents function `repayWithATokens(ERC20 underlying, uint256 amount)`
        #[prost(message, tag = "4")]
        RepayWithATokens(RepayWithATokens),
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct AaveV3DebtTokenAdaptorV1Calls {
    #[prost(message, repeated, tag = "1")]
    pub calls: ::prost::alloc::vec::Vec<AaveV3DebtTokenAdaptorV1>,
}
/// Represents call data for the Aave AToken adaptor V1, used to manage lending positions on Aave
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct AaveATokenAdaptorV1 {
    ///**** BASE ADAPTOR FUNCTIONS ****
    #[prost(oneof = "aave_a_token_adaptor_v1::Function", tags = "1, 2, 3, 4, 5")]
    pub function: ::core::option::Option<aave_a_token_adaptor_v1::Function>,
}
/// Nested message and enum types in `AaveATokenAdaptorV1`.
pub mod aave_a_token_adaptor_v1 {
    ///
    /// Allows strategists to lend assets on Aave.
    ///
    /// Represents function `depositToAave(ERC20 tokenToDeposit, uint256 amountToDeposit)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct DepositToAave {
        /// The address of the ERC20 token to deposit
        #[prost(string, tag = "1")]
        pub token: ::prost::alloc::string::String,
        /// The amount to deposit
        #[prost(string, tag = "2")]
        pub amount: ::prost::alloc::string::String,
    }
    ///
    /// Allows strategists to withdraw assets from Aave.
    ///
    /// Represents function `withdrawFromAave(ERC20 tokenToWithdraw, uint256 amountToWithdraw)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct WithdrawFromAave {
        /// The address of the ERC20 token to withdraw
        #[prost(string, tag = "1")]
        pub token: ::prost::alloc::string::String,
        /// The amount to withdraw
        #[prost(string, tag = "2")]
        pub amount: ::prost::alloc::string::String,
    }
    ///**** BASE ADAPTOR FUNCTIONS ****
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Oneof)]
    pub enum Function {
        /// Represents function `swap(ERC20 assetIn, ERC20 assetOut, uint256 amountIn, SwapRouter.Exchange exchange, bytes memory params)`
        #[prost(message, tag = "1")]
        Swap(super::Swap),
        /// Represents function `oracleSwap(ERC20 assetIn, ERC20 assetOut, uint256 amountIn, SwapRouter.Exchange exchange, bytes memory params, uint64 slippage)`
        #[prost(message, tag = "2")]
        OracleSwap(super::OracleSwap),
        /// Represents function `revokeApproval(ERC20 asset, address spender)`
        #[prost(message, tag = "3")]
        RevokeApproval(super::RevokeApproval),
        //**** ADAPTOR-SPECIFIC FUNCTIONS ****
        /// Represents function `depositToAave(ERC20 tokenToDeposit, uint256 amountToDeposit)`
        #[prost(message, tag = "4")]
        DepositToAave(DepositToAave),
        /// Represents function `withdrawFromAave(ERC20 tokenToWithdraw, uint256 amountToWithdraw)`
        #[prost(message, tag = "5")]
        WithdrawFromAave(WithdrawFromAave),
    }
}
/// Represents call data for the Aave AToken adaptor V2, used to manage lending positions on Aave
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct AaveATokenAdaptorV2 {
    ///**** BASE ADAPTOR FUNCTIONS ****
    #[prost(oneof = "aave_a_token_adaptor_v2::Function", tags = "1, 2, 3")]
    pub function: ::core::option::Option<aave_a_token_adaptor_v2::Function>,
}
/// Nested message and enum types in `AaveATokenAdaptorV2`.
pub mod aave_a_token_adaptor_v2 {
    ///
    /// Allows strategists to lend assets on Aave.
    ///
    /// Represents function `depositToAave(ERC20 tokenToDeposit, uint256 amountToDeposit)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct DepositToAave {
        /// The address of the ERC20 token to deposit
        #[prost(string, tag = "1")]
        pub token: ::prost::alloc::string::String,
        /// The amount to deposit
        #[prost(string, tag = "2")]
        pub amount: ::prost::alloc::string::String,
    }
    ///
    /// Allows strategists to withdraw assets from Aave.
    ///
    /// Represents function `withdrawFromAave(ERC20 tokenToWithdraw, uint256 amountToWithdraw)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct WithdrawFromAave {
        /// The address of the ERC20 token to withdraw
        #[prost(string, tag = "1")]
        pub token: ::prost::alloc::string::String,
        /// The amount to withdraw
        #[prost(string, tag = "2")]
        pub amount: ::prost::alloc::string::String,
    }
    ///**** BASE ADAPTOR FUNCTIONS ****
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Oneof)]
    pub enum Function {
        /// Represents function `revokeApproval(ERC20 asset, address spender)`
        #[prost(message, tag = "1")]
        RevokeApproval(super::RevokeApproval),
        //**** ADAPTOR-SPECIFIC FUNCTIONS ****
        /// Represents function `depositToAave(ERC20 tokenToDeposit, uint256 amountToDeposit)`
        #[prost(message, tag = "2")]
        DepositToAave(DepositToAave),
        /// Represents function `withdrawFromAave(ERC20 tokenToWithdraw, uint256 amountToWithdraw)`
        #[prost(message, tag = "3")]
        WithdrawFromAave(WithdrawFromAave),
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct AaveATokenAdaptorV1Calls {
    #[prost(message, repeated, tag = "1")]
    pub calls: ::prost::alloc::vec::Vec<AaveATokenAdaptorV1>,
}
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct AaveATokenAdaptorV2Calls {
    #[prost(message, repeated, tag = "1")]
    pub calls: ::prost::alloc::vec::Vec<AaveATokenAdaptorV2>,
}
/// Represents call data for the Aave AToken adaptor, used to manage lending positions on Aave
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct AaveV3aTokenAdaptorV1 {
    ///**** BASE ADAPTOR FUNCTIONS ****
    #[prost(oneof = "aave_v3a_token_adaptor_v1::Function", tags = "1, 2, 3, 4, 5")]
    pub function: ::core::option::Option<aave_v3a_token_adaptor_v1::Function>,
}
/// Nested message and enum types in `AaveV3ATokenAdaptorV1`.
pub mod aave_v3a_token_adaptor_v1 {
    ///
    /// Allows strategists to lend assets on Aave.
    ///
    /// Represents function `depositToAave(ERC20 tokenToDeposit, uint256 amountToDeposit)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct DepositToAave {
        /// The address of the ERC20 token to deposit
        #[prost(string, tag = "1")]
        pub token: ::prost::alloc::string::String,
        /// The amount to deposit
        #[prost(string, tag = "2")]
        pub amount: ::prost::alloc::string::String,
    }
    ///
    /// Allows strategists to withdraw assets from Aave.
    ///
    /// Represents function `withdrawFromAave(ERC20 tokenToWithdraw, uint256 amountToWithdraw)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct WithdrawFromAave {
        /// The address of the ERC20 token to withdraw
        #[prost(string, tag = "1")]
        pub token: ::prost::alloc::string::String,
        /// The amount to withdraw
        #[prost(string, tag = "2")]
        pub amount: ::prost::alloc::string::String,
    }
    ///
    /// Allows strategists to adjust an asset's isolation mode.
    ///
    /// Represents function `adjustIsolationModeAssetAsCollateral(ERC20 asset, bool useAsCollateral)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct AdjustIsolationModeAssetAsCollateral {
        /// The address of the ERC20 token
        #[prost(string, tag = "1")]
        pub asset: ::prost::alloc::string::String,
        /// Whether to use the asset as collateral
        #[prost(bool, tag = "2")]
        pub use_as_collateral: bool,
    }
    ///
    /// Allows strategists to enter different EModes.
    ///
    /// Represents function `changeEMode(uint8 categoryId)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct ChangeEMode {
        /// The category ID
        #[prost(uint32, tag = "1")]
        pub category_id: u32,
    }
    ///**** BASE ADAPTOR FUNCTIONS ****
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Oneof)]
    pub enum Function {
        /// Represents function `revokeApproval(ERC20 asset, address spender)`
        #[prost(message, tag = "1")]
        RevokeApproval(super::RevokeApproval),
        //**** ADAPTOR-SPECIFIC FUNCTIONS ****
        /// Represents function `depositToAave(ERC20 tokenToDeposit, uint256 amountToDeposit)`
        #[prost(message, tag = "2")]
        DepositToAave(DepositToAave),
        /// Represents function `withdrawFromAave(ERC20 tokenToWithdraw, uint256 amountToWithdraw)`
        #[prost(message, tag = "3")]
        WithdrawFromAave(WithdrawFromAave),
        /// Represents function `adjustIsolationModeAssetAsCollateral(ERC20 asset, bool useAsCollateral)`
        #[prost(message, tag = "4")]
        AdjustIsolationModeAssetAsCollateral(AdjustIsolationModeAssetAsCollateral),
        /// Represents function `changeEMode(uint8 categoryId)`
        #[prost(message, tag = "5")]
        ChangeEmode(ChangeEMode),
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct AaveV3aTokenAdaptorV1Calls {
    #[prost(message, repeated, tag = "1")]
    pub calls: ::prost::alloc::vec::Vec<AaveV3aTokenAdaptorV1>,
}
/// Represents call data for the Aura ERC4626 adaptor V1
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct AuraErc4626AdaptorV1 {
    ///**** BASE ADAPTOR FUNCTIONS ****
    #[prost(oneof = "aura_erc4626_adaptor_v1::Function", tags = "1, 2")]
    pub function: ::core::option::Option<aura_erc4626_adaptor_v1::Function>,
}
/// Nested message and enum types in `AuraERC4626AdaptorV1`.
pub mod aura_erc4626_adaptor_v1 {
    ///
    /// Allows strategist to get rewards for an Aura pool.
    ///
    /// Represents function `getRewards(IBaseRewardPool _auraPool, bool _claimExtras)`    
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct GetRewards {
        /// The address of the Aura pool to get rewards for
        #[prost(string, tag = "1")]
        pub aura_pool: ::prost::alloc::string::String,
        /// Whether to claim extra rewards associated with the pool
        #[prost(bool, tag = "2")]
        pub claim_extras: bool,
    }
    ///**** BASE ADAPTOR FUNCTIONS ****
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Oneof)]
    pub enum Function {
        /// Represents function `revokeApproval(ERC20 asset, address spender)`
        #[prost(message, tag = "1")]
        RevokeApproval(super::RevokeApproval),
        //**** ADAPTOR-SPECIFIC FUNCTIONS ****
        /// Represents function `getRewards(IBaseRewardPool _auraPool, bool _claimExtras)`
        #[prost(message, tag = "2")]
        GetRewards(GetRewards),
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct AuraErc4626AdaptorV1Calls {
    #[prost(message, repeated, tag = "1")]
    pub calls: ::prost::alloc::vec::Vec<AuraErc4626AdaptorV1>,
}
/// Represents call data for the Balancer Pool adaptor V1, for managing pool positions on Balancer.
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct BalancerPoolAdaptorV1 {
    ///**** BASE ADAPTOR FUNCTIONS ****
    #[prost(
        oneof = "balancer_pool_adaptor_v1::Function",
        tags = "1, 2, 3, 4, 5, 6"
    )]
    pub function: ::core::option::Option<balancer_pool_adaptor_v1::Function>,
}
/// Nested message and enum types in `BalancerPoolAdaptorV1`.
pub mod balancer_pool_adaptor_v1 {
    /// Data for a single swap executed by `swap`. `amount` is either `amountIn` or `amountOut` depending on the `kind` value.
    /// Represents the SingleSwap struct defined here:
    /// https://github.com/PeggyJV/cellar-contracts/blob/main/src/interfaces/external/Balancer/IVault.sol
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct SingleSwap {
        /// The pool ID (bytes32)
        #[prost(string, tag = "1")]
        pub pool_id: ::prost::alloc::string::String,
        /// The swap kind (enum)
        #[prost(enumeration = "SwapKind", tag = "2")]
        pub kind: i32,
        /// The asset in (address)
        #[prost(string, tag = "3")]
        pub asset_in: ::prost::alloc::string::String,
        /// The asset out (address)
        #[prost(string, tag = "4")]
        pub asset_out: ::prost::alloc::string::String,
        /// The amount (uint256)
        #[prost(string, tag = "5")]
        pub amount: ::prost::alloc::string::String,
        /// The user data (bytes)
        #[prost(bytes = "vec", tag = "6")]
        pub user_data: ::prost::alloc::vec::Vec<u8>,
    }
    /// Stores each swaps min amount, and deadline
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct SwapData {
        /// The minimum amounts for swaps
        #[prost(string, repeated, tag = "1")]
        pub min_amounts_for_swaps: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// The swap deadlines
        #[prost(string, repeated, tag = "2")]
        pub swap_deadlines: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    ///
    /// Allows strategists to join Balancer pools using EXACT_TOKENS_IN_FOR_BPT_OUT joins
    ///
    /// Represents function `joinPool(ERC20 targetBpt, IVault.SingleSwap[] memory swapsBeforeJoin, SwapData memory swapData, uint256 minimumBpt)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct JoinPool {
        /// The target pool
        #[prost(string, tag = "1")]
        pub target_bpt: ::prost::alloc::string::String,
        /// Swap to execute before joining pool
        #[prost(message, repeated, tag = "2")]
        pub swaps_before_join: ::prost::alloc::vec::Vec<SingleSwap>,
        /// Data for swaps
        #[prost(message, optional, tag = "3")]
        pub swap_data: ::core::option::Option<SwapData>,
        /// The minimum BPT to mint
        #[prost(string, tag = "4")]
        pub minimum_bpt: ::prost::alloc::string::String,
    }
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct ExitPoolRequest {
        #[prost(string, repeated, tag = "1")]
        pub assets: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        #[prost(string, repeated, tag = "2")]
        pub min_amounts_out: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        #[prost(bytes = "vec", tag = "3")]
        pub user_data: ::prost::alloc::vec::Vec<u8>,
        #[prost(bool, tag = "4")]
        pub to_internal_balance: bool,
    }
    ///
    /// Call `BalancerRelayer` on mainnet to carry out exit txs
    ///
    /// Represents function `exitPool(ERC20 targetBpt, IVault.SingleSwap[] memory swapsBeforeJoin, SwapData memory swapData, IVault.ExitPoolRequest request)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct ExitPool {
        /// The target pool
        #[prost(string, tag = "1")]
        pub target_bpt: ::prost::alloc::string::String,
        /// Swaps to execute after exiting pool
        #[prost(message, repeated, tag = "2")]
        pub swaps_after_exit: ::prost::alloc::vec::Vec<SingleSwap>,
        /// Data for swaps
        #[prost(message, optional, tag = "3")]
        pub swap_data: ::core::option::Option<SwapData>,
        #[prost(message, optional, tag = "4")]
        pub request: ::core::option::Option<ExitPoolRequest>,
    }
    ///
    /// Stake (deposit) BPTs into respective pool gauge
    ///
    /// Represents `function stakeBPT(ERC20 _bpt, address _liquidityGauge, uint256 _amountIn)``
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct StakeBpt {
        /// The BPT to stake
        #[prost(string, tag = "1")]
        pub bpt: ::prost::alloc::string::String,
        /// The liquidity gauge to stake into
        #[prost(string, tag = "2")]
        pub liquidity_gauge: ::prost::alloc::string::String,
        /// The amount to stake
        #[prost(string, tag = "3")]
        pub amount_in: ::prost::alloc::string::String,
    }
    ///
    /// Unstake (withdraw) BPT from respective pool gauge
    ///
    /// Represents `function unstakeBPT(ERC20 _bpt, address _liquidityGauge, uint256 _amountOut)``
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct UnstakeBpt {
        /// The BPT to unstake
        #[prost(string, tag = "1")]
        pub bpt: ::prost::alloc::string::String,
        /// The liquidity gauge to unstake from
        #[prost(string, tag = "2")]
        pub liquidity_gauge: ::prost::alloc::string::String,
        /// The amount to unstake
        #[prost(string, tag = "3")]
        pub amount_out: ::prost::alloc::string::String,
    }
    ///
    /// Claim rewards ($BAL) from LP position
    ///
    /// Represents `function claimRewards(address gauge)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct ClaimRewards {
        /// The gauge to claim rewards from
        #[prost(string, tag = "1")]
        pub gauge: ::prost::alloc::string::String,
    }
    /// Represents the SwapKind enum defined here:
    /// https://github.com/PeggyJV/cellar-contracts/blob/main/src/interfaces/external/Balancer/IVault.sol
    #[derive(
        serde::Deserialize,
        serde::Serialize,
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration,
    )]
    #[repr(i32)]
    pub enum SwapKind {
        Unspecified = 0,
        GivenIn = 1,
        GivenOut = 2,
    }
    ///**** BASE ADAPTOR FUNCTIONS ****
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Oneof)]
    pub enum Function {
        /// Represents function `revokeApproval(ERC20 asset, address spender)`
        #[prost(message, tag = "1")]
        RevokeApproval(super::RevokeApproval),
        //**** ADAPTOR-SPECIFIC FUNCTIONS ****
        /// Represents function `relayerJoinPool(ERC20[] tokensIn, uint256[] amountsIn, ERC20 btpOut, bytes[] memory callData)`
        #[prost(message, tag = "2")]
        JoinPool(JoinPool),
        /// Represents function `relayerExitPool(ERC20 bptIn, uint256 amountIn, ERC20[] memory tokensOut, bytes[] memory callData)`
        #[prost(message, tag = "3")]
        ExitPool(ExitPool),
        /// Represents function `stakeBPT(ERC20 _bpt, address _liquidityGauge, uint256 _amountIn)`
        #[prost(message, tag = "4")]
        StakeBpt(StakeBpt),
        /// Represents function `unstakeBPT(ERC20 _bpt, address _liquidityGauge, uint256 _amountOut)`
        #[prost(message, tag = "5")]
        UnstakeBpt(UnstakeBpt),
        /// Represents function `claimRewards(address gauge)`
        #[prost(message, tag = "6")]
        ClaimRewards(ClaimRewards),
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct BalancerPoolAdaptorV1Calls {
    #[prost(message, repeated, tag = "1")]
    pub calls: ::prost::alloc::vec::Vec<BalancerPoolAdaptorV1>,
}
/// Represents call data for the Compound C Token adaptor V2, managing lending positions on Compound.
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct CompoundCTokenAdaptorV2 {
    ///**** BASE ADAPTOR FUNCTIONS ****
    #[prost(oneof = "compound_c_token_adaptor_v2::Function", tags = "1, 2, 3, 4")]
    pub function: ::core::option::Option<compound_c_token_adaptor_v2::Function>,
}
/// Nested message and enum types in `CompoundCTokenAdaptorV2`.
pub mod compound_c_token_adaptor_v2 {
    ///
    /// Allows strategists to lend assets on Compound.
    ///
    /// Represents function `depositToCompound(CErc20 market, uint256 amountToDeposit)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct DepositToCompound {
        #[prost(string, tag = "1")]
        pub market: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub amount_to_deposit: ::prost::alloc::string::String,
    }
    ///
    /// Allows strategists to withdraw assets from Compound.
    ///
    /// Represents function `withdrawFromCompound(CErc20 market, uint256 amountToWithdraw)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct WithdrawFromCompound {
        #[prost(string, tag = "1")]
        pub market: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub amount_to_withdraw: ::prost::alloc::string::String,
    }
    ///
    /// Allows strategists to claim COMP rewards.
    ///
    /// Represents function `claimComp()`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct ClaimComp {}
    ///**** BASE ADAPTOR FUNCTIONS ****
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Oneof)]
    pub enum Function {
        /// Represents function `revokeApproval(ERC20 asset, address spender)`
        #[prost(message, tag = "1")]
        RevokeApproval(super::RevokeApproval),
        //**** ADAPTOR-SPECIFIC FUNCTIONS ****
        /// Represents function `depositToCompound(CErc20 market, uint256 amountToDeposit)`
        #[prost(message, tag = "2")]
        DepositToCompound(DepositToCompound),
        /// Represents function `withdrawFromCompound(CErc20 market, uint256 amountToWithdraw)`
        #[prost(message, tag = "3")]
        WithdrawFromCompound(WithdrawFromCompound),
        /// Represents function `claimComp()`
        #[prost(message, tag = "4")]
        ClaimComp(ClaimComp),
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct CompoundCTokenAdaptorV2Calls {
    #[prost(message, repeated, tag = "1")]
    pub calls: ::prost::alloc::vec::Vec<CompoundCTokenAdaptorV2>,
}
// TODO: Comments

/// Represents call data for the FeesAndReserves and FeesAndReservesAdaptor contracts.
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct FeesAndReservesAdaptorV1 {
    ///**** BASE ADAPTOR FUNCTIONS ****
    #[prost(
        oneof = "fees_and_reserves_adaptor_v1::Function",
        tags = "1, 2, 3, 4, 5, 6, 7, 8, 9"
    )]
    pub function: ::core::option::Option<fees_and_reserves_adaptor_v1::Function>,
}
/// Nested message and enum types in `FeesAndReservesAdaptorV1`.
pub mod fees_and_reserves_adaptor_v1 {
    ///
    /// Allows the owner to update a Cellar's performance fee.
    ///
    /// Represents function `updatePerformanceFee(uint32 performanceFee)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct UpdatePerformanceFees {
        #[prost(uint32, tag = "1")]
        pub performance_fee: u32,
    }
    ///
    /// Allows the owner to update a Cellar's management fee.
    ///
    /// Represents function `updateManagementFee(uint32 managementFee)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct UpdateManagementFees {
        #[prost(uint32, tag = "1")]
        pub management_fee: u32,
    }
    ///
    /// Allows the owner to update a Cellar's upkeep frequency.
    ///
    /// Represents function `changeUpkeepFrequency(uint64 newFrequency)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct ChangeUpkeepFrequency {
        #[prost(uint64, tag = "1")]
        pub new_frequency: u64,
    }
    ///
    /// Allows the owner to update a Cellar's upkeep max gas.
    ///
    /// Represents function `changeUpkeepMaxGas(uint64 newMaxGas)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct ChangeUpkeepMaxGas {
        #[prost(uint64, tag = "1")]
        pub new_max_gas: u64,
    }
    ///
    /// Allows the owner to set the Cellar's fee metadata
    ///
    /// Represents function `setupMetaData(uint32 managementFee, uint32 performanceFee)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct SetupMetaData {
        #[prost(uint32, tag = "1")]
        pub management_fee: u32,
        #[prost(uint32, tag = "2")]
        pub performance_fee: u32,
    }
    ///
    /// Allows the owner to add assets to the Cellar's reserves
    ///
    /// Represents function `addAssetsToReserves(uint256 amount)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct AddAssetsToReserves {
        #[prost(string, tag = "1")]
        pub amount: ::prost::alloc::string::String,
    }
    ///
    /// Allows the owner to withdraw assets from the Cellar's reserves
    ///
    /// Represents function `withdrawAssetsFromReserves(uint256 amount)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct WithdrawAssetsFromReserves {
        #[prost(string, tag = "1")]
        pub amount: ::prost::alloc::string::String,
    }
    ///
    /// Allows the owner to prepare fees to be split between the platform, strategist, and protocol
    ///
    /// Represents function `prepareFees(uint256 amount)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct PrepareFees {
        #[prost(string, tag = "1")]
        pub amount: ::prost::alloc::string::String,
    }
    ///**** BASE ADAPTOR FUNCTIONS ****
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Oneof)]
    pub enum Function {
        /// Represents function `revokeApproval(ERC20 asset, address spender)`
        #[prost(message, tag = "1")]
        RevokeApproval(super::RevokeApproval),
        //**** ADAPTOR-SPECIFIC FUNCTIONS ****
        /// Represents function `updatePerformanceFee(uint32 performanceFee)`
        #[prost(message, tag = "2")]
        UpdatePerformanceFees(UpdatePerformanceFees),
        /// Represents function `updateManagementFee(uint32 managementFee)`
        #[prost(message, tag = "3")]
        UpdateManagementFees(UpdateManagementFees),
        /// Represents function `changeUpkeepFrequency(uint64 newFrequency)`
        #[prost(message, tag = "4")]
        ChangeUpkeepFrequency(ChangeUpkeepFrequency),
        /// Represents function `changeUpkeepMaxGas(uint64 newMaxGas)`
        #[prost(message, tag = "5")]
        ChangeUpkeepMaxGas(ChangeUpkeepMaxGas),
        /// Represents function `setupMetaData(uint32 managementFee, uint32 performanceFee)`
        #[prost(message, tag = "6")]
        SetupMetaData(SetupMetaData),
        /// Represents function `addAssetsToReserves(uint256 amount)`
        #[prost(message, tag = "7")]
        AddAssetsToReserves(AddAssetsToReserves),
        /// Represents function `withdrawAssetsFromReserves(uint256 amount)`
        #[prost(message, tag = "8")]
        WithdrawAssetsFromReserves(WithdrawAssetsFromReserves),
        /// Represents function `prepareFees(uint256 amount)`
        #[prost(message, tag = "9")]
        PrepareFees(PrepareFees),
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct FeesAndReservesAdaptorV1Calls {
    #[prost(message, repeated, tag = "1")]
    pub calls: ::prost::alloc::vec::Vec<FeesAndReservesAdaptorV1>,
}
/// Represents call data for the OneInch adaptor.
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct OneInchAdaptorV1 {
    ///**** BASE ADAPTOR FUNCTIONS ****
    #[prost(oneof = "one_inch_adaptor_v1::Function", tags = "1, 2")]
    pub function: ::core::option::Option<one_inch_adaptor_v1::Function>,
}
/// Nested message and enum types in `OneInchAdaptorV1`.
pub mod one_inch_adaptor_v1 {
    ///
    /// Allows strategists to make ERC20 swaps using 1Inch.
    ///
    /// Represents function `swapWithOneInch(ERC20 tokenIn, ERC20 tokenOut, uint256 amount, bytes swapCallData)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct SwapWithOneInch {
        #[prost(string, tag = "1")]
        pub token_in: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub token_out: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub amount: ::prost::alloc::string::String,
        #[prost(bytes = "vec", tag = "4")]
        pub swap_call_data: ::prost::alloc::vec::Vec<u8>,
    }
    ///**** BASE ADAPTOR FUNCTIONS ****
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Oneof)]
    pub enum Function {
        /// Represents function `revokeApproval(ERC20 asset, address spender)`
        #[prost(message, tag = "1")]
        RevokeApproval(super::RevokeApproval),
        /// Represents function `swapWithOneInch(ERC20 tokenIn, ERC20 tokenOut, uint256 amount, bytes swapCallData)`
        #[prost(message, tag = "2")]
        SwapWithOneInch(SwapWithOneInch),
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct OneInchAdaptorV1Calls {
    #[prost(message, repeated, tag = "1")]
    pub calls: ::prost::alloc::vec::Vec<OneInchAdaptorV1>,
}
/// Represents call data for the Aave AToken adaptor, used to manage lending positions on Aave
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct AaveV2EnableAssetAsCollateralAdaptorV1 {
    ///**** BASE ADAPTOR FUNCTIONS ****
    #[prost(
        oneof = "aave_v2_enable_asset_as_collateral_adaptor_v1::Function",
        tags = "1, 2"
    )]
    pub function: ::core::option::Option<aave_v2_enable_asset_as_collateral_adaptor_v1::Function>,
}
/// Nested message and enum types in `AaveV2EnableAssetAsCollateralAdaptorV1`.
pub mod aave_v2_enable_asset_as_collateral_adaptor_v1 {
    ///
    /// Allows a strategist to choose to use an asset as collateral or not.
    ///
    /// Represents function `setUserUseReserveAsCollateral(address asset, bool useAsCollateral)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct SetUserUseReserveAsCollateral {
        /// The address of the asset to set as collateral
        #[prost(string, tag = "1")]
        pub asset: ::prost::alloc::string::String,
        /// Whether to use the asset as collateral
        #[prost(bool, tag = "2")]
        pub use_as_collateral: bool,
    }
    ///**** BASE ADAPTOR FUNCTIONS ****
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Oneof)]
    pub enum Function {
        /// Represents function `revokeApproval(ERC20 asset, address spender)`
        #[prost(message, tag = "1")]
        RevokeApproval(super::RevokeApproval),
        //**** ADAPTOR-SPECIFIC FUNCTIONS ****
        /// Represents function `setUserUseReserveAsCollateral(address asset, bool useAsCollateral)`
        #[prost(message, tag = "2")]
        SetUserUseReserveAsCollateral(SetUserUseReserveAsCollateral),
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct AaveV2EnableAssetAsCollateralAdaptorV1Calls {
    #[prost(message, repeated, tag = "1")]
    pub calls: ::prost::alloc::vec::Vec<AaveV2EnableAssetAsCollateralAdaptorV1>,
}
/// Represents call data for the Aave Debt Token adaptor, used for borrowing and repaying debt on Aave.
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct AaveV3DebtTokenAdaptorV1FlashLoan {
    /// Represents function `flashLoan(address[] loanToken, uint256[] loanAmount, bytes params)`
    #[prost(message, optional, tag = "1")]
    pub flash_loan: ::core::option::Option<aave_v3_debt_token_adaptor_v1_flash_loan::FlashLoan>,
}
/// Nested message and enum types in `AaveV3DebtTokenAdaptorV1FlashLoan`.
pub mod aave_v3_debt_token_adaptor_v1_flash_loan {
    ///
    /// Allows strategists to have Cellars take out flash loans
    ///
    /// Represents function `flashLoan(address[] loanToken, uint256[] loanAmount, bytes params)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct FlashLoan {
        /// The addresses of the ERC20 tokens to borrow
        #[prost(string, repeated, tag = "1")]
        pub loan_tokens: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// The amounts to borrow
        #[prost(string, repeated, tag = "2")]
        pub loan_amounts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// The params to pass to the flash loan callback.
        #[prost(message, repeated, tag = "3")]
        pub params: ::prost::alloc::vec::Vec<AdaptorCallForAaveV3FlashLoan>,
    }
    // NOTE: FlashLoan takes an array of AdaptorCall. cellar_v2.proto defines it, but also imports this file, therefore we can't import cellar_v2.proto in order to use the AdaptorCall message here. To avoid the recursive import, we duplicate the message definition.

    /// Represents a call to adaptor an. The cellar must be authorized to call the target adaptor.
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct AdaptorCallForAaveV3FlashLoan {
        /// Address of the adaptor
        #[prost(string, tag = "1")]
        pub adaptor: ::prost::alloc::string::String,
        /// The function call data for the adaptor
        #[prost(
            oneof = "adaptor_call_for_aave_v3_flash_loan::CallData",
            tags = "2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32"
        )]
        pub call_data: ::core::option::Option<adaptor_call_for_aave_v3_flash_loan::CallData>,
    }
    /// Nested message and enum types in `AdaptorCallForAaveV3FlashLoan`.
    pub mod adaptor_call_for_aave_v3_flash_loan {
        /// The function call data for the adaptor
        #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Oneof)]
        pub enum CallData {
            /// Represents function calls to the UniswapV3Adaptor V1
            #[prost(message, tag = "2")]
            UniswapV3V1Calls(super::super::UniswapV3AdaptorV1Calls),
            /// Represents function calls to the AaveATokenAdaptor V1
            #[prost(message, tag = "3")]
            AaveATokenV1Calls(super::super::AaveATokenAdaptorV1Calls),
            /// Represents function calls to the AavaDebtTokenAdaptor V1
            #[prost(message, tag = "4")]
            AaveDebtTokenV1Calls(super::super::AaveDebtTokenAdaptorV1Calls),
            /// Represents function calls to the CompoundCTokenAdaptor V2
            #[prost(message, tag = "5")]
            CompoundCTokenV2Calls(super::super::CompoundCTokenAdaptorV2Calls),
            /// Represents function calls to the AaveATokenV2Adaptor
            #[prost(message, tag = "6")]
            AaveATokenV2Calls(super::super::AaveATokenAdaptorV2Calls),
            /// Represents function calls to the AavaDebtTokenV2Adaptor
            #[prost(message, tag = "7")]
            AaveDebtTokenV2Calls(super::super::AaveDebtTokenAdaptorV2Calls),
            /// Represents function calls to the AaveATokenV1Adaptor
            #[prost(message, tag = "8")]
            AaveV3ATokenV1Calls(super::super::AaveV3aTokenAdaptorV1Calls),
            /// Represents function calls to the OneInchAdaptorV1
            #[prost(message, tag = "9")]
            OneInchV1Calls(super::super::OneInchAdaptorV1Calls),
            /// Represents function calls to the FeesAndReservesAdaptorV1
            #[prost(message, tag = "10")]
            FeesAndReservesV1Calls(super::super::FeesAndReservesAdaptorV1Calls),
            /// Represents functionc alls to the ZeroXAdaptorV1
            #[prost(message, tag = "11")]
            ZeroXV1Calls(super::super::ZeroXAdaptorV1Calls),
            /// Represents function calls to the SwapWithUniswapAdaptorV1
            #[prost(message, tag = "12")]
            SwapWithUniswapV1Calls(super::super::SwapWithUniswapAdaptorV1Calls),
            /// Represents function calls to VestingSimpleAdaptor
            #[prost(message, tag = "13")]
            VestingSimpleV2Calls(super::super::VestingSimpleAdaptorV2Calls),
            /// Represents function calls to the CellarAdaptor
            #[prost(message, tag = "14")]
            CellarV1Calls(super::super::CellarAdaptorV1Calls),
            /// Represents function calls to the UniswapV3Adaptor V2
            #[prost(message, tag = "15")]
            UniswapV3V2Calls(super::super::UniswapV3AdaptorV2Calls),
            /// Represents function calls to the AaveV2EnableAssetAsCollatorAdaptor V1
            #[prost(message, tag = "16")]
            AaveV2EnableAssetAsCollateralV1Calls(
                super::super::AaveV2EnableAssetAsCollateralAdaptorV1Calls,
            ),
            /// Represents function calls to the FTokenAdaptor V1
            #[prost(message, tag = "17")]
            FTokenV1Calls(super::super::FTokenAdaptorV1Calls),
            /// Represents function calls to the MorphoAaveV2AToken V1
            #[prost(message, tag = "18")]
            MorphoAaveV2ATokenV1Calls(super::super::MorphoAaveV2aTokenAdaptorV1Calls),
            /// Represents function calls to the MorphoAaveV2DebtToken V1
            #[prost(message, tag = "19")]
            MorphoAaveV2DebtTokenV1Calls(super::super::MorphoAaveV2DebtTokenAdaptorV1Calls),
            /// Represents function calls to the MorphoAaveV3ATokenCollateral V1
            #[prost(message, tag = "20")]
            MorphoAaveV3ATokenCollateralV1Calls(
                super::super::MorphoAaveV3aTokenCollateralAdaptorV1Calls,
            ),
            /// Represents function calls to the MorphoAaveV3ATokenP2P V1
            #[prost(message, tag = "21")]
            MorphoAaveV3ATokenP2pV1Calls(super::super::MorphoAaveV3aTokenP2pAdaptorV1Calls),
            /// Represents function calls to the MorphoAaveV3DebtToken V1
            #[prost(message, tag = "22")]
            MorphoAaveV3DebtTokenV1Calls(super::super::MorphoAaveV3DebtTokenAdaptorV1Calls),
            /// Represents function calls to the BalancerPoolAdaptor V1
            #[prost(message, tag = "23")]
            BalancerPoolV1Calls(super::super::BalancerPoolAdaptorV1Calls),
            /// Represents function calls to the LegacyCellarAdaptor V1
            #[prost(message, tag = "24")]
            LegacyCellarV1Calls(super::super::LegacyCellarAdaptorV1Calls),
            /// Represents function calls to the DebtFTokenAdaptor V1
            #[prost(message, tag = "25")]
            DebtFTokenV1Calls(super::super::DebtFTokenAdaptorV1Calls),
            /// Represents function calls to the CollateralFTokenAdaptor V1
            #[prost(message, tag = "26")]
            CollateralFTokenV1Calls(super::super::CollateralFTokenAdaptorV1Calls),
            /// Represents function calls for the ConvexCurveAdaptorV1
            #[prost(message, tag = "27")]
            ConvexCurveV1Calls(super::super::ConvexCurveAdaptorV1Calls),
            /// Represents function calls for the CurveAdaptorV1
            #[prost(message, tag = "28")]
            CurveV1Calls(super::super::CurveAdaptorV1Calls),
            /// Represents function calls for the AuraERC4626AdaptorV1
            #[prost(message, tag = "29")]
            AuraErc4626V1Calls(super::super::AuraErc4626AdaptorV1Calls),
            /// Represents function calls for the MorphoBlueCollateralAdaptorV1
            #[prost(message, tag = "30")]
            MorphoBlueCollateralV1Calls(super::super::MorphoBlueCollateralAdaptorV1Calls),
            /// Represents function calls for the MorphoBlueDebtAdaptorV1
            #[prost(message, tag = "31")]
            MorphoBlueDebtV1Calls(super::super::MorphoBlueDebtAdaptorV1Calls),
            /// Represents function calls for the MorphoBlueSupplyAdaptorV1
            #[prost(message, tag = "32")]
            MorphoBlueSupplyV1Calls(super::super::MorphoBlueSupplyAdaptorV1Calls),
        }
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct AaveV3DebtTokenAdaptorV1FlashLoanCalls {
    #[prost(message, repeated, tag = "1")]
    pub calls: ::prost::alloc::vec::Vec<AaveV3DebtTokenAdaptorV1FlashLoan>,
}
/// Represents flash loan call data for the Balancer Pool adaptor V1, for managing pool positions on Balancer.
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct BalancerPoolAdaptorV1FlashLoan {
    #[prost(message, optional, tag = "1")]
    pub make_flash_loan: ::core::option::Option<balancer_pool_adaptor_v1_flash_loan::MakeFlashLoan>,
}
/// Nested message and enum types in `BalancerPoolAdaptorV1FlashLoan`.
pub mod balancer_pool_adaptor_v1_flash_loan {
    ///
    /// Make a flash loan
    ///
    /// Represents `function makeFlashLoan(IERC20[] tokens, uint256[] amounts, bytes memory data)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct MakeFlashLoan {
        /// The tokens to flash loan
        #[prost(string, repeated, tag = "1")]
        pub tokens: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// The amounts to flash loan
        #[prost(string, repeated, tag = "2")]
        pub amounts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// The data to flash loan
        #[prost(message, repeated, tag = "3")]
        pub data: ::prost::alloc::vec::Vec<AdaptorCallForBalancerPoolFlashLoan>,
    }
    /// NOTE: Make FlashLoan takes an array of AdaptorCall. cellar_v2.proto defines it, but also imports this file, therefore we can't import cellar_v2.proto in order to use the AdaptorCall message here. To avoid the recursive import, we duplicate the message definition.
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct AdaptorCallForBalancerPoolFlashLoan {
        /// Address of the adaptor
        #[prost(string, tag = "1")]
        pub adaptor: ::prost::alloc::string::String,
        /// The function call data for the adaptor
        #[prost(
            oneof = "adaptor_call_for_balancer_pool_flash_loan::CallData",
            tags = "2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 25, 26, 27, 28, 29, 30, 31, 32, 33"
        )]
        pub call_data: ::core::option::Option<adaptor_call_for_balancer_pool_flash_loan::CallData>,
    }
    /// Nested message and enum types in `AdaptorCallForBalancerPoolFlashLoan`.
    pub mod adaptor_call_for_balancer_pool_flash_loan {
        /// The function call data for the adaptor
        #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Oneof)]
        pub enum CallData {
            /// Represents function calls to the UniswapV3Adaptor V1
            #[prost(message, tag = "2")]
            UniswapV3V1Calls(super::super::UniswapV3AdaptorV1Calls),
            /// Represents function calls to the AaveATokenAdaptor V1
            #[prost(message, tag = "3")]
            AaveATokenV1Calls(super::super::AaveATokenAdaptorV1Calls),
            /// Represents function calls to the AavaDebtTokenAdaptor V1
            #[prost(message, tag = "4")]
            AaveDebtTokenV1Calls(super::super::AaveDebtTokenAdaptorV1Calls),
            /// Represents function calls to the CompoundCTokenAdaptor V2
            #[prost(message, tag = "5")]
            CompoundCTokenV2Calls(super::super::CompoundCTokenAdaptorV2Calls),
            /// Represents function calls to the AaveATokenV2Adaptor
            #[prost(message, tag = "6")]
            AaveATokenV2Calls(super::super::AaveATokenAdaptorV2Calls),
            /// Represents function calls to the AavaDebtTokenV2Adaptor
            #[prost(message, tag = "7")]
            AaveDebtTokenV2Calls(super::super::AaveDebtTokenAdaptorV2Calls),
            /// Represents function calls to the AaveATokenV1Adaptor
            #[prost(message, tag = "8")]
            AaveV3ATokenV1Calls(super::super::AaveV3aTokenAdaptorV1Calls),
            /// Represents function calls to the AavaDebtTokenV1Adaptor
            #[prost(message, tag = "9")]
            AaveV3DebtTokenV1Calls(super::super::AaveV3DebtTokenAdaptorV1Calls),
            /// Represents function calls to the OneInchAdaptorV1
            #[prost(message, tag = "10")]
            OneInchV1Calls(super::super::OneInchAdaptorV1Calls),
            /// Represents function calls to the FeesAndReservesAdaptorV1
            #[prost(message, tag = "11")]
            FeesAndReservesV1Calls(super::super::FeesAndReservesAdaptorV1Calls),
            /// Represents functionc alls to the ZeroXAdaptorV1
            #[prost(message, tag = "12")]
            ZeroXV1Calls(super::super::ZeroXAdaptorV1Calls),
            /// Represents function calls to the SwapWithUniswapAdaptorV1
            #[prost(message, tag = "13")]
            SwapWithUniswapV1Calls(super::super::SwapWithUniswapAdaptorV1Calls),
            /// Represents function calls to VestingSimpleAdaptor
            #[prost(message, tag = "14")]
            VestingSimpleV2Calls(super::super::VestingSimpleAdaptorV2Calls),
            /// Represents function calls to the CellarAdaptor
            #[prost(message, tag = "15")]
            CellarV1Calls(super::super::CellarAdaptorV1Calls),
            /// Represents function calls to the UniswapV3Adaptor V2
            #[prost(message, tag = "16")]
            UniswapV3V2Calls(super::super::UniswapV3AdaptorV2Calls),
            /// Represents function calls to the AaveV2EnableAssetAsCollatorAdaptor V1
            #[prost(message, tag = "17")]
            AaveV2EnableAssetAsCollateralV1Calls(
                super::super::AaveV2EnableAssetAsCollateralAdaptorV1Calls,
            ),
            /// Represents function calls to the FTokenAdaptor V1
            #[prost(message, tag = "18")]
            FTokenV1Calls(super::super::FTokenAdaptorV1Calls),
            /// Represents function calls to the MorphoAaveV2AToken V1
            #[prost(message, tag = "19")]
            MorphoAaveV2ATokenV1Calls(super::super::MorphoAaveV2aTokenAdaptorV1Calls),
            /// Represents function calls to the MorphoAaveV2DebtToken V1
            #[prost(message, tag = "20")]
            MorphoAaveV2DebtTokenV1Calls(super::super::MorphoAaveV2DebtTokenAdaptorV1Calls),
            /// Represents function calls to the MorphoAaveV3ATokenCollateral V1
            #[prost(message, tag = "21")]
            MorphoAaveV3ATokenCollateralV1Calls(
                super::super::MorphoAaveV3aTokenCollateralAdaptorV1Calls,
            ),
            /// Represents function calls to the MorphoAaveV3ATokenP2P V1
            #[prost(message, tag = "22")]
            MorphoAaveV3ATokenP2pV1Calls(super::super::MorphoAaveV3aTokenP2pAdaptorV1Calls),
            /// Represents function calls to the MorphoAaveV3DebtToken V1
            #[prost(message, tag = "23")]
            MorphoAaveV3DebtTokenV1Calls(super::super::MorphoAaveV3DebtTokenAdaptorV1Calls),
            /// Represents function calls to the LegacyCellarAdaptor V1
            #[prost(message, tag = "25")]
            LegacyCellarV1Calls(super::super::LegacyCellarAdaptorV1Calls),
            /// Represents function calls to the DebtFTokenAdaptor V1
            #[prost(message, tag = "26")]
            DebtFTokenV1Calls(super::super::DebtFTokenAdaptorV1Calls),
            /// Represents function calls to the CollateralFTokenAdaptor V1
            #[prost(message, tag = "27")]
            CollateralFTokenV1Calls(super::super::CollateralFTokenAdaptorV1Calls),
            /// Represents function calls for the ConvexCurveAdaptorV1
            #[prost(message, tag = "28")]
            ConvexCurveV1Calls(super::super::ConvexCurveAdaptorV1Calls),
            /// Represents function calls for the CurveAdaptorV1
            #[prost(message, tag = "29")]
            CurveV1Calls(super::super::CurveAdaptorV1Calls),
            /// Represents function calls for the AuraERC4626AdaptorV1
            #[prost(message, tag = "30")]
            AuraErc4626V1Calls(super::super::AuraErc4626AdaptorV1Calls),
            /// Represents function calls for the MorphoBlueCollateralAdaptorV1
            #[prost(message, tag = "31")]
            MorphoBlueCollateralV1Calls(super::super::MorphoBlueCollateralAdaptorV1Calls),
            /// Represents function calls for the MorphoBlueDebtAdaptorV1
            #[prost(message, tag = "32")]
            MorphoBlueDebtV1Calls(super::super::MorphoBlueDebtAdaptorV1Calls),
            /// Represents function calls for the MorphoBlueSupplyAdaptorV1
            #[prost(message, tag = "33")]
            MorphoBlueSupplyV1Calls(super::super::MorphoBlueSupplyAdaptorV1Calls),
        }
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct BalancerPoolAdaptorV1FlashLoanCalls {
    #[prost(message, repeated, tag = "1")]
    pub calls: ::prost::alloc::vec::Vec<BalancerPoolAdaptorV1FlashLoan>,
}
///
/// Represents a function call to a cellar that implements Cellar.sol
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct CellarV2 {
    /// The function you wish to execute on the target cellar
    #[prost(
        oneof = "cellar_v2::Function",
        tags = "1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12"
    )]
    pub function: ::core::option::Option<cellar_v2::Function>,
}
/// Nested message and enum types in `CellarV2`.
pub mod cellar_v2 {
    ///
    /// Insert a trusted position to the list of positions used by the cellar at a given index.
    ///
    /// Represents function `addPosition(uint32 index, uint32 positionId, bytes configurationData, bool inDebtArray)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct AddPosition {
        /// Index at which to add the position
        #[prost(uint32, tag = "1")]
        pub index: u32,
        /// The position's ID in the cellar registry
        #[prost(uint32, tag = "2")]
        pub position_id: u32,
        /// Data used to configure how the position behaves
        #[prost(bytes = "vec", tag = "3")]
        pub configuration_data: ::prost::alloc::vec::Vec<u8>,
        /// Whether to add position in the debt array, or the credit array.
        #[prost(bool, tag = "4")]
        pub in_debt_array: bool,
    }
    ///
    /// Allows strategists to manage their Cellar using arbitrary logic calls to adaptors.
    ///
    /// Represents function `callOnAdaptor(AdaptorCall[] memory data)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct CallOnAdaptor {
        #[prost(message, repeated, tag = "1")]
        pub data: ::prost::alloc::vec::Vec<super::AdaptorCall>,
    }
    ///
    /// Remove the position at a given index from the list of positions used by the cellar.
    ///
    /// Represents function `removePosition(uint32 index, bool inDebtArray)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct RemovePosition {
        /// Index at which to remove the position
        #[prost(uint32, tag = "1")]
        pub index: u32,
        /// Whether to remove position from the debt array, or the credit array.
        #[prost(bool, tag = "2")]
        pub in_debt_array: bool,
    }
    ///
    /// Set the holding position used of the cellar.
    ///
    /// Represents function `setHoldingPosition(uint32 positionId)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct SetHoldingPosition {
        /// ID (index) of the new holding position to use
        #[prost(uint32, tag = "1")]
        pub position_id: u32,
    }
    ///
    /// Sets the Strategists payout address.
    ///
    /// Represents function `setStrategistPayoutAddress(address payout)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct SetStrategistPayoutAddress {
        #[prost(string, tag = "1")]
        pub payout: ::prost::alloc::string::String,
    }
    ///
    /// Swap the positions at two given indeces.
    ///
    /// Represents function `swapPositions(uint32 index1, uint32 index2, bool inDebtArray)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct SwapPositions {
        /// Index of the first position
        #[prost(uint32, tag = "1")]
        pub index_1: u32,
        /// Index of the second position
        #[prost(uint32, tag = "2")]
        pub index_2: u32,
        /// Whether to switch positions in the debt array, or the credit array.
        #[prost(bool, tag = "3")]
        pub in_debt_array: bool,
    }
    ///
    /// Allows share lock period to be updated.
    ///
    /// Represents function `setShareLockPeriod(uint256 newLock)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct SetShareLockPeriod {
        #[prost(string, tag = "1")]
        pub new_lock: ::prost::alloc::string::String,
    }
    ///
    /// Changes the cellar's allowed rebalance deviation, which is the percent the total assets of a cellar may deviate
    /// during a `callOnAdaptor`(rebalance) call. The maximum allowed deviation is 100000000000000000 (0.1e18), or 10%.
    ///
    /// Represents function `setRebalanceDeviation(uint256)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct SetRebalanceDeviation {
        #[prost(string, tag = "1")]
        pub new_deviation: ::prost::alloc::string::String,
    }
    ///
    /// Allows owner to add new adaptors for the cellar to use.
    ///
    /// Represents function `setupAdaptor(address adaptor)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct SetupAdaptor {
        /// Address of the adaptor
        #[prost(string, tag = "1")]
        pub adaptor: ::prost::alloc::string::String,
    }
    ///
    /// Allows owner to set the platform fee.
    ///
    /// Represents function `setPlatformFee(uint64 platformFee)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct SetPlatformFee {
        #[prost(uint64, tag = "1")]
        pub platform_fee: u64,
    }
    ///
    /// Shutdown the cellar. Used in an emergency or if the cellar has been deprecated.
    ///
    /// Represents function `initiateShutdown()`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct InitiateShutdown {}
    ///
    /// Allows the owner to restart a shut down Cellar
    ///
    /// Represents function `liftShutdown()`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct LiftShutdown {}
    /// The function you wish to execute on the target cellar
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Oneof)]
    pub enum Function {
        /// Represents function `addPosition(uint256 index, address position)`
        #[prost(message, tag = "1")]
        AddPosition(AddPosition),
        /// Represents function `callOnAdaptor(AdaptorCall[] memory data)`
        #[prost(message, tag = "2")]
        CallOnAdaptor(CallOnAdaptor),
        /// Represents function `removePosition(uint256 index)`
        #[prost(message, tag = "3")]
        RemovePosition(RemovePosition),
        /// Represents function `setHoldingPosition(uint32 position_id)`
        #[prost(message, tag = "4")]
        SetHoldingPosition(SetHoldingPosition),
        /// Represents function `setStrategistPayoutAddress(address payout)`
        #[prost(message, tag = "5")]
        SetStrategistPayoutAddress(SetStrategistPayoutAddress),
        /// Represents function `swapPositions(uint256 index1, uint256 index2)`
        #[prost(message, tag = "6")]
        SwapPositions(SwapPositions),
        /// Represents function `setRebalanceDeviation(uint265)`
        #[prost(message, tag = "7")]
        SetRebalanceDeviation(SetRebalanceDeviation),
        /// Represents function `setShareLockPeriod(uint256 newLock)`
        #[prost(message, tag = "8")]
        SetShareLockPeriod(SetShareLockPeriod),
        /// Represents function `setupAdaptor(address _adaptor)`
        #[prost(message, tag = "9")]
        SetupAdaptor(SetupAdaptor),
        /// Represents function `setPlatformFee(uint64 platformFee)`
        #[prost(message, tag = "10")]
        SetPlatformFee(SetPlatformFee),
        /// Represents function `initiateShutdown()`
        #[prost(message, tag = "11")]
        InitiateShutdown(InitiateShutdown),
        /// Represents function `liftShutdown()`
        #[prost(message, tag = "12")]
        LiftShutdown(LiftShutdown),
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct CellarV22 {
    #[prost(oneof = "cellar_v2_2::CallType", tags = "1, 2")]
    pub call_type: ::core::option::Option<cellar_v2_2::CallType>,
}
/// Nested message and enum types in `CellarV2_2`.
pub mod cellar_v2_2 {
    /// The function you wish to execute on the target cellar
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct FunctionCall {
        #[prost(
            oneof = "function_call::Function",
            tags = "1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16"
        )]
        pub function: ::core::option::Option<function_call::Function>,
    }
    /// Nested message and enum types in `FunctionCall`.
    pub mod function_call {
        #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Oneof)]
        pub enum Function {
            /// Represents function `addPosition(uint256 index, address position)`
            #[prost(message, tag = "1")]
            AddPosition(super::AddPosition),
            /// Represents function `callOnAdaptor(AdaptorCall[] memory data)`
            #[prost(message, tag = "2")]
            CallOnAdaptor(super::CallOnAdaptor),
            /// Represents function `removePosition(uint256 index)`
            #[prost(message, tag = "3")]
            RemovePosition(super::RemovePosition),
            /// Represents function `setHoldingPosition(uint32 position_id)`
            #[prost(message, tag = "4")]
            SetHoldingPosition(super::SetHoldingPosition),
            /// Represents function `setStrategistPayoutAddress(address payout)`
            #[prost(message, tag = "5")]
            SetStrategistPayoutAddress(super::SetStrategistPayoutAddress),
            /// Represents function `swapPositions(uint256 index1, uint256 index2)`
            #[prost(message, tag = "6")]
            SwapPositions(super::SwapPositions),
            /// Represents function `setRebalanceDeviation(uint265)`
            #[prost(message, tag = "7")]
            SetRebalanceDeviation(super::SetRebalanceDeviation),
            /// Represents function `setShareLockPeriod(uint256 newLock)`
            #[prost(message, tag = "8")]
            SetShareLockPeriod(super::SetShareLockPeriod),
            /// Represents function `initiateShutdown()`
            #[prost(message, tag = "9")]
            InitiateShutdown(super::InitiateShutdown),
            /// Represents function `setStrategistPlatformCut(uint64 cut)`
            #[prost(message, tag = "10")]
            SetStrategistPlatformCut(super::SetStrategistPlatformCut),
            /// Represents function `liftShutdown()`
            #[prost(message, tag = "11")]
            LiftShutdown(super::LiftShutdown),
            /// Represents function `addAdaptorToCatalogue(address adaptor)`
            #[prost(message, tag = "12")]
            AddAdaptorToCatalogue(super::AddAdaptorToCatalogue),
            /// Represents function `addPositionToCatalogue(uint32 positionId)`
            #[prost(message, tag = "13")]
            AddPositionToCatalogue(super::AddPositionToCatalogue),
            /// Represents function `removeAdaptorFromCatalogue(address adaptor)`
            #[prost(message, tag = "14")]
            RemoveAdaptorFromCatalogue(super::RemoveAdaptorFromCatalogue),
            /// Represents function `removePositionFromCatalogue(uint32 positionId)`
            #[prost(message, tag = "15")]
            RemovePositionFromCatalogue(super::RemovePositionFromCatalogue),
            /// Represents function `cachePriceRouter(bool checkTotalAssets, uint16 allowableRange)`
            #[prost(message, tag = "16")]
            CachePriceRouter(super::CachePriceRouter),
        }
    }
    ///
    /// Insert a trusted position to the list of positions used by the cellar at a given index.
    ///
    /// Represents function `addPosition(uint32 index, uint32 positionId, bytes configurationData, bool inDebtArray)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct AddPosition {
        /// Index at which to add the position
        #[prost(uint32, tag = "1")]
        pub index: u32,
        /// The position's ID in the cellar registry
        #[prost(uint32, tag = "2")]
        pub position_id: u32,
        /// Data used to configure how the position behaves
        #[prost(bytes = "vec", tag = "3")]
        pub configuration_data: ::prost::alloc::vec::Vec<u8>,
        /// Whether to add position in the debt array, or the credit array.
        #[prost(bool, tag = "4")]
        pub in_debt_array: bool,
    }
    ///
    /// Allows strategists to manage their Cellar using arbitrary logic calls to adaptors.
    ///
    /// Represents function `callOnAdaptor(AdaptorCall[] memory data)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct CallOnAdaptor {
        #[prost(message, repeated, tag = "1")]
        pub data: ::prost::alloc::vec::Vec<super::AdaptorCall>,
    }
    ///
    /// Remove the position at a given index from the list of positions used by the cellar.
    ///
    /// Represents function `removePosition(uint32 index, bool inDebtArray)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct RemovePosition {
        /// Index at which to remove the position
        #[prost(uint32, tag = "1")]
        pub index: u32,
        /// Whether to remove position from the debt array, or the credit array.
        #[prost(bool, tag = "2")]
        pub in_debt_array: bool,
    }
    ///
    /// Set the holding position used of the cellar.
    ///
    /// Represents function `setHoldingIndex(uint8 index)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct SetHoldingPosition {
        /// ID (index) of the new holding position to use
        #[prost(uint32, tag = "1")]
        pub position_id: u32,
    }
    ///
    /// Sets the Strategists payout address.
    ///
    /// Represents function `setStrategistPayoutAddress(address payout)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct SetStrategistPayoutAddress {
        #[prost(string, tag = "1")]
        pub payout: ::prost::alloc::string::String,
    }
    ///
    /// Swap the positions at two given indeces.
    ///
    /// Represents function `swapPositions(uint32 index1, uint32 index2)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct SwapPositions {
        /// Index of the first position
        #[prost(uint32, tag = "1")]
        pub index_1: u32,
        /// Index of the second position
        #[prost(uint32, tag = "2")]
        pub index_2: u32,
        /// Whether to switch positions in the debt array, or the credit array.
        #[prost(bool, tag = "3")]
        pub in_debt_array: bool,
    }
    ///
    /// Allows share lock period to be updated.
    ///
    /// Represents function `setShareLockPeriod()`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct SetShareLockPeriod {
        #[prost(string, tag = "1")]
        pub new_lock: ::prost::alloc::string::String,
    }
    ///
    /// Changes the cellar's allowed rebalance deviation, which is the percent the total assets of a cellar may deviate
    /// during a `callOnAdaptor`(rebalance) call. The maximum allowed deviation is 100000000000000000 (0.1e18), or 10%.
    ///
    /// Represents function `setRebalanceDeviation(uint256)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct SetRebalanceDeviation {
        #[prost(string, tag = "1")]
        pub new_deviation: ::prost::alloc::string::String,
    }
    ///
    /// Shutdown the cellar. Used in an emergency or if the cellar has been deprecated.
    ///
    /// Represents function `initiateShutdown()`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct InitiateShutdown {}
    ///
    /// Allows strategist to set the platform cut for the cellar.
    ///
    /// Represents function `setStrategistPlatformCut(uint64 cut)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct SetStrategistPlatformCut {
        /// The new strategist platform cut
        #[prost(uint64, tag = "1")]
        pub new_cut: u64,
    }
    ///
    /// Allows the owner to restart a shut down Cellar
    ///
    /// Represents function `liftShutdown()`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct LiftShutdown {}
    ///
    /// Allows the owner to add an adaptor to the Cellar's adaptor catalogue
    ///
    /// Represents function `addAdaptorToCatalogue(address adaptor)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct AddAdaptorToCatalogue {
        #[prost(string, tag = "1")]
        pub adaptor: ::prost::alloc::string::String,
    }
    ///
    /// Allows caller to call multiple functions in a single TX.
    ///
    /// Represents function `multicall(bytes[] data)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct Multicall {
        #[prost(message, repeated, tag = "1")]
        pub function_calls: ::prost::alloc::vec::Vec<FunctionCall>,
    }
    ///
    /// Allows the owner to add a position to the Cellar's position catalogue
    ///
    /// Represents function `addPositionToCatalogue(uint32 positionId)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct AddPositionToCatalogue {
        #[prost(uint32, tag = "1")]
        pub position_id: u32,
    }
    ///
    /// Allows callers to remove adaptors from this cellar's catalogue
    ///
    /// Represents function `removeAdaptorFromCatalogue(address adaptor)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct RemoveAdaptorFromCatalogue {
        #[prost(string, tag = "1")]
        pub adaptor: ::prost::alloc::string::String,
    }
    ///
    /// Allows caller to remove positions from this cellar's catalogue
    ///
    /// Represents function `removePositionFromCatalogue(uint32 positionId)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct RemovePositionFromCatalogue {
        #[prost(uint32, tag = "1")]
        pub position_id: u32,
    }
    ///
    /// Updates the cellar to use the latest price router in the registry.
    ///
    /// Represents function `cachePriceRouter(bool checkTotalAssets, uint16 allowableRange)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct CachePriceRouter {
        /// Whether to check the total assets of the cellar
        #[prost(bool, tag = "1")]
        pub check_total_assets: bool,
        /// The allowable range of the cellar's total assets to deviate between old and new routers
        #[prost(uint32, tag = "2")]
        pub allowable_range: u32,
    }
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Oneof)]
    pub enum CallType {
        /// Represents a single function call
        #[prost(message, tag = "1")]
        FunctionCall(FunctionCall),
        /// Represents multiple, ordered function calls
        #[prost(message, tag = "2")]
        Multicall(Multicall),
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct CellarV25 {
    #[prost(oneof = "cellar_v2_5::CallType", tags = "1, 2")]
    pub call_type: ::core::option::Option<cellar_v2_5::CallType>,
}
/// Nested message and enum types in `CellarV2_5`.
pub mod cellar_v2_5 {
    /// The function you wish to execute on the target cellar
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct FunctionCall {
        #[prost(
            oneof = "function_call::Function",
            tags = "1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22"
        )]
        pub function: ::core::option::Option<function_call::Function>,
    }
    /// Nested message and enum types in `FunctionCall`.
    pub mod function_call {
        #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Oneof)]
        pub enum Function {
            /// Represents function `addPosition(uint256 index, address position)`
            #[prost(message, tag = "1")]
            AddPosition(super::AddPosition),
            /// Represents function `callOnAdaptor(AdaptorCall[] memory data)`
            #[prost(message, tag = "2")]
            CallOnAdaptor(super::CallOnAdaptor),
            /// Represents function `removePosition(uint256 index)`
            #[prost(message, tag = "3")]
            RemovePosition(super::RemovePosition),
            /// Represents function `setHoldingPosition(uint32 position_id)`
            #[prost(message, tag = "4")]
            SetHoldingPosition(super::SetHoldingPosition),
            /// Represents function `setStrategistPayoutAddress(address payout)`
            #[prost(message, tag = "5")]
            SetStrategistPayoutAddress(super::SetStrategistPayoutAddress),
            /// Represents function `swapPositions(uint256 index1, uint256 index2)`
            #[prost(message, tag = "6")]
            SwapPositions(super::SwapPositions),
            /// Represents function `setRebalanceDeviation(uint265)`
            #[prost(message, tag = "7")]
            SetRebalanceDeviation(super::SetRebalanceDeviation),
            /// Represents function `setShareLockPeriod(uint256 newLock)`
            #[prost(message, tag = "8")]
            SetShareLockPeriod(super::SetShareLockPeriod),
            /// Represents function `initiateShutdown()`
            #[prost(message, tag = "9")]
            InitiateShutdown(super::InitiateShutdown),
            /// Represents function `setStrategistPlatformCut(uint64 cut)`
            #[prost(message, tag = "10")]
            SetStrategistPlatformCut(super::SetStrategistPlatformCut),
            /// Represents function `liftShutdown()`
            #[prost(message, tag = "11")]
            LiftShutdown(super::LiftShutdown),
            /// Represents function `addAdaptorToCatalogue(address adaptor)`
            #[prost(message, tag = "12")]
            AddAdaptorToCatalogue(super::AddAdaptorToCatalogue),
            /// Represents function `addPositionToCatalogue(uint32 positionId)`
            #[prost(message, tag = "13")]
            AddPositionToCatalogue(super::AddPositionToCatalogue),
            /// Represents function `removeAdaptorFromCatalogue(address adaptor)`
            #[prost(message, tag = "14")]
            RemoveAdaptorFromCatalogue(super::RemoveAdaptorFromCatalogue),
            /// Represents function `removePositionFromCatalogue(uint32 positionId)`
            #[prost(message, tag = "15")]
            RemovePositionFromCatalogue(super::RemovePositionFromCatalogue),
            /// Represents function `increaseShareSupplyCap(uint192)
            #[prost(message, tag = "16")]
            IncreaseShareSupplyCap(super::IncreaseShareSupplyCap),
            /// Represents function `decreaseShareSupplyCap(uint192)
            #[prost(message, tag = "17")]
            DecreaseShareSupplyCap(super::DecreaseShareSupplyCap),
            /// Represents function `setSharePriceOracle(uint256, address)
            #[prost(message, tag = "18")]
            SetSharePriceOracle(super::SetSharePriceOracle),
            /// Represents function `cachePriceRouter(bool checkTotalAssets, uint16 allowableRange, address expectedPriceRouter)`
            #[prost(message, tag = "19")]
            CachePriceRouter(super::CachePriceRouter),
            /// Represents function `forcePositionOut(uint32 index, uint32 positionId, bool inDebtArray)`
            #[prost(message, tag = "20")]
            ForcePositionOut(super::ForcePositionOut),
            /// Represents function `setAlternativeAssetData(ERC20 _alternativeAsset, uint32 _alternativeHoldingPosition, uint32 _alternativeAssetFee)`
            #[prost(message, tag = "21")]
            SetAlternativeAssetData(super::SetAlternativeAssetData),
            /// Represents function `setDropAlternativeAssetData(ERC20 _alternativeAsset)`
            #[prost(message, tag = "22")]
            DropAlternativeAssetData(super::DropAlternativeAssetData),
        }
    }
    ///
    /// Insert a trusted position to the list of positions used by the cellar at a given index.
    ///
    /// Represents function `addPosition(uint32 index, uint32 positionId, bytes configurationData, bool inDebtArray)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct AddPosition {
        /// Index at which to add the position
        #[prost(uint32, tag = "1")]
        pub index: u32,
        /// The position's ID in the cellar registry
        #[prost(uint32, tag = "2")]
        pub position_id: u32,
        /// Data used to configure how the position behaves
        #[prost(bytes = "vec", tag = "3")]
        pub configuration_data: ::prost::alloc::vec::Vec<u8>,
        /// Whether to add position in the debt array, or the credit array.
        #[prost(bool, tag = "4")]
        pub in_debt_array: bool,
    }
    ///
    /// Allows strategists to manage their Cellar using arbitrary logic calls to adaptors.
    ///
    /// Represents function `callOnAdaptor(AdaptorCall[] memory data)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct CallOnAdaptor {
        #[prost(message, repeated, tag = "1")]
        pub data: ::prost::alloc::vec::Vec<super::AdaptorCall>,
    }
    ///
    /// Remove the position at a given index from the list of positions used by the cellar.
    ///
    /// Represents function `removePosition(uint32 index, bool inDebtArray)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct RemovePosition {
        /// Index at which to remove the position
        #[prost(uint32, tag = "1")]
        pub index: u32,
        /// Whether to remove position from the debt array, or the credit array.
        #[prost(bool, tag = "2")]
        pub in_debt_array: bool,
    }
    ///
    /// Set the holding position used of the cellar.
    ///
    /// Represents function `setHoldingIndex(uint8 index)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct SetHoldingPosition {
        /// ID (index) of the new holding position to use
        #[prost(uint32, tag = "1")]
        pub position_id: u32,
    }
    ///
    /// Sets the Strategists payout address.
    ///
    /// Represents function `setStrategistPayoutAddress(address payout)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct SetStrategistPayoutAddress {
        #[prost(string, tag = "1")]
        pub payout: ::prost::alloc::string::String,
    }
    ///
    /// Swap the positions at two given indeces.
    ///
    /// Represents function `swapPositions(uint32 index1, uint32 index2)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct SwapPositions {
        /// Index of the first position
        #[prost(uint32, tag = "1")]
        pub index_1: u32,
        /// Index of the second position
        #[prost(uint32, tag = "2")]
        pub index_2: u32,
        /// Whether to switch positions in the debt array, or the credit array.
        #[prost(bool, tag = "3")]
        pub in_debt_array: bool,
    }
    ///
    /// Allows share lock period to be updated.
    ///
    /// Represents function `setShareLockPeriod()`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct SetShareLockPeriod {
        #[prost(string, tag = "1")]
        pub new_lock: ::prost::alloc::string::String,
    }
    ///
    /// Changes the cellar's allowed rebalance deviation, which is the percent the total assets of a cellar may deviate
    /// during a `callOnAdaptor`(rebalance) call. The maximum allowed deviation is 100000000000000000 (0.1e18), or 10%.
    ///
    /// Represents function `setRebalanceDeviation(uint256)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct SetRebalanceDeviation {
        #[prost(string, tag = "1")]
        pub new_deviation: ::prost::alloc::string::String,
    }
    ///
    /// Shutdown the cellar. Used in an emergency or if the cellar has been deprecated.
    ///
    /// Represents function `initiateShutdown()`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct InitiateShutdown {}
    ///
    /// Allows strategist to set the platform cut for the cellar.
    ///
    /// Represents function `setStrategistPlatformCut(uint64 cut)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct SetStrategistPlatformCut {
        /// The new strategist platform cut
        #[prost(uint64, tag = "1")]
        pub new_cut: u64,
    }
    ///
    /// Allows the owner to restart a shut down Cellar
    ///
    /// Represents function `liftShutdown()`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct LiftShutdown {}
    ///
    /// Allows the owner to add an adaptor to the Cellar's adaptor catalogue
    ///
    /// Represents function `addAdaptorToCatalogue(address adaptor)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct AddAdaptorToCatalogue {
        #[prost(string, tag = "1")]
        pub adaptor: ::prost::alloc::string::String,
    }
    ///
    /// Allows caller to call multiple functions in a single TX.
    ///
    /// Represents function `multicall(bytes[] data)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct Multicall {
        #[prost(message, repeated, tag = "1")]
        pub function_calls: ::prost::alloc::vec::Vec<FunctionCall>,
    }
    ///
    /// Allows the owner to add a position to the Cellar's position catalogue
    ///
    /// Represents function `addPositionToCatalogue(uint32 positionId)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct AddPositionToCatalogue {
        #[prost(uint32, tag = "1")]
        pub position_id: u32,
    }
    ///
    /// Allows callers to remove adaptors from this cellar's catalogue
    ///
    /// Represents function `removeAdaptorFromCatalogue(address adaptor)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct RemoveAdaptorFromCatalogue {
        #[prost(string, tag = "1")]
        pub adaptor: ::prost::alloc::string::String,
    }
    ///
    /// Allows caller to remove positions from this cellar's catalogue
    ///
    /// Represents function `removePositionFromCatalogue(uint32 positionId)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct RemovePositionFromCatalogue {
        #[prost(uint32, tag = "1")]
        pub position_id: u32,
    }
    ///
    /// Allows strategist to increase the share supply cap
    ///
    /// Represents function `increaseShareSupplyCap(uint192)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct IncreaseShareSupplyCap {
        #[prost(string, tag = "1")]
        pub new_cap: ::prost::alloc::string::String,
    }
    ///
    /// Allows strategist to decrease the share supply cap
    ///
    /// Represents function `decreaseShareSupplyCap(uint192)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct DecreaseShareSupplyCap {
        #[prost(string, tag = "1")]
        pub new_cap: ::prost::alloc::string::String,
    }
    ///
    /// Allows strategist to set the share price oracle contract
    ///
    /// Represents function `setSharePriceOracle(uint256, address)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct SetSharePriceOracle {
        /// The oracle registry ID
        #[prost(string, tag = "1")]
        pub registry_id: ::prost::alloc::string::String,
        /// The oracle contract address
        #[prost(string, tag = "2")]
        pub oracle: ::prost::alloc::string::String,
    }
    ///
    /// Updates the cellar to use the latest price router in the registry.
    ///
    /// Represents function `cachePriceRouter(bool checkTotalAssets, uint16 allowableRange, address expectedPriceRouter)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct CachePriceRouter {
        /// Whether to check the total assets of the cellar
        #[prost(bool, tag = "1")]
        pub check_total_assets: bool,
        /// The allowable range of the cellar's total assets to deviate between old and new routers
        #[prost(uint32, tag = "2")]
        pub allowable_range: u32,
        /// The expected price router address
        #[prost(string, tag = "3")]
        pub expected_price_router: ::prost::alloc::string::String,
    }
    ///
    /// Forceably remove a position from the cellar without checking its balance is zero.
    ///
    /// Represents function `forcePositionOut(uint32 index, uint32 positionId, bool inDebtArray)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct ForcePositionOut {
        /// Index of the position.
        #[prost(uint32, tag = "1")]
        pub index: u32,
        /// Position ID to force out.
        #[prost(uint32, tag = "2")]
        pub position_id: u32,
        /// Whether to switch positions in the debt array, or the credit array.
        #[prost(bool, tag = "3")]
        pub in_debt_array: bool,
    }
    ///
    /// Allows the strategist to add, or update an existing alternative asset deposit.
    ///
    /// Represents function `setAlternativeAssetData(ERC20 _alternativeAsset, uint32 _alternativeHoldingPosition, uint32 _alternativeAssetFee)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct SetAlternativeAssetData {
        /// The address of the alternative asset
        #[prost(string, tag = "1")]
        pub alternative_asset: ::prost::alloc::string::String,
        /// The holding position to direct alternative asset deposits to
        #[prost(uint32, tag = "2")]
        pub alternative_holding_position: u32,
        /// The fee to charge for depositing this alternative asset
        #[prost(uint32, tag = "3")]
        pub alternative_asset_fee: u32,
    }
    ///
    /// Allows the strategist to stop an alternative asset from being deposited.
    ///
    /// Represents function `dropAlternativeAssetData(ERC20 _alternativeAsset)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct DropAlternativeAssetData {
        /// The address of the alternative asset
        #[prost(string, tag = "1")]
        pub alternative_asset: ::prost::alloc::string::String,
    }
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Oneof)]
    pub enum CallType {
        /// Represents a single function call
        #[prost(message, tag = "1")]
        FunctionCall(FunctionCall),
        /// Represents multiple, ordered function calls
        #[prost(message, tag = "2")]
        Multicall(Multicall),
    }
}
/// Represents a call to adaptor an. The cellar must be authorized to call the target adaptor.
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct AdaptorCall {
    /// Address of the adaptor
    #[prost(string, tag = "1")]
    pub adaptor: ::prost::alloc::string::String,
    /// The function call data for the adaptor
    #[prost(
        oneof = "adaptor_call::CallData",
        tags = "2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35"
    )]
    pub call_data: ::core::option::Option<adaptor_call::CallData>,
}
/// Nested message and enum types in `AdaptorCall`.
pub mod adaptor_call {
    /// The function call data for the adaptor
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Oneof)]
    pub enum CallData {
        /// Represents function calls to the UniswapV3Adaptor V1
        #[prost(message, tag = "2")]
        UniswapV3V1Calls(super::UniswapV3AdaptorV1Calls),
        /// Represents function calls to the AaveATokenAdaptor V1
        #[prost(message, tag = "3")]
        AaveATokenV1Calls(super::AaveATokenAdaptorV1Calls),
        /// Represents function calls to the AavaDebtTokenAdaptor V1
        #[prost(message, tag = "4")]
        AaveDebtTokenV1Calls(super::AaveDebtTokenAdaptorV1Calls),
        /// Represents function calls to the CompoundCTokenAdaptor V2
        #[prost(message, tag = "5")]
        CompoundCTokenV2Calls(super::CompoundCTokenAdaptorV2Calls),
        /// Represents function calls to the AaveATokenV2Adaptor
        #[prost(message, tag = "6")]
        AaveATokenV2Calls(super::AaveATokenAdaptorV2Calls),
        /// Represents function calls to the AavaDebtTokenV2Adaptor
        #[prost(message, tag = "7")]
        AaveDebtTokenV2Calls(super::AaveDebtTokenAdaptorV2Calls),
        /// Represents function calls to the AaveATokenV1Adaptor
        #[prost(message, tag = "8")]
        AaveV3ATokenV1Calls(super::AaveV3aTokenAdaptorV1Calls),
        /// Represents function calls to the AavaDebtTokenV1Adaptor
        #[prost(message, tag = "9")]
        AaveV3DebtTokenV1Calls(super::AaveV3DebtTokenAdaptorV1Calls),
        /// Represents function calls to the OneInchAdaptorV1
        #[prost(message, tag = "10")]
        OneInchV1Calls(super::OneInchAdaptorV1Calls),
        /// Represents function calls to the FeesAndReservesAdaptorV1
        #[prost(message, tag = "11")]
        FeesAndReservesV1Calls(super::FeesAndReservesAdaptorV1Calls),
        /// Represents functionc alls to the ZeroXAdaptorV1
        #[prost(message, tag = "12")]
        ZeroXV1Calls(super::ZeroXAdaptorV1Calls),
        /// Represents function calls to the SwapWithUniswapAdaptorV1
        #[prost(message, tag = "13")]
        SwapWithUniswapV1Calls(super::SwapWithUniswapAdaptorV1Calls),
        /// Represents function calls to VestingSimpleAdaptor
        #[prost(message, tag = "14")]
        VestingSimpleV2Calls(super::VestingSimpleAdaptorV2Calls),
        /// Represents function calls to the CellarAdaptor
        #[prost(message, tag = "15")]
        CellarV1Calls(super::CellarAdaptorV1Calls),
        /// Represents function calls to the UniswapV3Adaptor V2
        #[prost(message, tag = "16")]
        UniswapV3V2Calls(super::UniswapV3AdaptorV2Calls),
        /// Represents function calls to the AaveV2EnableAssetAsCollatorAdaptor V1
        #[prost(message, tag = "17")]
        AaveV2EnableAssetAsCollateralV1Calls(super::AaveV2EnableAssetAsCollateralAdaptorV1Calls),
        /// Represents function calls to the FTokenAdaptor V1
        #[prost(message, tag = "18")]
        FTokenV1Calls(super::FTokenAdaptorV1Calls),
        /// Represents function calls to the MorphoAaveV2AToken V1
        #[prost(message, tag = "19")]
        MorphoAaveV2ATokenV1Calls(super::MorphoAaveV2aTokenAdaptorV1Calls),
        /// Represents function calls to the MorphoAaveV2DebtToken V1
        #[prost(message, tag = "20")]
        MorphoAaveV2DebtTokenV1Calls(super::MorphoAaveV2DebtTokenAdaptorV1Calls),
        /// Represents function calls to the MorphoAaveV3ATokenCollateral V1
        #[prost(message, tag = "21")]
        MorphoAaveV3ATokenCollateralV1Calls(super::MorphoAaveV3aTokenCollateralAdaptorV1Calls),
        /// Represents function calls to the MorphoAaveV3ATokenP2P V1
        #[prost(message, tag = "22")]
        MorphoAaveV3ATokenP2pV1Calls(super::MorphoAaveV3aTokenP2pAdaptorV1Calls),
        /// Represents function calls to the MorphoAaveV3DebtToken V1
        #[prost(message, tag = "23")]
        MorphoAaveV3DebtTokenV1Calls(super::MorphoAaveV3DebtTokenAdaptorV1Calls),
        /// Represents function calls to the BalancerPoolAdaptor V1
        #[prost(message, tag = "24")]
        BalancerPoolV1Calls(super::BalancerPoolAdaptorV1Calls),
        /// Represents function calls to the LegacyCellarAdaptor V1
        #[prost(message, tag = "25")]
        LegacyCellarV1Calls(super::LegacyCellarAdaptorV1Calls),
        /// Represents function calls to the DebtFTokenAdaptor V1
        #[prost(message, tag = "26")]
        DebtFTokenV1Calls(super::DebtFTokenAdaptorV1Calls),
        /// Represents function calls to the CollateralFTokenAdaptor V1
        #[prost(message, tag = "27")]
        CollateralFTokenV1Calls(super::CollateralFTokenAdaptorV1Calls),
        /// Represents function call for the AaveV3DebtTokenAdaptorV1
        #[prost(message, tag = "28")]
        AaveV3DebtTokenV1FlashLoanCalls(super::AaveV3DebtTokenAdaptorV1FlashLoanCalls),
        /// Represents function call for the BalancerPoolAdaptorV1
        #[prost(message, tag = "29")]
        BalancerPoolV1FlashLoanCalls(super::BalancerPoolAdaptorV1FlashLoanCalls),
        /// Represents function calls for the ConvexCurveAdaptorV1
        #[prost(message, tag = "30")]
        ConvexCurveV1Calls(super::ConvexCurveAdaptorV1Calls),
        /// Represents function calls for the CurveAdaptorV1
        #[prost(message, tag = "31")]
        CurveV1Calls(super::CurveAdaptorV1Calls),
        /// Represents function calls for the AuraERC4626AdaptorV1
        #[prost(message, tag = "32")]
        AuraErc4626V1Calls(super::AuraErc4626AdaptorV1Calls),
        /// Represents function calls for the MorphoBlueCollateralAdaptorV1
        #[prost(message, tag = "33")]
        MorphoBlueCollateralV1Calls(super::MorphoBlueCollateralAdaptorV1Calls),
        /// Represents function calls for the MorphoBlueDebtAdaptorV1
        #[prost(message, tag = "34")]
        MorphoBlueDebtV1Calls(super::MorphoBlueDebtAdaptorV1Calls),
        /// Represents function calls for the MorphoBlueSupplyAdaptorV1
        #[prost(message, tag = "35")]
        MorphoBlueSupplyV1Calls(super::MorphoBlueSupplyAdaptorV1Calls),
    }
}
///
/// Represents a function call to a cellar that implements Cellar.sol
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct CellarV1 {
    /// The function you wish to execute on the target cellar
    #[prost(
        oneof = "cellar_v1::Function",
        tags = "1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12"
    )]
    pub function: ::core::option::Option<cellar_v1::Function>,
}
/// Nested message and enum types in `CellarV1`.
pub mod cellar_v1 {
    ///
    /// Insert a trusted position to the list of positions used by the cellar at a given index.
    ///
    /// Represents function `addPosition(uint256 index, address position)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct AddPosition {
        /// Index at which to add the position
        #[prost(string, tag = "1")]
        pub index: ::prost::alloc::string::String,
        /// Address of the position to add
        #[prost(string, tag = "2")]
        pub position: ::prost::alloc::string::String,
    }
    ///
    /// Push a trusted position to the end of the list of positions used by the cellar. If you
    ///know you are going to add a position to the end of the array, this is more efficient then
    ///`addPosition`.
    ///
    /// Represents function `pushPosition(address position)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct PushPosition {
        /// Address of the position to push
        #[prost(string, tag = "1")]
        pub position: ::prost::alloc::string::String,
    }
    ///
    /// Remove the position at a given index from the list of positions used by the cellar.
    ///
    /// Represents function `removePosition(uint256 index)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct RemovePosition {
        /// Index at which to remove the position
        #[prost(string, tag = "1")]
        pub index: ::prost::alloc::string::String,
    }
    ///
    /// Set the holding position used by the cellar.
    ///
    /// Represents function `setHoldingPosition(address newHoldingPosition)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct SetHoldingPosition {
        /// Address of the new holding position to use
        #[prost(string, tag = "1")]
        pub new_holding_position: ::prost::alloc::string::String,
    }
    ///
    /// Move assets between positions. To move assets from/to this cellar's holdings, specify
    ///the address of this cellar as the `fromPosition`/`toPosition`.
    ///
    /// Represents function `rebalance(address fromPosition, address toPosition,
    ///  uint256 assetsFrom, SwapRouter.Exchange exchange, bytes calldata params)
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct Rebalance {
        #[prost(string, tag = "1")]
        pub from_position: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub to_position: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub assets_from: ::prost::alloc::string::String,
        #[prost(enumeration = "super::Exchange", tag = "4")]
        pub exchange: i32,
        #[prost(message, optional, tag = "5")]
        pub params: ::core::option::Option<super::SwapParams>,
    }
    ///
    /// Sets the Strategists payout address.
    ///
    /// Represents function `setStrategistPayoutAddress(address payout)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct SetStrategistPayoutAddress {
        #[prost(string, tag = "1")]
        pub payout: ::prost::alloc::string::String,
    }
    ///
    /// Set the withdraw type used by the cellar.
    ///
    /// Represents function `setWithdrawType(WithdrawType newWithdrawType)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct SetWithdrawType {
        /// The withdraw type to use for the cellar
        #[prost(enumeration = "WithdrawType", tag = "1")]
        pub new_withdraw_type: i32,
    }
    ///
    /// Swap the positions at two given indeces.
    ///
    /// Represents function `swapPositions(uint256 index1, uint256 index2)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct SwapPositions {
        /// Index of the first position
        #[prost(string, tag = "1")]
        pub index_1: ::prost::alloc::string::String,
        /// Index of the second position
        #[prost(string, tag = "2")]
        pub index_2: ::prost::alloc::string::String,
    }
    ///
    /// Set the per-wallet deposit limit. Uses the same decimals as the current asset.
    ///
    /// Represents function `setDepositLimit()`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct SetDepositLimit {
        #[prost(string, tag = "1")]
        pub new_limit: ::prost::alloc::string::String,
    }
    ///
    /// Set the maximum liquidity that cellar can manage. Uses the same decimals as the current asset.
    ///
    /// Represents function `setLiquidityLimit()`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct SetLiquidityLimit {
        #[prost(string, tag = "1")]
        pub new_limit: ::prost::alloc::string::String,
    }
    ///
    /// Allows share lock period to be updated.
    ///
    /// Represents function `setShareLockPeriod()`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct SetShareLockPeriod {
        #[prost(string, tag = "1")]
        pub new_lock: ::prost::alloc::string::String,
    }
    ///
    ///
    ///
    /// Represents function `setRebalanceDeviation(uint256)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct SetRebalanceDeviation {
        #[prost(string, tag = "1")]
        pub new_deviation: ::prost::alloc::string::String,
    }
    ///
    /// Represents the withdraw type to use for the cellar
    #[derive(
        serde::Deserialize,
        serde::Serialize,
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration,
    )]
    #[repr(i32)]
    pub enum WithdrawType {
        Unspecified = 0,
        Orderly = 1,
        Proportional = 2,
    }
    /// The function you wish to execute on the target cellar
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Oneof)]
    pub enum Function {
        /// Represents function `addPosition(uint256 index, address position)`
        #[prost(message, tag = "1")]
        AddPosition(AddPosition),
        /// Represents function `pushPosition(address position)`
        #[prost(message, tag = "2")]
        PushPosition(PushPosition),
        /// Represents function `removePosition(uint256 index)`
        #[prost(message, tag = "3")]
        RemovePosition(RemovePosition),
        /// Represents function `setHoldingPosition(address newHoldingPosition)`
        #[prost(message, tag = "4")]
        SetHoldingPosition(SetHoldingPosition),
        ///
        /// Represents function `rebalance(address fromPosition, address toPosition,
        ///uint256 assetsFrom, SwapRouter.Exchange exchange, bytes calldata params)`
        #[prost(message, tag = "5")]
        Rebalance(Rebalance),
        /// Represents function `setStrategistPayoutAddress(address payout)`
        #[prost(message, tag = "6")]
        SetStrategistPayoutAddress(SetStrategistPayoutAddress),
        /// Represents function `setWithdrawType(WithdrawType newWithdrawType)`
        #[prost(message, tag = "7")]
        SetWithdrawType(SetWithdrawType),
        /// Represents function `swapPositions(uint256 index1, uint256 index2)`
        #[prost(message, tag = "8")]
        SwapPositions(SwapPositions),
        /// Represents function `setDepositLimit()`
        #[prost(message, tag = "9")]
        SetDepositLimit(SetDepositLimit),
        /// Represents function `setLiquidityLimit()`
        #[prost(message, tag = "10")]
        SetLiquidityLimit(SetLiquidityLimit),
        /// Represents function `setShareLockPeriod()`
        #[prost(message, tag = "11")]
        SetShareLockPeriod(SetShareLockPeriod),
        /// Represents function `setRebalanceDeviation(uint265)`
        #[prost(message, tag = "12")]
        SetRebalanceDeviation(SetRebalanceDeviation),
    }
}
///
/// Represents a single function call on a particular Cellar
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct SubmitRequest {
    /// The ID (currently simply an Ethereum address) of the target Cellar
    #[prost(string, tag = "1")]
    pub cellar_id: ::prost::alloc::string::String,
    /// The data from which the desired contract function will be encoded
    #[prost(oneof = "submit_request::CallData", tags = "2, 3, 4, 5, 6")]
    pub call_data: ::core::option::Option<submit_request::CallData>,
}
/// Nested message and enum types in `SubmitRequest`.
pub mod submit_request {
    /// The data from which the desired contract function will be encoded
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Oneof)]
    pub enum CallData {
        #[prost(message, tag = "2")]
        AaveV2Stablecoin(super::AaveV2Stablecoin),
        #[prost(message, tag = "3")]
        CellarV1(super::CellarV1),
        #[prost(message, tag = "4")]
        CellarV2(super::CellarV2),
        #[prost(message, tag = "5")]
        CellarV2dot2(super::CellarV22),
        #[prost(message, tag = "6")]
        CellarV2dot5(super::CellarV25),
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct SubmitResponse {}
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct VersionRequest {}
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct VersionResponse {
    #[prost(string, tag = "1")]
    pub version: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod contract_call_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = ""]
    #[doc = " Service for handling Cellar contract calls"]
    pub struct ContractCallClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ContractCallClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> ContractCallClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        #[doc = " Handles simple contract call submission"]
        pub async fn submit(
            &mut self,
            request: impl tonic::IntoRequest<super::SubmitRequest>,
        ) -> Result<tonic::Response<super::SubmitResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/steward.v3.ContractCall/Submit");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for ContractCallClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for ContractCallClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "ContractCallClient {{ ... }}")
        }
    }
}
#[doc = r" Generated client implementations."]
pub mod status_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub struct StatusClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl StatusClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> StatusClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        pub async fn version(
            &mut self,
            request: impl tonic::IntoRequest<super::VersionRequest>,
        ) -> Result<tonic::Response<super::VersionResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/steward.v3.Status/Version");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for StatusClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for StatusClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "StatusClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod contract_call_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with ContractCallServer."]
    #[async_trait]
    pub trait ContractCall: Send + Sync + 'static {
        #[doc = " Handles simple contract call submission"]
        async fn submit(
            &self,
            request: tonic::Request<super::SubmitRequest>,
        ) -> Result<tonic::Response<super::SubmitResponse>, tonic::Status>;
    }
    #[doc = ""]
    #[doc = " Service for handling Cellar contract calls"]
    #[derive(Debug)]
    pub struct ContractCallServer<T: ContractCall> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: ContractCall> ContractCallServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for ContractCallServer<T>
    where
        T: ContractCall,
        B: HttpBody + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/steward.v3.ContractCall/Submit" => {
                    #[allow(non_camel_case_types)]
                    struct SubmitSvc<T: ContractCall>(pub Arc<T>);
                    impl<T: ContractCall> tonic::server::UnaryService<super::SubmitRequest> for SubmitSvc<T> {
                        type Response = super::SubmitResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SubmitRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).submit(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = SubmitSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: ContractCall> Clone for ContractCallServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: ContractCall> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ContractCall> tonic::transport::NamedService for ContractCallServer<T> {
        const NAME: &'static str = "steward.v3.ContractCall";
    }
}
#[doc = r" Generated server implementations."]
pub mod status_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with StatusServer."]
    #[async_trait]
    pub trait Status: Send + Sync + 'static {
        async fn version(
            &self,
            request: tonic::Request<super::VersionRequest>,
        ) -> Result<tonic::Response<super::VersionResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct StatusServer<T: Status> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: Status> StatusServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for StatusServer<T>
    where
        T: Status,
        B: HttpBody + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/steward.v3.Status/Version" => {
                    #[allow(non_camel_case_types)]
                    struct VersionSvc<T: Status>(pub Arc<T>);
                    impl<T: Status> tonic::server::UnaryService<super::VersionRequest> for VersionSvc<T> {
                        type Response = super::VersionResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::VersionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).version(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = VersionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: Status> Clone for StatusServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: Status> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Status> tonic::transport::NamedService for StatusServer<T> {
        const NAME: &'static str = "steward.v3.Status";
    }
}
