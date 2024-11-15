pub use managerwithmerkleverification_mod::*;
#[allow(clippy::too_many_arguments)]
mod managerwithmerkleverification_mod {
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
    #[doc = "ManagerWithMerkleVerification was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static MANAGERWITHMERKLEVERIFICATION_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[\n    {\n        \"type\": \"constructor\",\n        \"inputs\": [\n            {\n                \"name\": \"_owner\",\n                \"type\": \"address\",\n                \"internalType\": \"address\"\n            },\n            {\n                \"name\": \"_vault\",\n                \"type\": \"address\",\n                \"internalType\": \"address\"\n            },\n            {\n                \"name\": \"_balancerVault\",\n                \"type\": \"address\",\n                \"internalType\": \"address\"\n            }\n        ],\n        \"stateMutability\": \"nonpayable\"\n    },\n    {\n        \"type\": \"function\",\n        \"name\": \"authority\",\n        \"inputs\": [],\n        \"outputs\": [\n            {\n                \"name\": \"\",\n                \"type\": \"address\",\n                \"internalType\": \"contract Authority\"\n            }\n        ],\n        \"stateMutability\": \"view\"\n    },\n    {\n        \"type\": \"function\",\n        \"name\": \"balancerVault\",\n        \"inputs\": [],\n        \"outputs\": [\n            {\n                \"name\": \"\",\n                \"type\": \"address\",\n                \"internalType\": \"contract BalancerVault\"\n            }\n        ],\n        \"stateMutability\": \"view\"\n    },\n    {\n        \"type\": \"function\",\n        \"name\": \"flashLoan\",\n        \"inputs\": [\n            {\n                \"name\": \"recipient\",\n                \"type\": \"address\",\n                \"internalType\": \"address\"\n            },\n            {\n                \"name\": \"tokens\",\n                \"type\": \"address[]\",\n                \"internalType\": \"address[]\"\n            },\n            {\n                \"name\": \"amounts\",\n                \"type\": \"uint256[]\",\n                \"internalType\": \"uint256[]\"\n            },\n            {\n                \"name\": \"userData\",\n                \"type\": \"bytes\",\n                \"internalType\": \"bytes\"\n            }\n        ],\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\"\n    },\n    {\n        \"type\": \"function\",\n        \"name\": \"isPaused\",\n        \"inputs\": [],\n        \"outputs\": [\n            {\n                \"name\": \"\",\n                \"type\": \"bool\",\n                \"internalType\": \"bool\"\n            }\n        ],\n        \"stateMutability\": \"view\"\n    },\n    {\n        \"type\": \"function\",\n        \"name\": \"manageRoot\",\n        \"inputs\": [\n            {\n                \"name\": \"\",\n                \"type\": \"address\",\n                \"internalType\": \"address\"\n            }\n        ],\n        \"outputs\": [\n            {\n                \"name\": \"\",\n                \"type\": \"bytes32\",\n                \"internalType\": \"bytes32\"\n            }\n        ],\n        \"stateMutability\": \"view\"\n    },\n    {\n        \"type\": \"function\",\n        \"name\": \"manageVaultWithMerkleVerification\",\n        \"inputs\": [\n            {\n                \"name\": \"manageProofs\",\n                \"type\": \"bytes32[][]\",\n                \"internalType\": \"bytes32[][]\"\n            },\n            {\n                \"name\": \"decodersAndSanitizers\",\n                \"type\": \"address[]\",\n                \"internalType\": \"address[]\"\n            },\n            {\n                \"name\": \"targets\",\n                \"type\": \"address[]\",\n                \"internalType\": \"address[]\"\n            },\n            {\n                \"name\": \"targetData\",\n                \"type\": \"bytes[]\",\n                \"internalType\": \"bytes[]\"\n            },\n            {\n                \"name\": \"values\",\n                \"type\": \"uint256[]\",\n                \"internalType\": \"uint256[]\"\n            }\n        ],\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\"\n    },\n    {\n        \"type\": \"function\",\n        \"name\": \"owner\",\n        \"inputs\": [],\n        \"outputs\": [\n            {\n                \"name\": \"\",\n                \"type\": \"address\",\n                \"internalType\": \"address\"\n            }\n        ],\n        \"stateMutability\": \"view\"\n    },\n    {\n        \"type\": \"function\",\n        \"name\": \"pause\",\n        \"inputs\": [],\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\"\n    },\n    {\n        \"type\": \"function\",\n        \"name\": \"receiveFlashLoan\",\n        \"inputs\": [\n            {\n                \"name\": \"tokens\",\n                \"type\": \"address[]\",\n                \"internalType\": \"address[]\"\n            },\n            {\n                \"name\": \"amounts\",\n                \"type\": \"uint256[]\",\n                \"internalType\": \"uint256[]\"\n            },\n            {\n                \"name\": \"feeAmounts\",\n                \"type\": \"uint256[]\",\n                \"internalType\": \"uint256[]\"\n            },\n            {\n                \"name\": \"userData\",\n                \"type\": \"bytes\",\n                \"internalType\": \"bytes\"\n            }\n        ],\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\"\n    },\n    {\n        \"type\": \"function\",\n        \"name\": \"setAuthority\",\n        \"inputs\": [\n            {\n                \"name\": \"newAuthority\",\n                \"type\": \"address\",\n                \"internalType\": \"contract Authority\"\n            }\n        ],\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\"\n    },\n    {\n        \"type\": \"function\",\n        \"name\": \"setManageRoot\",\n        \"inputs\": [\n            {\n                \"name\": \"strategist\",\n                \"type\": \"address\",\n                \"internalType\": \"address\"\n            },\n            {\n                \"name\": \"_manageRoot\",\n                \"type\": \"bytes32\",\n                \"internalType\": \"bytes32\"\n            }\n        ],\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\"\n    },\n    {\n        \"type\": \"function\",\n        \"name\": \"transferOwnership\",\n        \"inputs\": [\n            {\n                \"name\": \"newOwner\",\n                \"type\": \"address\",\n                \"internalType\": \"address\"\n            }\n        ],\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\"\n    },\n    {\n        \"type\": \"function\",\n        \"name\": \"unpause\",\n        \"inputs\": [],\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\"\n    },\n    {\n        \"type\": \"function\",\n        \"name\": \"vault\",\n        \"inputs\": [],\n        \"outputs\": [\n            {\n                \"name\": \"\",\n                \"type\": \"address\",\n                \"internalType\": \"contract BoringVault\"\n            }\n        ],\n        \"stateMutability\": \"view\"\n    },\n    {\n        \"type\": \"event\",\n        \"name\": \"AuthorityUpdated\",\n        \"inputs\": [\n            {\n                \"name\": \"user\",\n                \"type\": \"address\",\n                \"indexed\": true,\n                \"internalType\": \"address\"\n            },\n            {\n                \"name\": \"newAuthority\",\n                \"type\": \"address\",\n                \"indexed\": true,\n                \"internalType\": \"contract Authority\"\n            }\n        ],\n        \"anonymous\": false\n    },\n    {\n        \"type\": \"event\",\n        \"name\": \"BoringVaultManaged\",\n        \"inputs\": [\n            {\n                \"name\": \"callsMade\",\n                \"type\": \"uint256\",\n                \"indexed\": false,\n                \"internalType\": \"uint256\"\n            }\n        ],\n        \"anonymous\": false\n    },\n    {\n        \"type\": \"event\",\n        \"name\": \"ManageRootUpdated\",\n        \"inputs\": [\n            {\n                \"name\": \"strategist\",\n                \"type\": \"address\",\n                \"indexed\": true,\n                \"internalType\": \"address\"\n            },\n            {\n                \"name\": \"oldRoot\",\n                \"type\": \"bytes32\",\n                \"indexed\": false,\n                \"internalType\": \"bytes32\"\n            },\n            {\n                \"name\": \"newRoot\",\n                \"type\": \"bytes32\",\n                \"indexed\": false,\n                \"internalType\": \"bytes32\"\n            }\n        ],\n        \"anonymous\": false\n    },\n    {\n        \"type\": \"event\",\n        \"name\": \"OwnershipTransferred\",\n        \"inputs\": [\n            {\n                \"name\": \"user\",\n                \"type\": \"address\",\n                \"indexed\": true,\n                \"internalType\": \"address\"\n            },\n            {\n                \"name\": \"newOwner\",\n                \"type\": \"address\",\n                \"indexed\": true,\n                \"internalType\": \"address\"\n            }\n        ],\n        \"anonymous\": false\n    },\n    {\n        \"type\": \"event\",\n        \"name\": \"Paused\",\n        \"inputs\": [],\n        \"anonymous\": false\n    },\n    {\n        \"type\": \"event\",\n        \"name\": \"Unpaused\",\n        \"inputs\": [],\n        \"anonymous\": false\n    },\n    {\n        \"type\": \"error\",\n        \"name\": \"AddressEmptyCode\",\n        \"inputs\": [\n            {\n                \"name\": \"target\",\n                \"type\": \"address\",\n                \"internalType\": \"address\"\n            }\n        ]\n    },\n    {\n        \"type\": \"error\",\n        \"name\": \"FailedInnerCall\",\n        \"inputs\": []\n    },\n    {\n        \"type\": \"error\",\n        \"name\": \"ManagerWithMerkleVerification__BadFlashLoanIntentHash\",\n        \"inputs\": []\n    },\n    {\n        \"type\": \"error\",\n        \"name\": \"ManagerWithMerkleVerification__FailedToVerifyManageProof\",\n        \"inputs\": [\n            {\n                \"name\": \"target\",\n                \"type\": \"address\",\n                \"internalType\": \"address\"\n            },\n            {\n                \"name\": \"targetData\",\n                \"type\": \"bytes\",\n                \"internalType\": \"bytes\"\n            },\n            {\n                \"name\": \"value\",\n                \"type\": \"uint256\",\n                \"internalType\": \"uint256\"\n            }\n        ]\n    },\n    {\n        \"type\": \"error\",\n        \"name\": \"ManagerWithMerkleVerification__FlashLoanNotExecuted\",\n        \"inputs\": []\n    },\n    {\n        \"type\": \"error\",\n        \"name\": \"ManagerWithMerkleVerification__FlashLoanNotInProgress\",\n        \"inputs\": []\n    },\n    {\n        \"type\": \"error\",\n        \"name\": \"ManagerWithMerkleVerification__InvalidDecodersAndSanitizersLength\",\n        \"inputs\": []\n    },\n    {\n        \"type\": \"error\",\n        \"name\": \"ManagerWithMerkleVerification__InvalidManageProofLength\",\n        \"inputs\": []\n    },\n    {\n        \"type\": \"error\",\n        \"name\": \"ManagerWithMerkleVerification__InvalidTargetDataLength\",\n        \"inputs\": []\n    },\n    {\n        \"type\": \"error\",\n        \"name\": \"ManagerWithMerkleVerification__InvalidValuesLength\",\n        \"inputs\": []\n    },\n    {\n        \"type\": \"error\",\n        \"name\": \"ManagerWithMerkleVerification__OnlyCallableByBalancerVault\",\n        \"inputs\": []\n    },\n    {\n        \"type\": \"error\",\n        \"name\": \"ManagerWithMerkleVerification__OnlyCallableByBoringVault\",\n        \"inputs\": []\n    },\n    {\n        \"type\": \"error\",\n        \"name\": \"ManagerWithMerkleVerification__Paused\",\n        \"inputs\": []\n    },\n    {\n        \"type\": \"error\",\n        \"name\": \"ManagerWithMerkleVerification__TotalSupplyMustRemainConstantDuringPlatform\",\n        \"inputs\": []\n    }\n]\n") . expect ("invalid abi")
        });
    #[derive(Clone)]
    pub struct ManagerWithMerkleVerification<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for ManagerWithMerkleVerification<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for ManagerWithMerkleVerification<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(ManagerWithMerkleVerification))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> ManagerWithMerkleVerification<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            let contract = ethers::contract::Contract::new(
                address.into(),
                MANAGERWITHMERKLEVERIFICATION_ABI.clone(),
                client,
            );
            Self(contract)
        }
        #[doc = "Calls the contract's `authority` (0xbf7e214f) function"]
        pub fn authority(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([191, 126, 33, 79], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `balancerVault` (0x158274a5) function"]
        pub fn balancer_vault(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([21, 130, 116, 165], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `flashLoan` (0x5c38449e) function"]
        pub fn flash_loan(
            &self,
            recipient: ethers::core::types::Address,
            tokens: ::std::vec::Vec<ethers::core::types::Address>,
            amounts: ::std::vec::Vec<ethers::core::types::U256>,
            user_data: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([92, 56, 68, 158], (recipient, tokens, amounts, user_data))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isPaused` (0xb187bd26) function"]
        pub fn is_paused(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([177, 135, 189, 38], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `manageRoot` (0x5ca58a99) function"]
        pub fn manage_root(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([92, 165, 138, 153], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `manageVaultWithMerkleVerification` (0x244b0f6a) function"]
        pub fn manage_vault_with_merkle_verification(
            &self,
            manage_proofs: ::std::vec::Vec<::std::vec::Vec<[u8; 32]>>,
            decoders_and_sanitizers: ::std::vec::Vec<ethers::core::types::Address>,
            targets: ::std::vec::Vec<ethers::core::types::Address>,
            target_data: ::std::vec::Vec<ethers::core::types::Bytes>,
            values: ::std::vec::Vec<ethers::core::types::U256>,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [36, 75, 15, 106],
                    (
                        manage_proofs,
                        decoders_and_sanitizers,
                        targets,
                        target_data,
                        values,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `owner` (0x8da5cb5b) function"]
        pub fn owner(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `pause` (0x8456cb59) function"]
        pub fn pause(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([132, 86, 203, 89], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `receiveFlashLoan` (0xf04f2707) function"]
        pub fn receive_flash_loan(
            &self,
            tokens: ::std::vec::Vec<ethers::core::types::Address>,
            amounts: ::std::vec::Vec<ethers::core::types::U256>,
            fee_amounts: ::std::vec::Vec<ethers::core::types::U256>,
            user_data: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([240, 79, 39, 7], (tokens, amounts, fee_amounts, user_data))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setAuthority` (0x7a9e5e4b) function"]
        pub fn set_authority(
            &self,
            new_authority: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([122, 158, 94, 75], new_authority)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setManageRoot` (0x21801a99) function"]
        pub fn set_manage_root(
            &self,
            strategist: ethers::core::types::Address,
            manage_root: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([33, 128, 26, 153], (strategist, manage_root))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferOwnership` (0xf2fde38b) function"]
        pub fn transfer_ownership(
            &self,
            new_owner: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `unpause` (0x3f4ba83a) function"]
        pub fn unpause(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([63, 75, 168, 58], ())
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
        #[doc = "Gets the contract's `AuthorityUpdated` event"]
        pub fn authority_updated_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, AuthorityUpdatedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `BoringVaultManaged` event"]
        pub fn boring_vault_managed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, BoringVaultManagedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ManageRootUpdated` event"]
        pub fn manage_root_updated_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ManageRootUpdatedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `OwnershipTransferred` event"]
        pub fn ownership_transferred_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, OwnershipTransferredFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Paused` event"]
        pub fn paused_filter(&self) -> ethers::contract::builders::Event<M, PausedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Unpaused` event"]
        pub fn unpaused_filter(&self) -> ethers::contract::builders::Event<M, UnpausedFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(
            &self,
        ) -> ethers::contract::builders::Event<M, ManagerWithMerkleVerificationEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethevent(name = "AuthorityUpdated", abi = "AuthorityUpdated(address,address)")]
    pub struct AuthorityUpdatedFilter {
        #[ethevent(indexed)]
        pub user: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_authority: ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethevent(name = "BoringVaultManaged", abi = "BoringVaultManaged(uint256)")]
    pub struct BoringVaultManagedFilter {
        pub calls_made: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethevent(
        name = "ManageRootUpdated",
        abi = "ManageRootUpdated(address,bytes32,bytes32)"
    )]
    pub struct ManageRootUpdatedFilter {
        #[ethevent(indexed)]
        pub strategist: ethers::core::types::Address,
        pub old_root: [u8; 32],
        pub new_root: [u8; 32],
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethevent(
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub user: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethevent(name = "Paused", abi = "Paused()")]
    pub struct PausedFilter();
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethevent(name = "Unpaused", abi = "Unpaused()")]
    pub struct UnpausedFilter();
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ManagerWithMerkleVerificationEvents {
        AuthorityUpdatedFilter(AuthorityUpdatedFilter),
        BoringVaultManagedFilter(BoringVaultManagedFilter),
        ManageRootUpdatedFilter(ManageRootUpdatedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        PausedFilter(PausedFilter),
        UnpausedFilter(UnpausedFilter),
    }
    impl ethers::contract::EthLogDecode for ManagerWithMerkleVerificationEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = AuthorityUpdatedFilter::decode_log(log) {
                return Ok(ManagerWithMerkleVerificationEvents::AuthorityUpdatedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = BoringVaultManagedFilter::decode_log(log) {
                return Ok(ManagerWithMerkleVerificationEvents::BoringVaultManagedFilter(decoded));
            }
            if let Ok(decoded) = ManageRootUpdatedFilter::decode_log(log) {
                return Ok(ManagerWithMerkleVerificationEvents::ManageRootUpdatedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(ManagerWithMerkleVerificationEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = PausedFilter::decode_log(log) {
                return Ok(ManagerWithMerkleVerificationEvents::PausedFilter(decoded));
            }
            if let Ok(decoded) = UnpausedFilter::decode_log(log) {
                return Ok(ManagerWithMerkleVerificationEvents::UnpausedFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for ManagerWithMerkleVerificationEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ManagerWithMerkleVerificationEvents::AuthorityUpdatedFilter(element) => {
                    element.fmt(f)
                }
                ManagerWithMerkleVerificationEvents::BoringVaultManagedFilter(element) => {
                    element.fmt(f)
                }
                ManagerWithMerkleVerificationEvents::ManageRootUpdatedFilter(element) => {
                    element.fmt(f)
                }
                ManagerWithMerkleVerificationEvents::OwnershipTransferredFilter(element) => {
                    element.fmt(f)
                }
                ManagerWithMerkleVerificationEvents::PausedFilter(element) => element.fmt(f),
                ManagerWithMerkleVerificationEvents::UnpausedFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `authority`function with signature `authority()` and selector `[191, 126, 33, 79]`"]
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
    #[ethcall(name = "authority", abi = "authority()")]
    pub struct AuthorityCall;
    #[doc = "Container type for all input parameters for the `balancerVault`function with signature `balancerVault()` and selector `[21, 130, 116, 165]`"]
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
    #[ethcall(name = "balancerVault", abi = "balancerVault()")]
    pub struct BalancerVaultCall;
    #[doc = "Container type for all input parameters for the `flashLoan`function with signature `flashLoan(address,address[],uint256[],bytes)` and selector `[92, 56, 68, 158]`"]
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
        name = "flashLoan",
        abi = "flashLoan(address,address[],uint256[],bytes)"
    )]
    pub struct FlashLoanCall {
        pub recipient: ethers::core::types::Address,
        pub tokens: ::std::vec::Vec<ethers::core::types::Address>,
        pub amounts: ::std::vec::Vec<ethers::core::types::U256>,
        pub user_data: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `isPaused`function with signature `isPaused()` and selector `[177, 135, 189, 38]`"]
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
    #[ethcall(name = "isPaused", abi = "isPaused()")]
    pub struct IsPausedCall;
    #[doc = "Container type for all input parameters for the `manageRoot`function with signature `manageRoot(address)` and selector `[92, 165, 138, 153]`"]
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
    #[ethcall(name = "manageRoot", abi = "manageRoot(address)")]
    pub struct ManageRootCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `manageVaultWithMerkleVerification`function with signature `manageVaultWithMerkleVerification(bytes32[][],address[],address[],bytes[],uint256[])` and selector `[36, 75, 15, 106]`"]
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
        name = "manageVaultWithMerkleVerification",
        abi = "manageVaultWithMerkleVerification(bytes32[][],address[],address[],bytes[],uint256[])"
    )]
    pub struct ManageVaultWithMerkleVerificationCall {
        pub manage_proofs: ::std::vec::Vec<::std::vec::Vec<[u8; 32]>>,
        pub decoders_and_sanitizers: ::std::vec::Vec<ethers::core::types::Address>,
        pub targets: ::std::vec::Vec<ethers::core::types::Address>,
        pub target_data: ::std::vec::Vec<ethers::core::types::Bytes>,
        pub values: ::std::vec::Vec<ethers::core::types::U256>,
    }
    #[doc = "Container type for all input parameters for the `owner`function with signature `owner()` and selector `[141, 165, 203, 91]`"]
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
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    #[doc = "Container type for all input parameters for the `pause`function with signature `pause()` and selector `[132, 86, 203, 89]`"]
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
    #[ethcall(name = "pause", abi = "pause()")]
    pub struct PauseCall;
    #[doc = "Container type for all input parameters for the `receiveFlashLoan`function with signature `receiveFlashLoan(address[],uint256[],uint256[],bytes)` and selector `[240, 79, 39, 7]`"]
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
        name = "receiveFlashLoan",
        abi = "receiveFlashLoan(address[],uint256[],uint256[],bytes)"
    )]
    pub struct ReceiveFlashLoanCall {
        pub tokens: ::std::vec::Vec<ethers::core::types::Address>,
        pub amounts: ::std::vec::Vec<ethers::core::types::U256>,
        pub fee_amounts: ::std::vec::Vec<ethers::core::types::U256>,
        pub user_data: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `setAuthority`function with signature `setAuthority(address)` and selector `[122, 158, 94, 75]`"]
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
    #[ethcall(name = "setAuthority", abi = "setAuthority(address)")]
    pub struct SetAuthorityCall {
        pub new_authority: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setManageRoot`function with signature `setManageRoot(address,bytes32)` and selector `[33, 128, 26, 153]`"]
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
    #[ethcall(name = "setManageRoot", abi = "setManageRoot(address,bytes32)")]
    pub struct SetManageRootCall {
        pub strategist: ethers::core::types::Address,
        pub manage_root: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `transferOwnership`function with signature `transferOwnership(address)` and selector `[242, 253, 227, 139]`"]
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
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `unpause`function with signature `unpause()` and selector `[63, 75, 168, 58]`"]
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
    #[ethcall(name = "unpause", abi = "unpause()")]
    pub struct UnpauseCall;
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
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ManagerWithMerkleVerificationCalls {
        Authority(AuthorityCall),
        BalancerVault(BalancerVaultCall),
        FlashLoan(FlashLoanCall),
        IsPaused(IsPausedCall),
        ManageRoot(ManageRootCall),
        ManageVaultWithMerkleVerification(ManageVaultWithMerkleVerificationCall),
        Owner(OwnerCall),
        Pause(PauseCall),
        ReceiveFlashLoan(ReceiveFlashLoanCall),
        SetAuthority(SetAuthorityCall),
        SetManageRoot(SetManageRootCall),
        TransferOwnership(TransferOwnershipCall),
        Unpause(UnpauseCall),
        Vault(VaultCall),
    }
    impl ethers::core::abi::AbiDecode for ManagerWithMerkleVerificationCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AuthorityCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ManagerWithMerkleVerificationCalls::Authority(decoded));
            }
            if let Ok(decoded) =
                <BalancerVaultCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ManagerWithMerkleVerificationCalls::BalancerVault(decoded));
            }
            if let Ok(decoded) =
                <FlashLoanCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ManagerWithMerkleVerificationCalls::FlashLoan(decoded));
            }
            if let Ok(decoded) =
                <IsPausedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ManagerWithMerkleVerificationCalls::IsPaused(decoded));
            }
            if let Ok(decoded) =
                <ManageRootCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ManagerWithMerkleVerificationCalls::ManageRoot(decoded));
            }
            if let Ok(decoded) =
                <ManageVaultWithMerkleVerificationCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(
                    ManagerWithMerkleVerificationCalls::ManageVaultWithMerkleVerification(decoded),
                );
            }
            if let Ok(decoded) = <OwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ManagerWithMerkleVerificationCalls::Owner(decoded));
            }
            if let Ok(decoded) = <PauseCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ManagerWithMerkleVerificationCalls::Pause(decoded));
            }
            if let Ok(decoded) =
                <ReceiveFlashLoanCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ManagerWithMerkleVerificationCalls::ReceiveFlashLoan(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <SetAuthorityCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ManagerWithMerkleVerificationCalls::SetAuthority(decoded));
            }
            if let Ok(decoded) =
                <SetManageRootCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ManagerWithMerkleVerificationCalls::SetManageRoot(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ManagerWithMerkleVerificationCalls::TransferOwnership(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <UnpauseCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ManagerWithMerkleVerificationCalls::Unpause(decoded));
            }
            if let Ok(decoded) = <VaultCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ManagerWithMerkleVerificationCalls::Vault(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for ManagerWithMerkleVerificationCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                ManagerWithMerkleVerificationCalls::Authority(element) => element.encode(),
                ManagerWithMerkleVerificationCalls::BalancerVault(element) => element.encode(),
                ManagerWithMerkleVerificationCalls::FlashLoan(element) => element.encode(),
                ManagerWithMerkleVerificationCalls::IsPaused(element) => element.encode(),
                ManagerWithMerkleVerificationCalls::ManageRoot(element) => element.encode(),
                ManagerWithMerkleVerificationCalls::ManageVaultWithMerkleVerification(element) => {
                    element.encode()
                }
                ManagerWithMerkleVerificationCalls::Owner(element) => element.encode(),
                ManagerWithMerkleVerificationCalls::Pause(element) => element.encode(),
                ManagerWithMerkleVerificationCalls::ReceiveFlashLoan(element) => element.encode(),
                ManagerWithMerkleVerificationCalls::SetAuthority(element) => element.encode(),
                ManagerWithMerkleVerificationCalls::SetManageRoot(element) => element.encode(),
                ManagerWithMerkleVerificationCalls::TransferOwnership(element) => element.encode(),
                ManagerWithMerkleVerificationCalls::Unpause(element) => element.encode(),
                ManagerWithMerkleVerificationCalls::Vault(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for ManagerWithMerkleVerificationCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ManagerWithMerkleVerificationCalls::Authority(element) => element.fmt(f),
                ManagerWithMerkleVerificationCalls::BalancerVault(element) => element.fmt(f),
                ManagerWithMerkleVerificationCalls::FlashLoan(element) => element.fmt(f),
                ManagerWithMerkleVerificationCalls::IsPaused(element) => element.fmt(f),
                ManagerWithMerkleVerificationCalls::ManageRoot(element) => element.fmt(f),
                ManagerWithMerkleVerificationCalls::ManageVaultWithMerkleVerification(element) => {
                    element.fmt(f)
                }
                ManagerWithMerkleVerificationCalls::Owner(element) => element.fmt(f),
                ManagerWithMerkleVerificationCalls::Pause(element) => element.fmt(f),
                ManagerWithMerkleVerificationCalls::ReceiveFlashLoan(element) => element.fmt(f),
                ManagerWithMerkleVerificationCalls::SetAuthority(element) => element.fmt(f),
                ManagerWithMerkleVerificationCalls::SetManageRoot(element) => element.fmt(f),
                ManagerWithMerkleVerificationCalls::TransferOwnership(element) => element.fmt(f),
                ManagerWithMerkleVerificationCalls::Unpause(element) => element.fmt(f),
                ManagerWithMerkleVerificationCalls::Vault(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AuthorityCall> for ManagerWithMerkleVerificationCalls {
        fn from(var: AuthorityCall) -> Self {
            ManagerWithMerkleVerificationCalls::Authority(var)
        }
    }
    impl ::std::convert::From<BalancerVaultCall> for ManagerWithMerkleVerificationCalls {
        fn from(var: BalancerVaultCall) -> Self {
            ManagerWithMerkleVerificationCalls::BalancerVault(var)
        }
    }
    impl ::std::convert::From<FlashLoanCall> for ManagerWithMerkleVerificationCalls {
        fn from(var: FlashLoanCall) -> Self {
            ManagerWithMerkleVerificationCalls::FlashLoan(var)
        }
    }
    impl ::std::convert::From<IsPausedCall> for ManagerWithMerkleVerificationCalls {
        fn from(var: IsPausedCall) -> Self {
            ManagerWithMerkleVerificationCalls::IsPaused(var)
        }
    }
    impl ::std::convert::From<ManageRootCall> for ManagerWithMerkleVerificationCalls {
        fn from(var: ManageRootCall) -> Self {
            ManagerWithMerkleVerificationCalls::ManageRoot(var)
        }
    }
    impl ::std::convert::From<ManageVaultWithMerkleVerificationCall>
        for ManagerWithMerkleVerificationCalls
    {
        fn from(var: ManageVaultWithMerkleVerificationCall) -> Self {
            ManagerWithMerkleVerificationCalls::ManageVaultWithMerkleVerification(var)
        }
    }
    impl ::std::convert::From<OwnerCall> for ManagerWithMerkleVerificationCalls {
        fn from(var: OwnerCall) -> Self {
            ManagerWithMerkleVerificationCalls::Owner(var)
        }
    }
    impl ::std::convert::From<PauseCall> for ManagerWithMerkleVerificationCalls {
        fn from(var: PauseCall) -> Self {
            ManagerWithMerkleVerificationCalls::Pause(var)
        }
    }
    impl ::std::convert::From<ReceiveFlashLoanCall> for ManagerWithMerkleVerificationCalls {
        fn from(var: ReceiveFlashLoanCall) -> Self {
            ManagerWithMerkleVerificationCalls::ReceiveFlashLoan(var)
        }
    }
    impl ::std::convert::From<SetAuthorityCall> for ManagerWithMerkleVerificationCalls {
        fn from(var: SetAuthorityCall) -> Self {
            ManagerWithMerkleVerificationCalls::SetAuthority(var)
        }
    }
    impl ::std::convert::From<SetManageRootCall> for ManagerWithMerkleVerificationCalls {
        fn from(var: SetManageRootCall) -> Self {
            ManagerWithMerkleVerificationCalls::SetManageRoot(var)
        }
    }
    impl ::std::convert::From<TransferOwnershipCall> for ManagerWithMerkleVerificationCalls {
        fn from(var: TransferOwnershipCall) -> Self {
            ManagerWithMerkleVerificationCalls::TransferOwnership(var)
        }
    }
    impl ::std::convert::From<UnpauseCall> for ManagerWithMerkleVerificationCalls {
        fn from(var: UnpauseCall) -> Self {
            ManagerWithMerkleVerificationCalls::Unpause(var)
        }
    }
    impl ::std::convert::From<VaultCall> for ManagerWithMerkleVerificationCalls {
        fn from(var: VaultCall) -> Self {
            ManagerWithMerkleVerificationCalls::Vault(var)
        }
    }
}
