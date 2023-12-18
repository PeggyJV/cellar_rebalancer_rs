pub use balancerpooladaptorv1_mod::*;
#[allow(clippy::too_many_arguments)]
mod balancerpooladaptorv1_mod {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    use ethers::contract::{
        builders::{ContractCall, Event},
        Contract, Lazy,
    };
    use ethers::core::{
        abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
        types::*,
    };
    use ethers::providers::Middleware;
    #[doc = "BalancerPoolAdaptorV1 was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static BALANCERPOOLADAPTORV1_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
        });
    #[derive(Clone)]
    pub struct BalancerPoolAdaptorV1<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for BalancerPoolAdaptorV1<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for BalancerPoolAdaptorV1<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(BalancerPoolAdaptorV1))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> BalancerPoolAdaptorV1<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            let contract = ethers::contract::Contract::new(
                address.into(),
                BALANCERPOOLADAPTORV1_ABI.clone(),
                client,
            );
            Self(contract)
        }
        #[doc = "Calls the contract's `EXACT_TOKENS_IN_FOR_BPT_OUT` (0x0e222e52) function"]
        pub fn exact_tokens_in_for_bpt_out(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([14, 34, 46, 82], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `assetOf` (0xe170a9bf) function"]
        pub fn asset_of(
            &self,
            adaptor_data: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([225, 112, 169, 191], adaptor_data)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `assetsUsed` (0xaeffddde) function"]
        pub fn assets_used(
            &self,
            adaptor_data: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<ethers::core::types::Address>,
        > {
            self.0
                .method_hash([174, 255, 221, 222], adaptor_data)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `balanceOf` (0x78415365) function"]
        pub fn balance_of(
            &self,
            adaptor_data: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([120, 65, 83, 101], adaptor_data)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `balancerSlippage` (0xf7e69b16) function"]
        pub fn balancer_slippage(&self) -> ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([247, 230, 155, 22], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `claimRewards` (0xef5cfb8c) function"]
        pub fn claim_rewards(
            &self,
            gauge: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([239, 92, 251, 140], gauge)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `deposit` (0x69445c31) function"]
        pub fn deposit(
            &self,
            p0: ethers::core::types::U256,
            p1: ethers::core::types::Bytes,
            p2: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([105, 68, 92, 49], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `exitPool` (0x97194b13) function"]
        pub fn exit_pool(
            &self,
            target_bpt: ethers::core::types::Address,
            swaps_after_exit: ::std::vec::Vec<SingleSwap>,
            swap_data: SwapData,
            request: ExitPoolRequest,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [151, 25, 75, 19],
                    (target_bpt, swaps_after_exit, swap_data, request),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getExpectedTokens` (0x7906afbf) function"]
        pub fn get_expected_tokens(
            &self,
            target_bpt: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<ethers::core::types::Address>,
        > {
            self.0
                .method_hash([121, 6, 175, 191], target_bpt)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `identifier` (0x7998a1c4) function"]
        pub fn identifier(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([121, 152, 161, 196], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isDebt` (0x89353a09) function"]
        pub fn is_debt(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([137, 53, 58, 9], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `joinPool` (0x5c14acdb) function"]
        pub fn join_pool(
            &self,
            target_bpt: ethers::core::types::Address,
            swaps_before_join: ::std::vec::Vec<SingleSwap>,
            swap_data: SwapData,
            minimum_bpt: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [92, 20, 172, 219],
                    (target_bpt, swaps_before_join, swap_data, minimum_bpt),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `makeFlashLoan` (0xc9a69562) function"]
        pub fn make_flash_loan(
            &self,
            tokens: ::std::vec::Vec<ethers::core::types::Address>,
            amounts: ::std::vec::Vec<ethers::core::types::U256>,
            data: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([201, 166, 149, 98], (tokens, amounts, data))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `minter` (0x07546172) function"]
        pub fn minter(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([7, 84, 97, 114], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `revokeApproval` (0xd3bfe76a) function"]
        pub fn revoke_approval(
            &self,
            asset: ethers::core::types::Address,
            spender: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([211, 191, 231, 106], (asset, spender))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `slippage` (0x3e032a3b) function"]
        pub fn slippage(&self) -> ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([62, 3, 42, 59], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `stakeBPT` (0x8f210a6a) function"]
        pub fn stake_bpt(
            &self,
            bpt: ethers::core::types::Address,
            liquidity_gauge: ethers::core::types::Address,
            amount_in: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([143, 33, 10, 106], (bpt, liquidity_gauge, amount_in))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `unstakeBPT` (0xd2e85806) function"]
        pub fn unstake_bpt(
            &self,
            bpt: ethers::core::types::Address,
            liquidity_gauge: ethers::core::types::Address,
            amount_out: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([210, 232, 88, 6], (bpt, liquidity_gauge, amount_out))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `vault` (0xfbfa77cf) function"]
        pub fn vault(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([251, 250, 119, 207], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `withdraw` (0xc9111bd7) function"]
        pub fn withdraw(
            &self,
            amount_bpt_to_send: ethers::core::types::U256,
            recipient: ethers::core::types::Address,
            adaptor_data: ethers::core::types::Bytes,
            p3: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [201, 17, 27, 215],
                    (amount_bpt_to_send, recipient, adaptor_data, p3),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `withdrawableFrom` (0xfa50e5d2) function"]
        pub fn withdrawable_from(
            &self,
            adaptor_data: ethers::core::types::Bytes,
            p1: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([250, 80, 229, 210], (adaptor_data, p1))
                .expect("method not found (this should never happen)")
        }
    }
    #[doc = "Container type for all input parameters for the `EXACT_TOKENS_IN_FOR_BPT_OUT`function with signature `EXACT_TOKENS_IN_FOR_BPT_OUT()` and selector `[14, 34, 46, 82]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(
        name = "EXACT_TOKENS_IN_FOR_BPT_OUT",
        abi = "EXACT_TOKENS_IN_FOR_BPT_OUT()"
    )]
    pub struct ExactTokensInForBptOutCall;
    #[doc = "Container type for all input parameters for the `assetOf`function with signature `assetOf(bytes)` and selector `[225, 112, 169, 191]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "assetOf", abi = "assetOf(bytes)")]
    pub struct AssetOfCall {
        pub adaptor_data: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `assetsUsed`function with signature `assetsUsed(bytes)` and selector `[174, 255, 221, 222]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "assetsUsed", abi = "assetsUsed(bytes)")]
    pub struct AssetsUsedCall {
        pub adaptor_data: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `balanceOf`function with signature `balanceOf(bytes)` and selector `[120, 65, 83, 101]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "balanceOf", abi = "balanceOf(bytes)")]
    pub struct BalanceOfCall {
        pub adaptor_data: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `balancerSlippage`function with signature `balancerSlippage()` and selector `[247, 230, 155, 22]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "balancerSlippage", abi = "balancerSlippage()")]
    pub struct BalancerSlippageCall;
    #[doc = "Container type for all input parameters for the `claimRewards`function with signature `claimRewards(address)` and selector `[239, 92, 251, 140]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "claimRewards", abi = "claimRewards(address)")]
    pub struct ClaimRewardsCall {
        pub gauge: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `deposit`function with signature `deposit(uint256,bytes,bytes)` and selector `[105, 68, 92, 49]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "deposit", abi = "deposit(uint256,bytes,bytes)")]
    pub struct DepositCall(
        pub ethers::core::types::U256,
        pub ethers::core::types::Bytes,
        pub ethers::core::types::Bytes,
    );
    #[doc = "Container type for all input parameters for the `exitPool`function with signature `exitPool(address,(bytes32,uint8,address,address,uint256,bytes)[],(uint256[],uint256[]),(address[],uint256[],bytes,bool))` and selector `[151, 25, 75, 19]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(
        name = "exitPool",
        abi = "exitPool(address,(bytes32,uint8,address,address,uint256,bytes)[],(uint256[],uint256[]),(address[],uint256[],bytes,bool))"
    )]
    pub struct ExitPoolCall {
        pub target_bpt: ethers::core::types::Address,
        pub swaps_after_exit: ::std::vec::Vec<SingleSwap>,
        pub swap_data: SwapData,
        pub request: ExitPoolRequest,
    }
    #[doc = "Container type for all input parameters for the `getExpectedTokens`function with signature `getExpectedTokens(address)` and selector `[121, 6, 175, 191]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "getExpectedTokens", abi = "getExpectedTokens(address)")]
    pub struct GetExpectedTokensCall {
        pub target_bpt: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `identifier`function with signature `identifier()` and selector `[121, 152, 161, 196]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "identifier", abi = "identifier()")]
    pub struct IdentifierCall;
    #[doc = "Container type for all input parameters for the `isDebt`function with signature `isDebt()` and selector `[137, 53, 58, 9]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "isDebt", abi = "isDebt()")]
    pub struct IsDebtCall;
    #[doc = "Container type for all input parameters for the `joinPool`function with signature `joinPool(address,(bytes32,uint8,address,address,uint256,bytes)[],(uint256[],uint256[]),uint256)` and selector `[92, 20, 172, 219]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(
        name = "joinPool",
        abi = "joinPool(address,(bytes32,uint8,address,address,uint256,bytes)[],(uint256[],uint256[]),uint256)"
    )]
    pub struct JoinPoolCall {
        pub target_bpt: ethers::core::types::Address,
        pub swaps_before_join: ::std::vec::Vec<SingleSwap>,
        pub swap_data: SwapData,
        pub minimum_bpt: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `makeFlashLoan`function with signature `makeFlashLoan(address[],uint256[],bytes)` and selector `[201, 166, 149, 98]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(
        name = "makeFlashLoan",
        abi = "makeFlashLoan(address[],uint256[],bytes)"
    )]
    pub struct MakeFlashLoanCall {
        pub tokens: ::std::vec::Vec<ethers::core::types::Address>,
        pub amounts: ::std::vec::Vec<ethers::core::types::U256>,
        pub data: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `minter`function with signature `minter()` and selector `[7, 84, 97, 114]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "minter", abi = "minter()")]
    pub struct MinterCall;
    #[doc = "Container type for all input parameters for the `revokeApproval`function with signature `revokeApproval(address,address)` and selector `[211, 191, 231, 106]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "revokeApproval", abi = "revokeApproval(address,address)")]
    pub struct RevokeApprovalCall {
        pub asset: ethers::core::types::Address,
        pub spender: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `slippage`function with signature `slippage()` and selector `[62, 3, 42, 59]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "slippage", abi = "slippage()")]
    pub struct SlippageCall;
    #[doc = "Container type for all input parameters for the `stakeBPT`function with signature `stakeBPT(address,address,uint256)` and selector `[143, 33, 10, 106]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "stakeBPT", abi = "stakeBPT(address,address,uint256)")]
    pub struct StakeBPTCall {
        pub bpt: ethers::core::types::Address,
        pub liquidity_gauge: ethers::core::types::Address,
        pub amount_in: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `unstakeBPT`function with signature `unstakeBPT(address,address,uint256)` and selector `[210, 232, 88, 6]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "unstakeBPT", abi = "unstakeBPT(address,address,uint256)")]
    pub struct UnstakeBPTCall {
        pub bpt: ethers::core::types::Address,
        pub liquidity_gauge: ethers::core::types::Address,
        pub amount_out: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `vault`function with signature `vault()` and selector `[251, 250, 119, 207]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "vault", abi = "vault()")]
    pub struct VaultCall;
    #[doc = "Container type for all input parameters for the `withdraw`function with signature `withdraw(uint256,address,bytes,bytes)` and selector `[201, 17, 27, 215]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "withdraw", abi = "withdraw(uint256,address,bytes,bytes)")]
    pub struct WithdrawCall {
        pub amount_bpt_to_send: ethers::core::types::U256,
        pub recipient: ethers::core::types::Address,
        pub adaptor_data: ethers::core::types::Bytes,
        pub p3: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `withdrawableFrom`function with signature `withdrawableFrom(bytes,bytes)` and selector `[250, 80, 229, 210]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "withdrawableFrom", abi = "withdrawableFrom(bytes,bytes)")]
    pub struct WithdrawableFromCall {
        pub adaptor_data: ethers::core::types::Bytes,
        pub p1: ethers::core::types::Bytes,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum BalancerPoolAdaptorV1Calls {
        ExactTokensInForBptOut(ExactTokensInForBptOutCall),
        AssetOf(AssetOfCall),
        AssetsUsed(AssetsUsedCall),
        BalanceOf(BalanceOfCall),
        BalancerSlippage(BalancerSlippageCall),
        ClaimRewards(ClaimRewardsCall),
        Deposit(DepositCall),
        ExitPool(ExitPoolCall),
        GetExpectedTokens(GetExpectedTokensCall),
        Identifier(IdentifierCall),
        IsDebt(IsDebtCall),
        JoinPool(JoinPoolCall),
        MakeFlashLoan(MakeFlashLoanCall),
        Minter(MinterCall),
        RevokeApproval(RevokeApprovalCall),
        Slippage(SlippageCall),
        StakeBPT(StakeBPTCall),
        UnstakeBPT(UnstakeBPTCall),
        Vault(VaultCall),
        Withdraw(WithdrawCall),
        WithdrawableFrom(WithdrawableFromCall),
    }
    impl ethers::core::abi::AbiDecode for BalancerPoolAdaptorV1Calls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <ExactTokensInForBptOutCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BalancerPoolAdaptorV1Calls::ExactTokensInForBptOut(decoded));
            }
            if let Ok(decoded) =
                <AssetOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BalancerPoolAdaptorV1Calls::AssetOf(decoded));
            }
            if let Ok(decoded) =
                <AssetsUsedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BalancerPoolAdaptorV1Calls::AssetsUsed(decoded));
            }
            if let Ok(decoded) =
                <BalanceOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BalancerPoolAdaptorV1Calls::BalanceOf(decoded));
            }
            if let Ok(decoded) =
                <BalancerSlippageCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BalancerPoolAdaptorV1Calls::BalancerSlippage(decoded));
            }
            if let Ok(decoded) =
                <ClaimRewardsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BalancerPoolAdaptorV1Calls::ClaimRewards(decoded));
            }
            if let Ok(decoded) =
                <DepositCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BalancerPoolAdaptorV1Calls::Deposit(decoded));
            }
            if let Ok(decoded) =
                <ExitPoolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BalancerPoolAdaptorV1Calls::ExitPool(decoded));
            }
            if let Ok(decoded) =
                <GetExpectedTokensCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BalancerPoolAdaptorV1Calls::GetExpectedTokens(decoded));
            }
            if let Ok(decoded) =
                <IdentifierCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BalancerPoolAdaptorV1Calls::Identifier(decoded));
            }
            if let Ok(decoded) = <IsDebtCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BalancerPoolAdaptorV1Calls::IsDebt(decoded));
            }
            if let Ok(decoded) =
                <JoinPoolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BalancerPoolAdaptorV1Calls::JoinPool(decoded));
            }
            if let Ok(decoded) =
                <MakeFlashLoanCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BalancerPoolAdaptorV1Calls::MakeFlashLoan(decoded));
            }
            if let Ok(decoded) = <MinterCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BalancerPoolAdaptorV1Calls::Minter(decoded));
            }
            if let Ok(decoded) =
                <RevokeApprovalCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BalancerPoolAdaptorV1Calls::RevokeApproval(decoded));
            }
            if let Ok(decoded) =
                <SlippageCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BalancerPoolAdaptorV1Calls::Slippage(decoded));
            }
            if let Ok(decoded) =
                <StakeBPTCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BalancerPoolAdaptorV1Calls::StakeBPT(decoded));
            }
            if let Ok(decoded) =
                <UnstakeBPTCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BalancerPoolAdaptorV1Calls::UnstakeBPT(decoded));
            }
            if let Ok(decoded) = <VaultCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BalancerPoolAdaptorV1Calls::Vault(decoded));
            }
            if let Ok(decoded) =
                <WithdrawCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BalancerPoolAdaptorV1Calls::Withdraw(decoded));
            }
            if let Ok(decoded) =
                <WithdrawableFromCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BalancerPoolAdaptorV1Calls::WithdrawableFrom(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for BalancerPoolAdaptorV1Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                BalancerPoolAdaptorV1Calls::ExactTokensInForBptOut(element) => element.encode(),
                BalancerPoolAdaptorV1Calls::AssetOf(element) => element.encode(),
                BalancerPoolAdaptorV1Calls::AssetsUsed(element) => element.encode(),
                BalancerPoolAdaptorV1Calls::BalanceOf(element) => element.encode(),
                BalancerPoolAdaptorV1Calls::BalancerSlippage(element) => element.encode(),
                BalancerPoolAdaptorV1Calls::ClaimRewards(element) => element.encode(),
                BalancerPoolAdaptorV1Calls::Deposit(element) => element.encode(),
                BalancerPoolAdaptorV1Calls::ExitPool(element) => element.encode(),
                BalancerPoolAdaptorV1Calls::GetExpectedTokens(element) => element.encode(),
                BalancerPoolAdaptorV1Calls::Identifier(element) => element.encode(),
                BalancerPoolAdaptorV1Calls::IsDebt(element) => element.encode(),
                BalancerPoolAdaptorV1Calls::JoinPool(element) => element.encode(),
                BalancerPoolAdaptorV1Calls::MakeFlashLoan(element) => element.encode(),
                BalancerPoolAdaptorV1Calls::Minter(element) => element.encode(),
                BalancerPoolAdaptorV1Calls::RevokeApproval(element) => element.encode(),
                BalancerPoolAdaptorV1Calls::Slippage(element) => element.encode(),
                BalancerPoolAdaptorV1Calls::StakeBPT(element) => element.encode(),
                BalancerPoolAdaptorV1Calls::UnstakeBPT(element) => element.encode(),
                BalancerPoolAdaptorV1Calls::Vault(element) => element.encode(),
                BalancerPoolAdaptorV1Calls::Withdraw(element) => element.encode(),
                BalancerPoolAdaptorV1Calls::WithdrawableFrom(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for BalancerPoolAdaptorV1Calls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                BalancerPoolAdaptorV1Calls::ExactTokensInForBptOut(element) => element.fmt(f),
                BalancerPoolAdaptorV1Calls::AssetOf(element) => element.fmt(f),
                BalancerPoolAdaptorV1Calls::AssetsUsed(element) => element.fmt(f),
                BalancerPoolAdaptorV1Calls::BalanceOf(element) => element.fmt(f),
                BalancerPoolAdaptorV1Calls::BalancerSlippage(element) => element.fmt(f),
                BalancerPoolAdaptorV1Calls::ClaimRewards(element) => element.fmt(f),
                BalancerPoolAdaptorV1Calls::Deposit(element) => element.fmt(f),
                BalancerPoolAdaptorV1Calls::ExitPool(element) => element.fmt(f),
                BalancerPoolAdaptorV1Calls::GetExpectedTokens(element) => element.fmt(f),
                BalancerPoolAdaptorV1Calls::Identifier(element) => element.fmt(f),
                BalancerPoolAdaptorV1Calls::IsDebt(element) => element.fmt(f),
                BalancerPoolAdaptorV1Calls::JoinPool(element) => element.fmt(f),
                BalancerPoolAdaptorV1Calls::MakeFlashLoan(element) => element.fmt(f),
                BalancerPoolAdaptorV1Calls::Minter(element) => element.fmt(f),
                BalancerPoolAdaptorV1Calls::RevokeApproval(element) => element.fmt(f),
                BalancerPoolAdaptorV1Calls::Slippage(element) => element.fmt(f),
                BalancerPoolAdaptorV1Calls::StakeBPT(element) => element.fmt(f),
                BalancerPoolAdaptorV1Calls::UnstakeBPT(element) => element.fmt(f),
                BalancerPoolAdaptorV1Calls::Vault(element) => element.fmt(f),
                BalancerPoolAdaptorV1Calls::Withdraw(element) => element.fmt(f),
                BalancerPoolAdaptorV1Calls::WithdrawableFrom(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<ExactTokensInForBptOutCall> for BalancerPoolAdaptorV1Calls {
        fn from(var: ExactTokensInForBptOutCall) -> Self {
            BalancerPoolAdaptorV1Calls::ExactTokensInForBptOut(var)
        }
    }
    impl ::std::convert::From<AssetOfCall> for BalancerPoolAdaptorV1Calls {
        fn from(var: AssetOfCall) -> Self {
            BalancerPoolAdaptorV1Calls::AssetOf(var)
        }
    }
    impl ::std::convert::From<AssetsUsedCall> for BalancerPoolAdaptorV1Calls {
        fn from(var: AssetsUsedCall) -> Self {
            BalancerPoolAdaptorV1Calls::AssetsUsed(var)
        }
    }
    impl ::std::convert::From<BalanceOfCall> for BalancerPoolAdaptorV1Calls {
        fn from(var: BalanceOfCall) -> Self {
            BalancerPoolAdaptorV1Calls::BalanceOf(var)
        }
    }
    impl ::std::convert::From<BalancerSlippageCall> for BalancerPoolAdaptorV1Calls {
        fn from(var: BalancerSlippageCall) -> Self {
            BalancerPoolAdaptorV1Calls::BalancerSlippage(var)
        }
    }
    impl ::std::convert::From<ClaimRewardsCall> for BalancerPoolAdaptorV1Calls {
        fn from(var: ClaimRewardsCall) -> Self {
            BalancerPoolAdaptorV1Calls::ClaimRewards(var)
        }
    }
    impl ::std::convert::From<DepositCall> for BalancerPoolAdaptorV1Calls {
        fn from(var: DepositCall) -> Self {
            BalancerPoolAdaptorV1Calls::Deposit(var)
        }
    }
    impl ::std::convert::From<ExitPoolCall> for BalancerPoolAdaptorV1Calls {
        fn from(var: ExitPoolCall) -> Self {
            BalancerPoolAdaptorV1Calls::ExitPool(var)
        }
    }
    impl ::std::convert::From<GetExpectedTokensCall> for BalancerPoolAdaptorV1Calls {
        fn from(var: GetExpectedTokensCall) -> Self {
            BalancerPoolAdaptorV1Calls::GetExpectedTokens(var)
        }
    }
    impl ::std::convert::From<IdentifierCall> for BalancerPoolAdaptorV1Calls {
        fn from(var: IdentifierCall) -> Self {
            BalancerPoolAdaptorV1Calls::Identifier(var)
        }
    }
    impl ::std::convert::From<IsDebtCall> for BalancerPoolAdaptorV1Calls {
        fn from(var: IsDebtCall) -> Self {
            BalancerPoolAdaptorV1Calls::IsDebt(var)
        }
    }
    impl ::std::convert::From<JoinPoolCall> for BalancerPoolAdaptorV1Calls {
        fn from(var: JoinPoolCall) -> Self {
            BalancerPoolAdaptorV1Calls::JoinPool(var)
        }
    }
    impl ::std::convert::From<MakeFlashLoanCall> for BalancerPoolAdaptorV1Calls {
        fn from(var: MakeFlashLoanCall) -> Self {
            BalancerPoolAdaptorV1Calls::MakeFlashLoan(var)
        }
    }
    impl ::std::convert::From<MinterCall> for BalancerPoolAdaptorV1Calls {
        fn from(var: MinterCall) -> Self {
            BalancerPoolAdaptorV1Calls::Minter(var)
        }
    }
    impl ::std::convert::From<RevokeApprovalCall> for BalancerPoolAdaptorV1Calls {
        fn from(var: RevokeApprovalCall) -> Self {
            BalancerPoolAdaptorV1Calls::RevokeApproval(var)
        }
    }
    impl ::std::convert::From<SlippageCall> for BalancerPoolAdaptorV1Calls {
        fn from(var: SlippageCall) -> Self {
            BalancerPoolAdaptorV1Calls::Slippage(var)
        }
    }
    impl ::std::convert::From<StakeBPTCall> for BalancerPoolAdaptorV1Calls {
        fn from(var: StakeBPTCall) -> Self {
            BalancerPoolAdaptorV1Calls::StakeBPT(var)
        }
    }
    impl ::std::convert::From<UnstakeBPTCall> for BalancerPoolAdaptorV1Calls {
        fn from(var: UnstakeBPTCall) -> Self {
            BalancerPoolAdaptorV1Calls::UnstakeBPT(var)
        }
    }
    impl ::std::convert::From<VaultCall> for BalancerPoolAdaptorV1Calls {
        fn from(var: VaultCall) -> Self {
            BalancerPoolAdaptorV1Calls::Vault(var)
        }
    }
    impl ::std::convert::From<WithdrawCall> for BalancerPoolAdaptorV1Calls {
        fn from(var: WithdrawCall) -> Self {
            BalancerPoolAdaptorV1Calls::Withdraw(var)
        }
    }
    impl ::std::convert::From<WithdrawableFromCall> for BalancerPoolAdaptorV1Calls {
        fn from(var: WithdrawableFromCall) -> Self {
            BalancerPoolAdaptorV1Calls::WithdrawableFrom(var)
        }
    }
    #[doc = "`SwapData(uint256[],uint256[])`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    pub struct SwapData {
        pub min_amounts_for_swaps: Vec<ethers::core::types::U256>,
        pub swap_deadlines: Vec<ethers::core::types::U256>,
    }
    #[doc = "`ExitPoolRequest(address[],uint256[],bytes,bool)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    pub struct ExitPoolRequest {
        pub assets: Vec<ethers::core::types::Address>,
        pub min_amounts_out: Vec<ethers::core::types::U256>,
        pub user_data: ethers::core::types::Bytes,
        pub to_internal_balance: bool,
    }
    #[doc = "`SingleSwap(bytes32,uint8,address,address,uint256,bytes)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    pub struct SingleSwap {
        pub pool_id: [u8; 32],
        pub kind: u8,
        pub asset_in: ethers::core::types::Address,
        pub asset_out: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        pub user_data: ethers::core::types::Bytes,
    }
}