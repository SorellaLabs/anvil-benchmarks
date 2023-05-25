pub use convex_mod::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types
)]
pub mod convex_mod {
    #[rustfmt::skip]
    const __ABI: &str = "[\n  {\n    \"inputs\": [\n      { \"internalType\": \"address\", \"name\": \"_staker\", \"type\": \"address\" },\n      { \"internalType\": \"address\", \"name\": \"_minter\", \"type\": \"address\" }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"constructor\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"user\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"uint256\",\n        \"name\": \"poolid\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"Deposited\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"user\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"uint256\",\n        \"name\": \"poolid\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"Withdrawn\",\n    \"type\": \"event\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"FEE_DENOMINATOR\",\n    \"outputs\": [{ \"internalType\": \"uint256\", \"name\": \"\", \"type\": \"uint256\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"MaxFees\",\n    \"outputs\": [{ \"internalType\": \"uint256\", \"name\": \"\", \"type\": \"uint256\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"address\", \"name\": \"_lptoken\", \"type\": \"address\" },\n      { \"internalType\": \"address\", \"name\": \"_gauge\", \"type\": \"address\" },\n      { \"internalType\": \"uint256\", \"name\": \"_stashVersion\", \"type\": \"uint256\" }\n    ],\n    \"name\": \"addPool\",\n    \"outputs\": [{ \"internalType\": \"bool\", \"name\": \"\", \"type\": \"bool\" }],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"uint256\", \"name\": \"_pid\", \"type\": \"uint256\" },\n      { \"internalType\": \"address\", \"name\": \"_gauge\", \"type\": \"address\" }\n    ],\n    \"name\": \"claimRewards\",\n    \"outputs\": [{ \"internalType\": \"bool\", \"name\": \"\", \"type\": \"bool\" }],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"crv\",\n    \"outputs\": [{ \"internalType\": \"address\", \"name\": \"\", \"type\": \"address\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"uint256\", \"name\": \"_pid\", \"type\": \"uint256\" },\n      { \"internalType\": \"uint256\", \"name\": \"_amount\", \"type\": \"uint256\" },\n      { \"internalType\": \"bool\", \"name\": \"_stake\", \"type\": \"bool\" }\n    ],\n    \"name\": \"deposit\",\n    \"outputs\": [{ \"internalType\": \"bool\", \"name\": \"\", \"type\": \"bool\" }],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"uint256\", \"name\": \"_pid\", \"type\": \"uint256\" },\n      { \"internalType\": \"bool\", \"name\": \"_stake\", \"type\": \"bool\" }\n    ],\n    \"name\": \"depositAll\",\n    \"outputs\": [{ \"internalType\": \"bool\", \"name\": \"\", \"type\": \"bool\" }],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"distributionAddressId\",\n    \"outputs\": [{ \"internalType\": \"uint256\", \"name\": \"\", \"type\": \"uint256\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"earmarkFees\",\n    \"outputs\": [{ \"internalType\": \"bool\", \"name\": \"\", \"type\": \"bool\" }],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"earmarkIncentive\",\n    \"outputs\": [{ \"internalType\": \"uint256\", \"name\": \"\", \"type\": \"uint256\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"uint256\", \"name\": \"_pid\", \"type\": \"uint256\" }\n    ],\n    \"name\": \"earmarkRewards\",\n    \"outputs\": [{ \"internalType\": \"bool\", \"name\": \"\", \"type\": \"bool\" }],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"feeDistro\",\n    \"outputs\": [{ \"internalType\": \"address\", \"name\": \"\", \"type\": \"address\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"feeManager\",\n    \"outputs\": [{ \"internalType\": \"address\", \"name\": \"\", \"type\": \"address\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"feeToken\",\n    \"outputs\": [{ \"internalType\": \"address\", \"name\": \"\", \"type\": \"address\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [{ \"internalType\": \"address\", \"name\": \"\", \"type\": \"address\" }],\n    \"name\": \"gaugeMap\",\n    \"outputs\": [{ \"internalType\": \"bool\", \"name\": \"\", \"type\": \"bool\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"isShutdown\",\n    \"outputs\": [{ \"internalType\": \"bool\", \"name\": \"\", \"type\": \"bool\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"lockFees\",\n    \"outputs\": [{ \"internalType\": \"address\", \"name\": \"\", \"type\": \"address\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"lockIncentive\",\n    \"outputs\": [{ \"internalType\": \"uint256\", \"name\": \"\", \"type\": \"uint256\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"lockRewards\",\n    \"outputs\": [{ \"internalType\": \"address\", \"name\": \"\", \"type\": \"address\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"minter\",\n    \"outputs\": [{ \"internalType\": \"address\", \"name\": \"\", \"type\": \"address\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"owner\",\n    \"outputs\": [{ \"internalType\": \"address\", \"name\": \"\", \"type\": \"address\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"platformFee\",\n    \"outputs\": [{ \"internalType\": \"uint256\", \"name\": \"\", \"type\": \"uint256\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [{ \"internalType\": \"uint256\", \"name\": \"\", \"type\": \"uint256\" }],\n    \"name\": \"poolInfo\",\n    \"outputs\": [\n      { \"internalType\": \"address\", \"name\": \"lptoken\", \"type\": \"address\" },\n      { \"internalType\": \"address\", \"name\": \"token\", \"type\": \"address\" },\n      { \"internalType\": \"address\", \"name\": \"gauge\", \"type\": \"address\" },\n      { \"internalType\": \"address\", \"name\": \"crvRewards\", \"type\": \"address\" },\n      { \"internalType\": \"address\", \"name\": \"stash\", \"type\": \"address\" },\n      { \"internalType\": \"bool\", \"name\": \"shutdown\", \"type\": \"bool\" }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"poolLength\",\n    \"outputs\": [{ \"internalType\": \"uint256\", \"name\": \"\", \"type\": \"uint256\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"poolManager\",\n    \"outputs\": [{ \"internalType\": \"address\", \"name\": \"\", \"type\": \"address\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"registry\",\n    \"outputs\": [{ \"internalType\": \"address\", \"name\": \"\", \"type\": \"address\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"rewardArbitrator\",\n    \"outputs\": [{ \"internalType\": \"address\", \"name\": \"\", \"type\": \"address\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"uint256\", \"name\": \"_pid\", \"type\": \"uint256\" },\n      { \"internalType\": \"address\", \"name\": \"_address\", \"type\": \"address\" },\n      { \"internalType\": \"uint256\", \"name\": \"_amount\", \"type\": \"uint256\" }\n    ],\n    \"name\": \"rewardClaimed\",\n    \"outputs\": [{ \"internalType\": \"bool\", \"name\": \"\", \"type\": \"bool\" }],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"rewardFactory\",\n    \"outputs\": [{ \"internalType\": \"address\", \"name\": \"\", \"type\": \"address\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"address\", \"name\": \"_arb\", \"type\": \"address\" }\n    ],\n    \"name\": \"setArbitrator\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"address\", \"name\": \"_rfactory\", \"type\": \"address\" },\n      { \"internalType\": \"address\", \"name\": \"_sfactory\", \"type\": \"address\" },\n      { \"internalType\": \"address\", \"name\": \"_tfactory\", \"type\": \"address\" }\n    ],\n    \"name\": \"setFactories\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"setFeeInfo\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"address\", \"name\": \"_feeM\", \"type\": \"address\" }\n    ],\n    \"name\": \"setFeeManager\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"uint256\", \"name\": \"_lockFees\", \"type\": \"uint256\" },\n      { \"internalType\": \"uint256\", \"name\": \"_stakerFees\", \"type\": \"uint256\" },\n      { \"internalType\": \"uint256\", \"name\": \"_callerFees\", \"type\": \"uint256\" },\n      { \"internalType\": \"uint256\", \"name\": \"_platform\", \"type\": \"uint256\" }\n    ],\n    \"name\": \"setFees\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"uint256\", \"name\": \"_pid\", \"type\": \"uint256\" }\n    ],\n    \"name\": \"setGaugeRedirect\",\n    \"outputs\": [{ \"internalType\": \"bool\", \"name\": \"\", \"type\": \"bool\" }],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"address\", \"name\": \"_owner\", \"type\": \"address\" }\n    ],\n    \"name\": \"setOwner\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"address\", \"name\": \"_poolM\", \"type\": \"address\" }\n    ],\n    \"name\": \"setPoolManager\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"address\", \"name\": \"_rewards\", \"type\": \"address\" },\n      { \"internalType\": \"address\", \"name\": \"_stakerRewards\", \"type\": \"address\" }\n    ],\n    \"name\": \"setRewardContracts\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"address\", \"name\": \"_treasury\", \"type\": \"address\" }\n    ],\n    \"name\": \"setTreasury\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"address\", \"name\": \"_voteDelegate\", \"type\": \"address\" }\n    ],\n    \"name\": \"setVoteDelegate\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"uint256\", \"name\": \"_pid\", \"type\": \"uint256\" }\n    ],\n    \"name\": \"shutdownPool\",\n    \"outputs\": [{ \"internalType\": \"bool\", \"name\": \"\", \"type\": \"bool\" }],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"shutdownSystem\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"staker\",\n    \"outputs\": [{ \"internalType\": \"address\", \"name\": \"\", \"type\": \"address\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"stakerIncentive\",\n    \"outputs\": [{ \"internalType\": \"uint256\", \"name\": \"\", \"type\": \"uint256\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"stakerRewards\",\n    \"outputs\": [{ \"internalType\": \"address\", \"name\": \"\", \"type\": \"address\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"stashFactory\",\n    \"outputs\": [{ \"internalType\": \"address\", \"name\": \"\", \"type\": \"address\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"tokenFactory\",\n    \"outputs\": [{ \"internalType\": \"address\", \"name\": \"\", \"type\": \"address\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"treasury\",\n    \"outputs\": [{ \"internalType\": \"address\", \"name\": \"\", \"type\": \"address\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"uint256\", \"name\": \"_voteId\", \"type\": \"uint256\" },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_votingAddress\",\n        \"type\": \"address\"\n      },\n      { \"internalType\": \"bool\", \"name\": \"_support\", \"type\": \"bool\" }\n    ],\n    \"name\": \"vote\",\n    \"outputs\": [{ \"internalType\": \"bool\", \"name\": \"\", \"type\": \"bool\" }],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"voteDelegate\",\n    \"outputs\": [{ \"internalType\": \"address\", \"name\": \"\", \"type\": \"address\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"address[]\", \"name\": \"_gauge\", \"type\": \"address[]\" },\n      { \"internalType\": \"uint256[]\", \"name\": \"_weight\", \"type\": \"uint256[]\" }\n    ],\n    \"name\": \"voteGaugeWeight\",\n    \"outputs\": [{ \"internalType\": \"bool\", \"name\": \"\", \"type\": \"bool\" }],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"voteOwnership\",\n    \"outputs\": [{ \"internalType\": \"address\", \"name\": \"\", \"type\": \"address\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"voteParameter\",\n    \"outputs\": [{ \"internalType\": \"address\", \"name\": \"\", \"type\": \"address\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"uint256\", \"name\": \"_pid\", \"type\": \"uint256\" },\n      { \"internalType\": \"uint256\", \"name\": \"_amount\", \"type\": \"uint256\" }\n    ],\n    \"name\": \"withdraw\",\n    \"outputs\": [{ \"internalType\": \"bool\", \"name\": \"\", \"type\": \"bool\" }],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"uint256\", \"name\": \"_pid\", \"type\": \"uint256\" }\n    ],\n    \"name\": \"withdrawAll\",\n    \"outputs\": [{ \"internalType\": \"bool\", \"name\": \"\", \"type\": \"bool\" }],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"uint256\", \"name\": \"_pid\", \"type\": \"uint256\" },\n      { \"internalType\": \"uint256\", \"name\": \"_amount\", \"type\": \"uint256\" },\n      { \"internalType\": \"address\", \"name\": \"_to\", \"type\": \"address\" }\n    ],\n    \"name\": \"withdrawTo\",\n    \"outputs\": [{ \"internalType\": \"bool\", \"name\": \"\", \"type\": \"bool\" }],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  }\n]\n";
    ///The parsed JSON ABI of the contract.
    pub static CONVEX_MOD_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(|| {
            ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid")
        });
    pub struct convex_mod<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for convex_mod<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for convex_mod<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for convex_mod<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for convex_mod<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(convex_mod)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> convex_mod<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(address.into(), CONVEX_MOD_ABI.clone(), client))
        }
        ///Calls the contract's `FEE_DENOMINATOR` (0xd73792a9) function
        pub fn fee_denominator(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([215, 55, 146, 169], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `MaxFees` (0x7303df9a) function
        pub fn max_fees(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([115, 3, 223, 154], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addPool` (0x7e29d6c2) function
        pub fn add_pool(
            &self,
            lptoken: ::ethers::core::types::Address,
            gauge: ::ethers::core::types::Address,
            stash_version: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([126, 41, 214, 194], (lptoken, gauge, stash_version))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `claimRewards` (0x6c7b69cb) function
        pub fn claim_rewards(
            &self,
            pid: ::ethers::core::types::U256,
            gauge: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([108, 123, 105, 203], (pid, gauge))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `crv` (0x6a4874a1) function
        pub fn crv(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([106, 72, 116, 161], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deposit` (0x43a0d066) function
        pub fn deposit(
            &self,
            pid: ::ethers::core::types::U256,
            amount: ::ethers::core::types::U256,
            stake: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([67, 160, 208, 102], (pid, amount, stake))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `depositAll` (0x60759fce) function
        pub fn deposit_all(
            &self,
            pid: ::ethers::core::types::U256,
            stake: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([96, 117, 159, 206], (pid, stake))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `distributionAddressId` (0x93e846a0) function
        pub fn distribution_address_id(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([147, 232, 70, 160], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `earmarkFees` (0x22230b96) function
        pub fn earmark_fees(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([34, 35, 11, 150], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `earmarkIncentive` (0x3a088cd2) function
        pub fn earmark_incentive(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([58, 8, 140, 210], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `earmarkRewards` (0xcc956f3f) function
        pub fn earmark_rewards(
            &self,
            pid: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([204, 149, 111, 63], pid)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `feeDistro` (0xd6a0f530) function
        pub fn fee_distro(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([214, 160, 245, 48], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `feeManager` (0xd0fb0203) function
        pub fn fee_manager(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([208, 251, 2, 3], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `feeToken` (0x647846a5) function
        pub fn fee_token(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([100, 120, 70, 165], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `gaugeMap` (0xcb0d5b52) function
        pub fn gauge_map(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([203, 13, 91, 82], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isShutdown` (0xbf86d690) function
        pub fn is_shutdown(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([191, 134, 214, 144], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lockFees` (0xab366292) function
        pub fn lock_fees(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([171, 54, 98, 146], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lockIncentive` (0x50940618) function
        pub fn lock_incentive(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([80, 148, 6, 24], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lockRewards` (0x376d771a) function
        pub fn lock_rewards(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([55, 109, 119, 26], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `minter` (0x07546172) function
        pub fn minter(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([7, 84, 97, 114], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `owner` (0x8da5cb5b) function
        pub fn owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `platformFee` (0x26232a2e) function
        pub fn platform_fee(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([38, 35, 42, 46], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `poolInfo` (0x1526fe27) function
        pub fn pool_info(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::Address,
                ::ethers::core::types::Address,
                ::ethers::core::types::Address,
                ::ethers::core::types::Address,
                ::ethers::core::types::Address,
                bool,
            ),
        > {
            self.0
                .method_hash([21, 38, 254, 39], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `poolLength` (0x081e3eda) function
        pub fn pool_length(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([8, 30, 62, 218], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `poolManager` (0xdc4c90d3) function
        pub fn pool_manager(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([220, 76, 144, 211], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `registry` (0x7b103999) function
        pub fn registry(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([123, 16, 57, 153], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rewardArbitrator` (0x043b684a) function
        pub fn reward_arbitrator(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([4, 59, 104, 74], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rewardClaimed` (0x71192b17) function
        pub fn reward_claimed(
            &self,
            pid: ::ethers::core::types::U256,
            address: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([113, 25, 43, 23], (pid, address, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rewardFactory` (0x245e4bf0) function
        pub fn reward_factory(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([36, 94, 75, 240], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setArbitrator` (0xb0eefabe) function
        pub fn set_arbitrator(
            &self,
            arb: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([176, 238, 250, 190], arb)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setFactories` (0x7bd3b995) function
        pub fn set_factories(
            &self,
            rfactory: ::ethers::core::types::Address,
            sfactory: ::ethers::core::types::Address,
            tfactory: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([123, 211, 185, 149], (rfactory, sfactory, tfactory))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setFeeInfo` (0x5a4ae5ca) function
        pub fn set_fee_info(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([90, 74, 229, 202], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setFeeManager` (0x472d35b9) function
        pub fn set_fee_manager(
            &self,
            fee_m: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([71, 45, 53, 185], fee_m)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setFees` (0x6fcba377) function
        pub fn set_fees(
            &self,
            lock_fees: ::ethers::core::types::U256,
            staker_fees: ::ethers::core::types::U256,
            caller_fees: ::ethers::core::types::U256,
            platform: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([111, 203, 163, 119], (lock_fees, staker_fees, caller_fees, platform))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setGaugeRedirect` (0x9123d404) function
        pub fn set_gauge_redirect(
            &self,
            pid: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([145, 35, 212, 4], pid)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setOwner` (0x13af4035) function
        pub fn set_owner(
            &self,
            owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([19, 175, 64, 53], owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setPoolManager` (0x7aef6715) function
        pub fn set_pool_manager(
            &self,
            pool_m: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([122, 239, 103, 21], pool_m)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setRewardContracts` (0x95539a1d) function
        pub fn set_reward_contracts(
            &self,
            rewards: ::ethers::core::types::Address,
            staker_rewards: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([149, 83, 154, 29], (rewards, staker_rewards))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setTreasury` (0xf0f44260) function
        pub fn set_treasury(
            &self,
            treasury: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([240, 244, 66, 96], treasury)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setVoteDelegate` (0x74874323) function
        pub fn set_vote_delegate(
            &self,
            vote_delegate: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([116, 135, 67, 35], vote_delegate)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `shutdownPool` (0x60cafe84) function
        pub fn shutdown_pool(
            &self,
            pid: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([96, 202, 254, 132], pid)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `shutdownSystem` (0x354af919) function
        pub fn shutdown_system(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([53, 74, 249, 25], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `staker` (0x5ebaf1db) function
        pub fn staker(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([94, 186, 241, 219], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `stakerIncentive` (0x62d28ac7) function
        pub fn staker_incentive(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([98, 210, 138, 199], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `stakerRewards` (0xcfb9cfba) function
        pub fn staker_rewards(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([207, 185, 207, 186], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `stashFactory` (0x068eb19e) function
        pub fn stash_factory(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([6, 142, 177, 158], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `tokenFactory` (0xe77772fe) function
        pub fn token_factory(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([231, 119, 114, 254], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `treasury` (0x61d027b3) function
        pub fn treasury(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([97, 208, 39, 179], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `vote` (0xe2cdd42a) function
        pub fn vote(
            &self,
            vote_id: ::ethers::core::types::U256,
            voting_address: ::ethers::core::types::Address,
            support: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([226, 205, 212, 42], (vote_id, voting_address, support))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `voteDelegate` (0x9f00332b) function
        pub fn vote_delegate(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([159, 0, 51, 43], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `voteGaugeWeight` (0xbfad96ba) function
        pub fn vote_gauge_weight(
            &self,
            gauge: ::std::vec::Vec<::ethers::core::types::Address>,
            weight: ::std::vec::Vec<::ethers::core::types::U256>,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([191, 173, 150, 186], (gauge, weight))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `voteOwnership` (0xa386a080) function
        pub fn vote_ownership(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([163, 134, 160, 128], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `voteParameter` (0xb42eda71) function
        pub fn vote_parameter(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([180, 46, 218, 113], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdraw` (0x441a3e70) function
        pub fn withdraw(
            &self,
            pid: ::ethers::core::types::U256,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([68, 26, 62, 112], (pid, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawAll` (0x958e2d31) function
        pub fn withdraw_all(
            &self,
            pid: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([149, 142, 45, 49], pid)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawTo` (0x14cd70e4) function
        pub fn withdraw_to(
            &self,
            pid: ::ethers::core::types::U256,
            amount: ::ethers::core::types::U256,
            to: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([20, 205, 112, 228], (pid, amount, to))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Deposited` event
        pub fn deposited_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, DepositedFilter> {
            self.0.event()
        }
        ///Gets the contract's `Withdrawn` event
        pub fn withdrawn_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, WithdrawnFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, convex_modEvents> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for convex_mod<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "Deposited", abi = "Deposited(address,uint256,uint256)")]
    pub struct DepositedFilter {
        #[ethevent(indexed)]
        pub user: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub poolid: ::ethers::core::types::U256,
        pub amount: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "Withdrawn", abi = "Withdrawn(address,uint256,uint256)")]
    pub struct WithdrawnFilter {
        #[ethevent(indexed)]
        pub user: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub poolid: ::ethers::core::types::U256,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum convex_modEvents {
        DepositedFilter(DepositedFilter),
        WithdrawnFilter(WithdrawnFilter),
    }
    impl ::ethers::contract::EthLogDecode for convex_modEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = DepositedFilter::decode_log(log) {
                return Ok(convex_modEvents::DepositedFilter(decoded))
            }
            if let Ok(decoded) = WithdrawnFilter::decode_log(log) {
                return Ok(convex_modEvents::WithdrawnFilter(decoded))
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for convex_modEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DepositedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawnFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DepositedFilter> for convex_modEvents {
        fn from(value: DepositedFilter) -> Self {
            Self::DepositedFilter(value)
        }
    }
    impl ::core::convert::From<WithdrawnFilter> for convex_modEvents {
        fn from(value: WithdrawnFilter) -> Self {
            Self::WithdrawnFilter(value)
        }
    }
    ///Container type for all input parameters for the `FEE_DENOMINATOR` function with signature
    /// `FEE_DENOMINATOR()` and selector `0xd73792a9`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "FEE_DENOMINATOR", abi = "FEE_DENOMINATOR()")]
    pub struct FeeDenominatorCall;
    ///Container type for all input parameters for the `MaxFees` function with signature
    /// `MaxFees()` and selector `0x7303df9a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "MaxFees", abi = "MaxFees()")]
    pub struct MaxFeesCall;
    ///Container type for all input parameters for the `addPool` function with signature
    /// `addPool(address,address,uint256)` and selector `0x7e29d6c2`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "addPool", abi = "addPool(address,address,uint256)")]
    pub struct AddPoolCall {
        pub lptoken: ::ethers::core::types::Address,
        pub gauge: ::ethers::core::types::Address,
        pub stash_version: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `claimRewards` function with signature
    /// `claimRewards(uint256,address)` and selector `0x6c7b69cb`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "claimRewards", abi = "claimRewards(uint256,address)")]
    pub struct ClaimRewardsCall {
        pub pid: ::ethers::core::types::U256,
        pub gauge: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `crv` function with signature `crv()` and
    /// selector `0x6a4874a1`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "crv", abi = "crv()")]
    pub struct CrvCall;
    ///Container type for all input parameters for the `deposit` function with signature
    /// `deposit(uint256,uint256,bool)` and selector `0x43a0d066`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "deposit", abi = "deposit(uint256,uint256,bool)")]
    pub struct DepositCall {
        pub pid: ::ethers::core::types::U256,
        pub amount: ::ethers::core::types::U256,
        pub stake: bool,
    }
    ///Container type for all input parameters for the `depositAll` function with signature
    /// `depositAll(uint256,bool)` and selector `0x60759fce`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "depositAll", abi = "depositAll(uint256,bool)")]
    pub struct DepositAllCall {
        pub pid: ::ethers::core::types::U256,
        pub stake: bool,
    }
    ///Container type for all input parameters for the `distributionAddressId` function with
    /// signature `distributionAddressId()` and selector `0x93e846a0`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "distributionAddressId", abi = "distributionAddressId()")]
    pub struct DistributionAddressIdCall;
    ///Container type for all input parameters for the `earmarkFees` function with signature
    /// `earmarkFees()` and selector `0x22230b96`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "earmarkFees", abi = "earmarkFees()")]
    pub struct EarmarkFeesCall;
    ///Container type for all input parameters for the `earmarkIncentive` function with signature
    /// `earmarkIncentive()` and selector `0x3a088cd2`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "earmarkIncentive", abi = "earmarkIncentive()")]
    pub struct EarmarkIncentiveCall;
    ///Container type for all input parameters for the `earmarkRewards` function with signature
    /// `earmarkRewards(uint256)` and selector `0xcc956f3f`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "earmarkRewards", abi = "earmarkRewards(uint256)")]
    pub struct EarmarkRewardsCall {
        pub pid: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `feeDistro` function with signature
    /// `feeDistro()` and selector `0xd6a0f530`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "feeDistro", abi = "feeDistro()")]
    pub struct FeeDistroCall;
    ///Container type for all input parameters for the `feeManager` function with signature
    /// `feeManager()` and selector `0xd0fb0203`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "feeManager", abi = "feeManager()")]
    pub struct FeeManagerCall;
    ///Container type for all input parameters for the `feeToken` function with signature
    /// `feeToken()` and selector `0x647846a5`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "feeToken", abi = "feeToken()")]
    pub struct FeeTokenCall;
    ///Container type for all input parameters for the `gaugeMap` function with signature
    /// `gaugeMap(address)` and selector `0xcb0d5b52`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "gaugeMap", abi = "gaugeMap(address)")]
    pub struct GaugeMapCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `isShutdown` function with signature
    /// `isShutdown()` and selector `0xbf86d690`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "isShutdown", abi = "isShutdown()")]
    pub struct IsShutdownCall;
    ///Container type for all input parameters for the `lockFees` function with signature
    /// `lockFees()` and selector `0xab366292`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "lockFees", abi = "lockFees()")]
    pub struct LockFeesCall;
    ///Container type for all input parameters for the `lockIncentive` function with signature
    /// `lockIncentive()` and selector `0x50940618`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "lockIncentive", abi = "lockIncentive()")]
    pub struct LockIncentiveCall;
    ///Container type for all input parameters for the `lockRewards` function with signature
    /// `lockRewards()` and selector `0x376d771a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "lockRewards", abi = "lockRewards()")]
    pub struct LockRewardsCall;
    ///Container type for all input parameters for the `minter` function with signature `minter()`
    /// and selector `0x07546172`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "minter", abi = "minter()")]
    pub struct MinterCall;
    ///Container type for all input parameters for the `owner` function with signature `owner()`
    /// and selector `0x8da5cb5b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    ///Container type for all input parameters for the `platformFee` function with signature
    /// `platformFee()` and selector `0x26232a2e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "platformFee", abi = "platformFee()")]
    pub struct PlatformFeeCall;
    ///Container type for all input parameters for the `poolInfo` function with signature
    /// `poolInfo(uint256)` and selector `0x1526fe27`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "poolInfo", abi = "poolInfo(uint256)")]
    pub struct PoolInfoCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `poolLength` function with signature
    /// `poolLength()` and selector `0x081e3eda`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "poolLength", abi = "poolLength()")]
    pub struct PoolLengthCall;
    ///Container type for all input parameters for the `poolManager` function with signature
    /// `poolManager()` and selector `0xdc4c90d3`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "poolManager", abi = "poolManager()")]
    pub struct PoolManagerCall;
    ///Container type for all input parameters for the `registry` function with signature
    /// `registry()` and selector `0x7b103999`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "registry", abi = "registry()")]
    pub struct RegistryCall;
    ///Container type for all input parameters for the `rewardArbitrator` function with signature
    /// `rewardArbitrator()` and selector `0x043b684a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "rewardArbitrator", abi = "rewardArbitrator()")]
    pub struct RewardArbitratorCall;
    ///Container type for all input parameters for the `rewardClaimed` function with signature
    /// `rewardClaimed(uint256,address,uint256)` and selector `0x71192b17`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "rewardClaimed", abi = "rewardClaimed(uint256,address,uint256)")]
    pub struct RewardClaimedCall {
        pub pid: ::ethers::core::types::U256,
        pub address: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `rewardFactory` function with signature
    /// `rewardFactory()` and selector `0x245e4bf0`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "rewardFactory", abi = "rewardFactory()")]
    pub struct RewardFactoryCall;
    ///Container type for all input parameters for the `setArbitrator` function with signature
    /// `setArbitrator(address)` and selector `0xb0eefabe`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "setArbitrator", abi = "setArbitrator(address)")]
    pub struct SetArbitratorCall {
        pub arb: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setFactories` function with signature
    /// `setFactories(address,address,address)` and selector `0x7bd3b995`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "setFactories", abi = "setFactories(address,address,address)")]
    pub struct SetFactoriesCall {
        pub rfactory: ::ethers::core::types::Address,
        pub sfactory: ::ethers::core::types::Address,
        pub tfactory: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setFeeInfo` function with signature
    /// `setFeeInfo()` and selector `0x5a4ae5ca`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "setFeeInfo", abi = "setFeeInfo()")]
    pub struct SetFeeInfoCall;
    ///Container type for all input parameters for the `setFeeManager` function with signature
    /// `setFeeManager(address)` and selector `0x472d35b9`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "setFeeManager", abi = "setFeeManager(address)")]
    pub struct SetFeeManagerCall {
        pub fee_m: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setFees` function with signature
    /// `setFees(uint256,uint256,uint256,uint256)` and selector `0x6fcba377`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "setFees", abi = "setFees(uint256,uint256,uint256,uint256)")]
    pub struct SetFeesCall {
        pub lock_fees: ::ethers::core::types::U256,
        pub staker_fees: ::ethers::core::types::U256,
        pub caller_fees: ::ethers::core::types::U256,
        pub platform: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setGaugeRedirect` function with signature
    /// `setGaugeRedirect(uint256)` and selector `0x9123d404`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "setGaugeRedirect", abi = "setGaugeRedirect(uint256)")]
    pub struct SetGaugeRedirectCall {
        pub pid: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setOwner` function with signature
    /// `setOwner(address)` and selector `0x13af4035`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "setOwner", abi = "setOwner(address)")]
    pub struct SetOwnerCall {
        pub owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setPoolManager` function with signature
    /// `setPoolManager(address)` and selector `0x7aef6715`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "setPoolManager", abi = "setPoolManager(address)")]
    pub struct SetPoolManagerCall {
        pub pool_m: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setRewardContracts` function with signature
    /// `setRewardContracts(address,address)` and selector `0x95539a1d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "setRewardContracts", abi = "setRewardContracts(address,address)")]
    pub struct SetRewardContractsCall {
        pub rewards: ::ethers::core::types::Address,
        pub staker_rewards: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setTreasury` function with signature
    /// `setTreasury(address)` and selector `0xf0f44260`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "setTreasury", abi = "setTreasury(address)")]
    pub struct SetTreasuryCall {
        pub treasury: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setVoteDelegate` function with signature
    /// `setVoteDelegate(address)` and selector `0x74874323`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "setVoteDelegate", abi = "setVoteDelegate(address)")]
    pub struct SetVoteDelegateCall {
        pub vote_delegate: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `shutdownPool` function with signature
    /// `shutdownPool(uint256)` and selector `0x60cafe84`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "shutdownPool", abi = "shutdownPool(uint256)")]
    pub struct ShutdownPoolCall {
        pub pid: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `shutdownSystem` function with signature
    /// `shutdownSystem()` and selector `0x354af919`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "shutdownSystem", abi = "shutdownSystem()")]
    pub struct ShutdownSystemCall;
    ///Container type for all input parameters for the `staker` function with signature `staker()`
    /// and selector `0x5ebaf1db`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "staker", abi = "staker()")]
    pub struct StakerCall;
    ///Container type for all input parameters for the `stakerIncentive` function with signature
    /// `stakerIncentive()` and selector `0x62d28ac7`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "stakerIncentive", abi = "stakerIncentive()")]
    pub struct StakerIncentiveCall;
    ///Container type for all input parameters for the `stakerRewards` function with signature
    /// `stakerRewards()` and selector `0xcfb9cfba`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "stakerRewards", abi = "stakerRewards()")]
    pub struct StakerRewardsCall;
    ///Container type for all input parameters for the `stashFactory` function with signature
    /// `stashFactory()` and selector `0x068eb19e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "stashFactory", abi = "stashFactory()")]
    pub struct StashFactoryCall;
    ///Container type for all input parameters for the `tokenFactory` function with signature
    /// `tokenFactory()` and selector `0xe77772fe`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "tokenFactory", abi = "tokenFactory()")]
    pub struct TokenFactoryCall;
    ///Container type for all input parameters for the `treasury` function with signature
    /// `treasury()` and selector `0x61d027b3`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "treasury", abi = "treasury()")]
    pub struct TreasuryCall;
    ///Container type for all input parameters for the `vote` function with signature
    /// `vote(uint256,address,bool)` and selector `0xe2cdd42a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "vote", abi = "vote(uint256,address,bool)")]
    pub struct VoteCall {
        pub vote_id: ::ethers::core::types::U256,
        pub voting_address: ::ethers::core::types::Address,
        pub support: bool,
    }
    ///Container type for all input parameters for the `voteDelegate` function with signature
    /// `voteDelegate()` and selector `0x9f00332b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "voteDelegate", abi = "voteDelegate()")]
    pub struct VoteDelegateCall;
    ///Container type for all input parameters for the `voteGaugeWeight` function with signature
    /// `voteGaugeWeight(address[],uint256[])` and selector `0xbfad96ba`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "voteGaugeWeight", abi = "voteGaugeWeight(address[],uint256[])")]
    pub struct VoteGaugeWeightCall {
        pub gauge: ::std::vec::Vec<::ethers::core::types::Address>,
        pub weight: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all input parameters for the `voteOwnership` function with signature
    /// `voteOwnership()` and selector `0xa386a080`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "voteOwnership", abi = "voteOwnership()")]
    pub struct VoteOwnershipCall;
    ///Container type for all input parameters for the `voteParameter` function with signature
    /// `voteParameter()` and selector `0xb42eda71`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "voteParameter", abi = "voteParameter()")]
    pub struct VoteParameterCall;
    ///Container type for all input parameters for the `withdraw` function with signature
    /// `withdraw(uint256,uint256)` and selector `0x441a3e70`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "withdraw", abi = "withdraw(uint256,uint256)")]
    pub struct WithdrawCall {
        pub pid: ::ethers::core::types::U256,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `withdrawAll` function with signature
    /// `withdrawAll(uint256)` and selector `0x958e2d31`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "withdrawAll", abi = "withdrawAll(uint256)")]
    pub struct WithdrawAllCall {
        pub pid: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `withdrawTo` function with signature
    /// `withdrawTo(uint256,uint256,address)` and selector `0x14cd70e4`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "withdrawTo", abi = "withdrawTo(uint256,uint256,address)")]
    pub struct WithdrawToCall {
        pub pid: ::ethers::core::types::U256,
        pub amount: ::ethers::core::types::U256,
        pub to: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum convex_modCalls {
        FeeDenominator(FeeDenominatorCall),
        MaxFees(MaxFeesCall),
        AddPool(AddPoolCall),
        ClaimRewards(ClaimRewardsCall),
        Crv(CrvCall),
        Deposit(DepositCall),
        DepositAll(DepositAllCall),
        DistributionAddressId(DistributionAddressIdCall),
        EarmarkFees(EarmarkFeesCall),
        EarmarkIncentive(EarmarkIncentiveCall),
        EarmarkRewards(EarmarkRewardsCall),
        FeeDistro(FeeDistroCall),
        FeeManager(FeeManagerCall),
        FeeToken(FeeTokenCall),
        GaugeMap(GaugeMapCall),
        IsShutdown(IsShutdownCall),
        LockFees(LockFeesCall),
        LockIncentive(LockIncentiveCall),
        LockRewards(LockRewardsCall),
        Minter(MinterCall),
        Owner(OwnerCall),
        PlatformFee(PlatformFeeCall),
        PoolInfo(PoolInfoCall),
        PoolLength(PoolLengthCall),
        PoolManager(PoolManagerCall),
        Registry(RegistryCall),
        RewardArbitrator(RewardArbitratorCall),
        RewardClaimed(RewardClaimedCall),
        RewardFactory(RewardFactoryCall),
        SetArbitrator(SetArbitratorCall),
        SetFactories(SetFactoriesCall),
        SetFeeInfo(SetFeeInfoCall),
        SetFeeManager(SetFeeManagerCall),
        SetFees(SetFeesCall),
        SetGaugeRedirect(SetGaugeRedirectCall),
        SetOwner(SetOwnerCall),
        SetPoolManager(SetPoolManagerCall),
        SetRewardContracts(SetRewardContractsCall),
        SetTreasury(SetTreasuryCall),
        SetVoteDelegate(SetVoteDelegateCall),
        ShutdownPool(ShutdownPoolCall),
        ShutdownSystem(ShutdownSystemCall),
        Staker(StakerCall),
        StakerIncentive(StakerIncentiveCall),
        StakerRewards(StakerRewardsCall),
        StashFactory(StashFactoryCall),
        TokenFactory(TokenFactoryCall),
        Treasury(TreasuryCall),
        Vote(VoteCall),
        VoteDelegate(VoteDelegateCall),
        VoteGaugeWeight(VoteGaugeWeightCall),
        VoteOwnership(VoteOwnershipCall),
        VoteParameter(VoteParameterCall),
        Withdraw(WithdrawCall),
        WithdrawAll(WithdrawAllCall),
        WithdrawTo(WithdrawToCall),
    }
    impl ::ethers::core::abi::AbiDecode for convex_modCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <FeeDenominatorCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::FeeDenominator(decoded))
            }
            if let Ok(decoded) = <MaxFeesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MaxFees(decoded))
            }
            if let Ok(decoded) = <AddPoolCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AddPool(decoded))
            }
            if let Ok(decoded) = <ClaimRewardsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ClaimRewards(decoded))
            }
            if let Ok(decoded) = <CrvCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Crv(decoded))
            }
            if let Ok(decoded) = <DepositCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Deposit(decoded))
            }
            if let Ok(decoded) = <DepositAllCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DepositAll(decoded))
            }
            if let Ok(decoded) =
                <DistributionAddressIdCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DistributionAddressId(decoded))
            }
            if let Ok(decoded) = <EarmarkFeesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::EarmarkFees(decoded))
            }
            if let Ok(decoded) =
                <EarmarkIncentiveCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::EarmarkIncentive(decoded))
            }
            if let Ok(decoded) =
                <EarmarkRewardsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::EarmarkRewards(decoded))
            }
            if let Ok(decoded) = <FeeDistroCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::FeeDistro(decoded))
            }
            if let Ok(decoded) = <FeeManagerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::FeeManager(decoded))
            }
            if let Ok(decoded) = <FeeTokenCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::FeeToken(decoded))
            }
            if let Ok(decoded) = <GaugeMapCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GaugeMap(decoded))
            }
            if let Ok(decoded) = <IsShutdownCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsShutdown(decoded))
            }
            if let Ok(decoded) = <LockFeesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LockFees(decoded))
            }
            if let Ok(decoded) = <LockIncentiveCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::LockIncentive(decoded))
            }
            if let Ok(decoded) = <LockRewardsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LockRewards(decoded))
            }
            if let Ok(decoded) = <MinterCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Minter(decoded))
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Owner(decoded))
            }
            if let Ok(decoded) = <PlatformFeeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PlatformFee(decoded))
            }
            if let Ok(decoded) = <PoolInfoCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PoolInfo(decoded))
            }
            if let Ok(decoded) = <PoolLengthCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PoolLength(decoded))
            }
            if let Ok(decoded) = <PoolManagerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PoolManager(decoded))
            }
            if let Ok(decoded) = <RegistryCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Registry(decoded))
            }
            if let Ok(decoded) =
                <RewardArbitratorCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RewardArbitrator(decoded))
            }
            if let Ok(decoded) = <RewardClaimedCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RewardClaimed(decoded))
            }
            if let Ok(decoded) = <RewardFactoryCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RewardFactory(decoded))
            }
            if let Ok(decoded) = <SetArbitratorCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetArbitrator(decoded))
            }
            if let Ok(decoded) = <SetFactoriesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetFactories(decoded))
            }
            if let Ok(decoded) = <SetFeeInfoCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetFeeInfo(decoded))
            }
            if let Ok(decoded) = <SetFeeManagerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetFeeManager(decoded))
            }
            if let Ok(decoded) = <SetFeesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetFees(decoded))
            }
            if let Ok(decoded) =
                <SetGaugeRedirectCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetGaugeRedirect(decoded))
            }
            if let Ok(decoded) = <SetOwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetOwner(decoded))
            }
            if let Ok(decoded) =
                <SetPoolManagerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetPoolManager(decoded))
            }
            if let Ok(decoded) =
                <SetRewardContractsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetRewardContracts(decoded))
            }
            if let Ok(decoded) = <SetTreasuryCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetTreasury(decoded))
            }
            if let Ok(decoded) =
                <SetVoteDelegateCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetVoteDelegate(decoded))
            }
            if let Ok(decoded) = <ShutdownPoolCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ShutdownPool(decoded))
            }
            if let Ok(decoded) =
                <ShutdownSystemCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ShutdownSystem(decoded))
            }
            if let Ok(decoded) = <StakerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Staker(decoded))
            }
            if let Ok(decoded) =
                <StakerIncentiveCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::StakerIncentive(decoded))
            }
            if let Ok(decoded) = <StakerRewardsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::StakerRewards(decoded))
            }
            if let Ok(decoded) = <StashFactoryCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::StashFactory(decoded))
            }
            if let Ok(decoded) = <TokenFactoryCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TokenFactory(decoded))
            }
            if let Ok(decoded) = <TreasuryCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Treasury(decoded))
            }
            if let Ok(decoded) = <VoteCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Vote(decoded))
            }
            if let Ok(decoded) = <VoteDelegateCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::VoteDelegate(decoded))
            }
            if let Ok(decoded) =
                <VoteGaugeWeightCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::VoteGaugeWeight(decoded))
            }
            if let Ok(decoded) = <VoteOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::VoteOwnership(decoded))
            }
            if let Ok(decoded) = <VoteParameterCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::VoteParameter(decoded))
            }
            if let Ok(decoded) = <WithdrawCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Withdraw(decoded))
            }
            if let Ok(decoded) = <WithdrawAllCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::WithdrawAll(decoded))
            }
            if let Ok(decoded) = <WithdrawToCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::WithdrawTo(decoded))
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for convex_modCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::FeeDenominator(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MaxFees(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AddPool(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ClaimRewards(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Crv(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Deposit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DepositAll(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DistributionAddressId(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EarmarkFees(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::EarmarkIncentive(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::EarmarkRewards(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::FeeDistro(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::FeeManager(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::FeeToken(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GaugeMap(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IsShutdown(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LockFees(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LockIncentive(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LockRewards(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Minter(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PlatformFee(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PoolInfo(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PoolLength(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PoolManager(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Registry(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RewardArbitrator(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RewardClaimed(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RewardFactory(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetArbitrator(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetFactories(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetFeeInfo(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetFeeManager(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetFees(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetGaugeRedirect(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetOwner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetPoolManager(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetRewardContracts(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetTreasury(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetVoteDelegate(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ShutdownPool(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ShutdownSystem(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Staker(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::StakerIncentive(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::StakerRewards(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::StashFactory(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TokenFactory(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Treasury(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Vote(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::VoteDelegate(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::VoteGaugeWeight(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::VoteOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::VoteParameter(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Withdraw(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::WithdrawAll(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::WithdrawTo(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for convex_modCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::FeeDenominator(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxFees(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddPool(element) => ::core::fmt::Display::fmt(element, f),
                Self::ClaimRewards(element) => ::core::fmt::Display::fmt(element, f),
                Self::Crv(element) => ::core::fmt::Display::fmt(element, f),
                Self::Deposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::DepositAll(element) => ::core::fmt::Display::fmt(element, f),
                Self::DistributionAddressId(element) => ::core::fmt::Display::fmt(element, f),
                Self::EarmarkFees(element) => ::core::fmt::Display::fmt(element, f),
                Self::EarmarkIncentive(element) => ::core::fmt::Display::fmt(element, f),
                Self::EarmarkRewards(element) => ::core::fmt::Display::fmt(element, f),
                Self::FeeDistro(element) => ::core::fmt::Display::fmt(element, f),
                Self::FeeManager(element) => ::core::fmt::Display::fmt(element, f),
                Self::FeeToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::GaugeMap(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsShutdown(element) => ::core::fmt::Display::fmt(element, f),
                Self::LockFees(element) => ::core::fmt::Display::fmt(element, f),
                Self::LockIncentive(element) => ::core::fmt::Display::fmt(element, f),
                Self::LockRewards(element) => ::core::fmt::Display::fmt(element, f),
                Self::Minter(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::PlatformFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::PoolInfo(element) => ::core::fmt::Display::fmt(element, f),
                Self::PoolLength(element) => ::core::fmt::Display::fmt(element, f),
                Self::PoolManager(element) => ::core::fmt::Display::fmt(element, f),
                Self::Registry(element) => ::core::fmt::Display::fmt(element, f),
                Self::RewardArbitrator(element) => ::core::fmt::Display::fmt(element, f),
                Self::RewardClaimed(element) => ::core::fmt::Display::fmt(element, f),
                Self::RewardFactory(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetArbitrator(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetFactories(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetFeeInfo(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetFeeManager(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetFees(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetGaugeRedirect(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetOwner(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetPoolManager(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetRewardContracts(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetTreasury(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetVoteDelegate(element) => ::core::fmt::Display::fmt(element, f),
                Self::ShutdownPool(element) => ::core::fmt::Display::fmt(element, f),
                Self::ShutdownSystem(element) => ::core::fmt::Display::fmt(element, f),
                Self::Staker(element) => ::core::fmt::Display::fmt(element, f),
                Self::StakerIncentive(element) => ::core::fmt::Display::fmt(element, f),
                Self::StakerRewards(element) => ::core::fmt::Display::fmt(element, f),
                Self::StashFactory(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenFactory(element) => ::core::fmt::Display::fmt(element, f),
                Self::Treasury(element) => ::core::fmt::Display::fmt(element, f),
                Self::Vote(element) => ::core::fmt::Display::fmt(element, f),
                Self::VoteDelegate(element) => ::core::fmt::Display::fmt(element, f),
                Self::VoteGaugeWeight(element) => ::core::fmt::Display::fmt(element, f),
                Self::VoteOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::VoteParameter(element) => ::core::fmt::Display::fmt(element, f),
                Self::Withdraw(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawAll(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawTo(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<FeeDenominatorCall> for convex_modCalls {
        fn from(value: FeeDenominatorCall) -> Self {
            Self::FeeDenominator(value)
        }
    }
    impl ::core::convert::From<MaxFeesCall> for convex_modCalls {
        fn from(value: MaxFeesCall) -> Self {
            Self::MaxFees(value)
        }
    }
    impl ::core::convert::From<AddPoolCall> for convex_modCalls {
        fn from(value: AddPoolCall) -> Self {
            Self::AddPool(value)
        }
    }
    impl ::core::convert::From<ClaimRewardsCall> for convex_modCalls {
        fn from(value: ClaimRewardsCall) -> Self {
            Self::ClaimRewards(value)
        }
    }
    impl ::core::convert::From<CrvCall> for convex_modCalls {
        fn from(value: CrvCall) -> Self {
            Self::Crv(value)
        }
    }
    impl ::core::convert::From<DepositCall> for convex_modCalls {
        fn from(value: DepositCall) -> Self {
            Self::Deposit(value)
        }
    }
    impl ::core::convert::From<DepositAllCall> for convex_modCalls {
        fn from(value: DepositAllCall) -> Self {
            Self::DepositAll(value)
        }
    }
    impl ::core::convert::From<DistributionAddressIdCall> for convex_modCalls {
        fn from(value: DistributionAddressIdCall) -> Self {
            Self::DistributionAddressId(value)
        }
    }
    impl ::core::convert::From<EarmarkFeesCall> for convex_modCalls {
        fn from(value: EarmarkFeesCall) -> Self {
            Self::EarmarkFees(value)
        }
    }
    impl ::core::convert::From<EarmarkIncentiveCall> for convex_modCalls {
        fn from(value: EarmarkIncentiveCall) -> Self {
            Self::EarmarkIncentive(value)
        }
    }
    impl ::core::convert::From<EarmarkRewardsCall> for convex_modCalls {
        fn from(value: EarmarkRewardsCall) -> Self {
            Self::EarmarkRewards(value)
        }
    }
    impl ::core::convert::From<FeeDistroCall> for convex_modCalls {
        fn from(value: FeeDistroCall) -> Self {
            Self::FeeDistro(value)
        }
    }
    impl ::core::convert::From<FeeManagerCall> for convex_modCalls {
        fn from(value: FeeManagerCall) -> Self {
            Self::FeeManager(value)
        }
    }
    impl ::core::convert::From<FeeTokenCall> for convex_modCalls {
        fn from(value: FeeTokenCall) -> Self {
            Self::FeeToken(value)
        }
    }
    impl ::core::convert::From<GaugeMapCall> for convex_modCalls {
        fn from(value: GaugeMapCall) -> Self {
            Self::GaugeMap(value)
        }
    }
    impl ::core::convert::From<IsShutdownCall> for convex_modCalls {
        fn from(value: IsShutdownCall) -> Self {
            Self::IsShutdown(value)
        }
    }
    impl ::core::convert::From<LockFeesCall> for convex_modCalls {
        fn from(value: LockFeesCall) -> Self {
            Self::LockFees(value)
        }
    }
    impl ::core::convert::From<LockIncentiveCall> for convex_modCalls {
        fn from(value: LockIncentiveCall) -> Self {
            Self::LockIncentive(value)
        }
    }
    impl ::core::convert::From<LockRewardsCall> for convex_modCalls {
        fn from(value: LockRewardsCall) -> Self {
            Self::LockRewards(value)
        }
    }
    impl ::core::convert::From<MinterCall> for convex_modCalls {
        fn from(value: MinterCall) -> Self {
            Self::Minter(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for convex_modCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<PlatformFeeCall> for convex_modCalls {
        fn from(value: PlatformFeeCall) -> Self {
            Self::PlatformFee(value)
        }
    }
    impl ::core::convert::From<PoolInfoCall> for convex_modCalls {
        fn from(value: PoolInfoCall) -> Self {
            Self::PoolInfo(value)
        }
    }
    impl ::core::convert::From<PoolLengthCall> for convex_modCalls {
        fn from(value: PoolLengthCall) -> Self {
            Self::PoolLength(value)
        }
    }
    impl ::core::convert::From<PoolManagerCall> for convex_modCalls {
        fn from(value: PoolManagerCall) -> Self {
            Self::PoolManager(value)
        }
    }
    impl ::core::convert::From<RegistryCall> for convex_modCalls {
        fn from(value: RegistryCall) -> Self {
            Self::Registry(value)
        }
    }
    impl ::core::convert::From<RewardArbitratorCall> for convex_modCalls {
        fn from(value: RewardArbitratorCall) -> Self {
            Self::RewardArbitrator(value)
        }
    }
    impl ::core::convert::From<RewardClaimedCall> for convex_modCalls {
        fn from(value: RewardClaimedCall) -> Self {
            Self::RewardClaimed(value)
        }
    }
    impl ::core::convert::From<RewardFactoryCall> for convex_modCalls {
        fn from(value: RewardFactoryCall) -> Self {
            Self::RewardFactory(value)
        }
    }
    impl ::core::convert::From<SetArbitratorCall> for convex_modCalls {
        fn from(value: SetArbitratorCall) -> Self {
            Self::SetArbitrator(value)
        }
    }
    impl ::core::convert::From<SetFactoriesCall> for convex_modCalls {
        fn from(value: SetFactoriesCall) -> Self {
            Self::SetFactories(value)
        }
    }
    impl ::core::convert::From<SetFeeInfoCall> for convex_modCalls {
        fn from(value: SetFeeInfoCall) -> Self {
            Self::SetFeeInfo(value)
        }
    }
    impl ::core::convert::From<SetFeeManagerCall> for convex_modCalls {
        fn from(value: SetFeeManagerCall) -> Self {
            Self::SetFeeManager(value)
        }
    }
    impl ::core::convert::From<SetFeesCall> for convex_modCalls {
        fn from(value: SetFeesCall) -> Self {
            Self::SetFees(value)
        }
    }
    impl ::core::convert::From<SetGaugeRedirectCall> for convex_modCalls {
        fn from(value: SetGaugeRedirectCall) -> Self {
            Self::SetGaugeRedirect(value)
        }
    }
    impl ::core::convert::From<SetOwnerCall> for convex_modCalls {
        fn from(value: SetOwnerCall) -> Self {
            Self::SetOwner(value)
        }
    }
    impl ::core::convert::From<SetPoolManagerCall> for convex_modCalls {
        fn from(value: SetPoolManagerCall) -> Self {
            Self::SetPoolManager(value)
        }
    }
    impl ::core::convert::From<SetRewardContractsCall> for convex_modCalls {
        fn from(value: SetRewardContractsCall) -> Self {
            Self::SetRewardContracts(value)
        }
    }
    impl ::core::convert::From<SetTreasuryCall> for convex_modCalls {
        fn from(value: SetTreasuryCall) -> Self {
            Self::SetTreasury(value)
        }
    }
    impl ::core::convert::From<SetVoteDelegateCall> for convex_modCalls {
        fn from(value: SetVoteDelegateCall) -> Self {
            Self::SetVoteDelegate(value)
        }
    }
    impl ::core::convert::From<ShutdownPoolCall> for convex_modCalls {
        fn from(value: ShutdownPoolCall) -> Self {
            Self::ShutdownPool(value)
        }
    }
    impl ::core::convert::From<ShutdownSystemCall> for convex_modCalls {
        fn from(value: ShutdownSystemCall) -> Self {
            Self::ShutdownSystem(value)
        }
    }
    impl ::core::convert::From<StakerCall> for convex_modCalls {
        fn from(value: StakerCall) -> Self {
            Self::Staker(value)
        }
    }
    impl ::core::convert::From<StakerIncentiveCall> for convex_modCalls {
        fn from(value: StakerIncentiveCall) -> Self {
            Self::StakerIncentive(value)
        }
    }
    impl ::core::convert::From<StakerRewardsCall> for convex_modCalls {
        fn from(value: StakerRewardsCall) -> Self {
            Self::StakerRewards(value)
        }
    }
    impl ::core::convert::From<StashFactoryCall> for convex_modCalls {
        fn from(value: StashFactoryCall) -> Self {
            Self::StashFactory(value)
        }
    }
    impl ::core::convert::From<TokenFactoryCall> for convex_modCalls {
        fn from(value: TokenFactoryCall) -> Self {
            Self::TokenFactory(value)
        }
    }
    impl ::core::convert::From<TreasuryCall> for convex_modCalls {
        fn from(value: TreasuryCall) -> Self {
            Self::Treasury(value)
        }
    }
    impl ::core::convert::From<VoteCall> for convex_modCalls {
        fn from(value: VoteCall) -> Self {
            Self::Vote(value)
        }
    }
    impl ::core::convert::From<VoteDelegateCall> for convex_modCalls {
        fn from(value: VoteDelegateCall) -> Self {
            Self::VoteDelegate(value)
        }
    }
    impl ::core::convert::From<VoteGaugeWeightCall> for convex_modCalls {
        fn from(value: VoteGaugeWeightCall) -> Self {
            Self::VoteGaugeWeight(value)
        }
    }
    impl ::core::convert::From<VoteOwnershipCall> for convex_modCalls {
        fn from(value: VoteOwnershipCall) -> Self {
            Self::VoteOwnership(value)
        }
    }
    impl ::core::convert::From<VoteParameterCall> for convex_modCalls {
        fn from(value: VoteParameterCall) -> Self {
            Self::VoteParameter(value)
        }
    }
    impl ::core::convert::From<WithdrawCall> for convex_modCalls {
        fn from(value: WithdrawCall) -> Self {
            Self::Withdraw(value)
        }
    }
    impl ::core::convert::From<WithdrawAllCall> for convex_modCalls {
        fn from(value: WithdrawAllCall) -> Self {
            Self::WithdrawAll(value)
        }
    }
    impl ::core::convert::From<WithdrawToCall> for convex_modCalls {
        fn from(value: WithdrawToCall) -> Self {
            Self::WithdrawTo(value)
        }
    }
    ///Container type for all return fields from the `FEE_DENOMINATOR` function with signature
    /// `FEE_DENOMINATOR()` and selector `0xd73792a9`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct FeeDenominatorReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `MaxFees` function with signature `MaxFees()`
    /// and selector `0x7303df9a`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct MaxFeesReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `addPool` function with signature
    /// `addPool(address,address,uint256)` and selector `0x7e29d6c2`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct AddPoolReturn(pub bool);
    ///Container type for all return fields from the `claimRewards` function with signature
    /// `claimRewards(uint256,address)` and selector `0x6c7b69cb`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct ClaimRewardsReturn(pub bool);
    ///Container type for all return fields from the `crv` function with signature `crv()` and
    /// selector `0x6a4874a1`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct CrvReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `deposit` function with signature
    /// `deposit(uint256,uint256,bool)` and selector `0x43a0d066`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct DepositReturn(pub bool);
    ///Container type for all return fields from the `depositAll` function with signature
    /// `depositAll(uint256,bool)` and selector `0x60759fce`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct DepositAllReturn(pub bool);
    ///Container type for all return fields from the `distributionAddressId` function with
    /// signature `distributionAddressId()` and selector `0x93e846a0`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct DistributionAddressIdReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `earmarkFees` function with signature
    /// `earmarkFees()` and selector `0x22230b96`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct EarmarkFeesReturn(pub bool);
    ///Container type for all return fields from the `earmarkIncentive` function with signature
    /// `earmarkIncentive()` and selector `0x3a088cd2`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct EarmarkIncentiveReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `earmarkRewards` function with signature
    /// `earmarkRewards(uint256)` and selector `0xcc956f3f`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct EarmarkRewardsReturn(pub bool);
    ///Container type for all return fields from the `feeDistro` function with signature
    /// `feeDistro()` and selector `0xd6a0f530`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct FeeDistroReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `feeManager` function with signature
    /// `feeManager()` and selector `0xd0fb0203`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct FeeManagerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `feeToken` function with signature
    /// `feeToken()` and selector `0x647846a5`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct FeeTokenReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `gaugeMap` function with signature
    /// `gaugeMap(address)` and selector `0xcb0d5b52`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GaugeMapReturn(pub bool);
    ///Container type for all return fields from the `isShutdown` function with signature
    /// `isShutdown()` and selector `0xbf86d690`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct IsShutdownReturn(pub bool);
    ///Container type for all return fields from the `lockFees` function with signature
    /// `lockFees()` and selector `0xab366292`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct LockFeesReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `lockIncentive` function with signature
    /// `lockIncentive()` and selector `0x50940618`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct LockIncentiveReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `lockRewards` function with signature
    /// `lockRewards()` and selector `0x376d771a`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct LockRewardsReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `minter` function with signature `minter()`
    /// and selector `0x07546172`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct MinterReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `owner` function with signature `owner()` and
    /// selector `0x8da5cb5b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `platformFee` function with signature
    /// `platformFee()` and selector `0x26232a2e`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct PlatformFeeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `poolInfo` function with signature
    /// `poolInfo(uint256)` and selector `0x1526fe27`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct PoolInfoReturn {
        pub lptoken: ::ethers::core::types::Address,
        pub token: ::ethers::core::types::Address,
        pub gauge: ::ethers::core::types::Address,
        pub crv_rewards: ::ethers::core::types::Address,
        pub stash: ::ethers::core::types::Address,
        pub shutdown: bool,
    }
    ///Container type for all return fields from the `poolLength` function with signature
    /// `poolLength()` and selector `0x081e3eda`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct PoolLengthReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `poolManager` function with signature
    /// `poolManager()` and selector `0xdc4c90d3`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct PoolManagerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `registry` function with signature
    /// `registry()` and selector `0x7b103999`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct RegistryReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `rewardArbitrator` function with signature
    /// `rewardArbitrator()` and selector `0x043b684a`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct RewardArbitratorReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `rewardClaimed` function with signature
    /// `rewardClaimed(uint256,address,uint256)` and selector `0x71192b17`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct RewardClaimedReturn(pub bool);
    ///Container type for all return fields from the `rewardFactory` function with signature
    /// `rewardFactory()` and selector `0x245e4bf0`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct RewardFactoryReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `setGaugeRedirect` function with signature
    /// `setGaugeRedirect(uint256)` and selector `0x9123d404`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct SetGaugeRedirectReturn(pub bool);
    ///Container type for all return fields from the `shutdownPool` function with signature
    /// `shutdownPool(uint256)` and selector `0x60cafe84`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct ShutdownPoolReturn(pub bool);
    ///Container type for all return fields from the `staker` function with signature `staker()`
    /// and selector `0x5ebaf1db`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct StakerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `stakerIncentive` function with signature
    /// `stakerIncentive()` and selector `0x62d28ac7`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct StakerIncentiveReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `stakerRewards` function with signature
    /// `stakerRewards()` and selector `0xcfb9cfba`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct StakerRewardsReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `stashFactory` function with signature
    /// `stashFactory()` and selector `0x068eb19e`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct StashFactoryReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `tokenFactory` function with signature
    /// `tokenFactory()` and selector `0xe77772fe`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct TokenFactoryReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `treasury` function with signature
    /// `treasury()` and selector `0x61d027b3`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct TreasuryReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `vote` function with signature
    /// `vote(uint256,address,bool)` and selector `0xe2cdd42a`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct VoteReturn(pub bool);
    ///Container type for all return fields from the `voteDelegate` function with signature
    /// `voteDelegate()` and selector `0x9f00332b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct VoteDelegateReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `voteGaugeWeight` function with signature
    /// `voteGaugeWeight(address[],uint256[])` and selector `0xbfad96ba`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct VoteGaugeWeightReturn(pub bool);
    ///Container type for all return fields from the `voteOwnership` function with signature
    /// `voteOwnership()` and selector `0xa386a080`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct VoteOwnershipReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `voteParameter` function with signature
    /// `voteParameter()` and selector `0xb42eda71`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct VoteParameterReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `withdraw` function with signature
    /// `withdraw(uint256,uint256)` and selector `0x441a3e70`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct WithdrawReturn(pub bool);
    ///Container type for all return fields from the `withdrawAll` function with signature
    /// `withdrawAll(uint256)` and selector `0x958e2d31`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct WithdrawAllReturn(pub bool);
    ///Container type for all return fields from the `withdrawTo` function with signature
    /// `withdrawTo(uint256,uint256,address)` and selector `0x14cd70e4`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct WithdrawToReturn(pub bool);
}
