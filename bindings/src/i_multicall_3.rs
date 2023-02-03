pub use i_multicall_3::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod i_multicall_3 {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    ///IMulticall3 was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs
    use std::sync::Arc;
    use ::ethers::core::{
        abi::{Abi, Token, Detokenize, InvalidOutputType, Tokenizable},
        types::*,
    };
    use ::ethers::contract::{
        Contract, builders::{ContractCall, Event},
        Lazy,
    };
    use ::ethers::providers::Middleware;
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"struct IMulticall3.Call[]\",\"name\":\"calls\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"target\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"callData\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"aggregate\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"blockNumber\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes[]\",\"name\":\"returnData\",\"type\":\"bytes[]\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct IMulticall3.Call3[]\",\"name\":\"calls\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"target\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"allowFailure\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"callData\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"aggregate3\",\"outputs\":[{\"internalType\":\"struct IMulticall3.Result[]\",\"name\":\"returnData\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"bool\",\"name\":\"success\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"returnData\",\"type\":\"bytes\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"struct IMulticall3.Call3Value[]\",\"name\":\"calls\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"target\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"allowFailure\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"callData\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"aggregate3Value\",\"outputs\":[{\"internalType\":\"struct IMulticall3.Result[]\",\"name\":\"returnData\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"bool\",\"name\":\"success\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"returnData\",\"type\":\"bytes\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"struct IMulticall3.Call[]\",\"name\":\"calls\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"target\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"callData\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"blockAndAggregate\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"blockNumber\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"blockHash\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"struct IMulticall3.Result[]\",\"name\":\"returnData\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"bool\",\"name\":\"success\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"returnData\",\"type\":\"bytes\",\"components\":[]}]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getBasefee\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"basefee\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"blockNumber\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getBlockHash\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"blockHash\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getBlockNumber\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"blockNumber\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getChainId\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"chainid\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getCurrentBlockCoinbase\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"coinbase\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getCurrentBlockDifficulty\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"difficulty\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getCurrentBlockGasLimit\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"gaslimit\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getCurrentBlockTimestamp\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"timestamp\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"addr\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getEthBalance\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"balance\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getLastBlockHash\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"blockHash\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"requireSuccess\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"struct IMulticall3.Call[]\",\"name\":\"calls\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"target\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"callData\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"tryAggregate\",\"outputs\":[{\"internalType\":\"struct IMulticall3.Result[]\",\"name\":\"returnData\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"bool\",\"name\":\"success\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"returnData\",\"type\":\"bytes\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"requireSuccess\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"struct IMulticall3.Call[]\",\"name\":\"calls\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"target\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"callData\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"tryBlockAndAggregate\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"blockNumber\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"blockHash\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"struct IMulticall3.Result[]\",\"name\":\"returnData\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"bool\",\"name\":\"success\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"returnData\",\"type\":\"bytes\",\"components\":[]}]}]}]";
    /// The parsed JSON-ABI of the contract.
    pub static IMULTICALL3_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(||
    ::ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi"));
    pub struct IMulticall3<M>(::ethers::contract::Contract<M>);
    impl<M> Clone for IMulticall3<M> {
        fn clone(&self) -> Self {
            IMulticall3(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for IMulticall3<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for IMulticall3<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IMulticall3)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IMulticall3<M> {
        /// Creates a new contract instance with the specified `ethers`
        /// client at the given `Address`. The contract derefs to a `ethers::Contract`
        /// object
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    IMULTICALL3_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `aggregate` (0x252dba42) function
        pub fn aggregate(
            &self,
            calls: ::std::vec::Vec<Call>,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::std::vec::Vec<::ethers::core::types::Bytes>),
        > {
            self.0
                .method_hash([37, 45, 186, 66], calls)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `aggregate3` (0x82ad56cb) function
        pub fn aggregate_3(
            &self,
            calls: ::std::vec::Vec<Call3>,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<Result>> {
            self.0
                .method_hash([130, 173, 86, 203], calls)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `aggregate3Value` (0x174dea71) function
        pub fn aggregate_3_value(
            &self,
            calls: ::std::vec::Vec<Call3Value>,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<Result>> {
            self.0
                .method_hash([23, 77, 234, 113], calls)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `blockAndAggregate` (0xc3077fa9) function
        pub fn block_and_aggregate(
            &self,
            calls: ::std::vec::Vec<Call>,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, [u8; 32], ::std::vec::Vec<Result>),
        > {
            self.0
                .method_hash([195, 7, 127, 169], calls)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getBasefee` (0x3e64a696) function
        pub fn get_basefee(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([62, 100, 166, 150], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getBlockHash` (0xee82ac5e) function
        pub fn get_block_hash(
            &self,
            block_number: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([238, 130, 172, 94], block_number)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getBlockNumber` (0x42cbb15c) function
        pub fn get_block_number(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([66, 203, 177, 92], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getChainId` (0x3408e470) function
        pub fn get_chain_id(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([52, 8, 228, 112], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getCurrentBlockCoinbase` (0xa8b0574e) function
        pub fn get_current_block_coinbase(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([168, 176, 87, 78], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getCurrentBlockDifficulty` (0x72425d9d) function
        pub fn get_current_block_difficulty(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([114, 66, 93, 157], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getCurrentBlockGasLimit` (0x86d516e8) function
        pub fn get_current_block_gas_limit(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([134, 213, 22, 232], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getCurrentBlockTimestamp` (0x0f28c97d) function
        pub fn get_current_block_timestamp(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([15, 40, 201, 125], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getEthBalance` (0x4d2301cc) function
        pub fn get_eth_balance(
            &self,
            addr: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([77, 35, 1, 204], addr)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getLastBlockHash` (0x27e86d6e) function
        pub fn get_last_block_hash(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([39, 232, 109, 110], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `tryAggregate` (0xbce38bd7) function
        pub fn try_aggregate(
            &self,
            require_success: bool,
            calls: ::std::vec::Vec<Call>,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<Result>> {
            self.0
                .method_hash([188, 227, 139, 215], (require_success, calls))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `tryBlockAndAggregate` (0x399542e9) function
        pub fn try_block_and_aggregate(
            &self,
            require_success: bool,
            calls: ::std::vec::Vec<Call>,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, [u8; 32], ::std::vec::Vec<Result>),
        > {
            self.0
                .method_hash([57, 149, 66, 233], (require_success, calls))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IMulticall3<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `aggregate` function with signature `aggregate((address,bytes)[])` and selector `0x252dba42`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "aggregate", abi = "aggregate((address,bytes)[])")]
    pub struct AggregateCall {
        pub calls: ::std::vec::Vec<Call>,
    }
    ///Container type for all input parameters for the `aggregate3` function with signature `aggregate3((address,bool,bytes)[])` and selector `0x82ad56cb`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "aggregate3", abi = "aggregate3((address,bool,bytes)[])")]
    pub struct Aggregate3Call {
        pub calls: ::std::vec::Vec<Call3>,
    }
    ///Container type for all input parameters for the `aggregate3Value` function with signature `aggregate3Value((address,bool,uint256,bytes)[])` and selector `0x174dea71`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(
        name = "aggregate3Value",
        abi = "aggregate3Value((address,bool,uint256,bytes)[])"
    )]
    pub struct Aggregate3ValueCall {
        pub calls: ::std::vec::Vec<Call3Value>,
    }
    ///Container type for all input parameters for the `blockAndAggregate` function with signature `blockAndAggregate((address,bytes)[])` and selector `0xc3077fa9`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "blockAndAggregate", abi = "blockAndAggregate((address,bytes)[])")]
    pub struct BlockAndAggregateCall {
        pub calls: ::std::vec::Vec<Call>,
    }
    ///Container type for all input parameters for the `getBasefee` function with signature `getBasefee()` and selector `0x3e64a696`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "getBasefee", abi = "getBasefee()")]
    pub struct GetBasefeeCall;
    ///Container type for all input parameters for the `getBlockHash` function with signature `getBlockHash(uint256)` and selector `0xee82ac5e`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "getBlockHash", abi = "getBlockHash(uint256)")]
    pub struct GetBlockHashCall {
        pub block_number: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getBlockNumber` function with signature `getBlockNumber()` and selector `0x42cbb15c`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "getBlockNumber", abi = "getBlockNumber()")]
    pub struct GetBlockNumberCall;
    ///Container type for all input parameters for the `getChainId` function with signature `getChainId()` and selector `0x3408e470`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "getChainId", abi = "getChainId()")]
    pub struct GetChainIdCall;
    ///Container type for all input parameters for the `getCurrentBlockCoinbase` function with signature `getCurrentBlockCoinbase()` and selector `0xa8b0574e`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "getCurrentBlockCoinbase", abi = "getCurrentBlockCoinbase()")]
    pub struct GetCurrentBlockCoinbaseCall;
    ///Container type for all input parameters for the `getCurrentBlockDifficulty` function with signature `getCurrentBlockDifficulty()` and selector `0x72425d9d`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "getCurrentBlockDifficulty", abi = "getCurrentBlockDifficulty()")]
    pub struct GetCurrentBlockDifficultyCall;
    ///Container type for all input parameters for the `getCurrentBlockGasLimit` function with signature `getCurrentBlockGasLimit()` and selector `0x86d516e8`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "getCurrentBlockGasLimit", abi = "getCurrentBlockGasLimit()")]
    pub struct GetCurrentBlockGasLimitCall;
    ///Container type for all input parameters for the `getCurrentBlockTimestamp` function with signature `getCurrentBlockTimestamp()` and selector `0x0f28c97d`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "getCurrentBlockTimestamp", abi = "getCurrentBlockTimestamp()")]
    pub struct GetCurrentBlockTimestampCall;
    ///Container type for all input parameters for the `getEthBalance` function with signature `getEthBalance(address)` and selector `0x4d2301cc`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "getEthBalance", abi = "getEthBalance(address)")]
    pub struct GetEthBalanceCall {
        pub addr: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getLastBlockHash` function with signature `getLastBlockHash()` and selector `0x27e86d6e`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "getLastBlockHash", abi = "getLastBlockHash()")]
    pub struct GetLastBlockHashCall;
    ///Container type for all input parameters for the `tryAggregate` function with signature `tryAggregate(bool,(address,bytes)[])` and selector `0xbce38bd7`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "tryAggregate", abi = "tryAggregate(bool,(address,bytes)[])")]
    pub struct TryAggregateCall {
        pub require_success: bool,
        pub calls: ::std::vec::Vec<Call>,
    }
    ///Container type for all input parameters for the `tryBlockAndAggregate` function with signature `tryBlockAndAggregate(bool,(address,bytes)[])` and selector `0x399542e9`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(
        name = "tryBlockAndAggregate",
        abi = "tryBlockAndAggregate(bool,(address,bytes)[])"
    )]
    pub struct TryBlockAndAggregateCall {
        pub require_success: bool,
        pub calls: ::std::vec::Vec<Call>,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ::ethers::contract::EthAbiType)]
    pub enum IMulticall3Calls {
        Aggregate(AggregateCall),
        Aggregate3(Aggregate3Call),
        Aggregate3Value(Aggregate3ValueCall),
        BlockAndAggregate(BlockAndAggregateCall),
        GetBasefee(GetBasefeeCall),
        GetBlockHash(GetBlockHashCall),
        GetBlockNumber(GetBlockNumberCall),
        GetChainId(GetChainIdCall),
        GetCurrentBlockCoinbase(GetCurrentBlockCoinbaseCall),
        GetCurrentBlockDifficulty(GetCurrentBlockDifficultyCall),
        GetCurrentBlockGasLimit(GetCurrentBlockGasLimitCall),
        GetCurrentBlockTimestamp(GetCurrentBlockTimestampCall),
        GetEthBalance(GetEthBalanceCall),
        GetLastBlockHash(GetLastBlockHashCall),
        TryAggregate(TryAggregateCall),
        TryBlockAndAggregate(TryBlockAndAggregateCall),
    }
    impl ::ethers::core::abi::AbiDecode for IMulticall3Calls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ::ethers::core::abi::AbiError> {
            if let Ok(decoded)
                = <AggregateCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(IMulticall3Calls::Aggregate(decoded));
            }
            if let Ok(decoded)
                = <Aggregate3Call as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(IMulticall3Calls::Aggregate3(decoded));
            }
            if let Ok(decoded)
                = <Aggregate3ValueCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(IMulticall3Calls::Aggregate3Value(decoded));
            }
            if let Ok(decoded)
                = <BlockAndAggregateCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(IMulticall3Calls::BlockAndAggregate(decoded));
            }
            if let Ok(decoded)
                = <GetBasefeeCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(IMulticall3Calls::GetBasefee(decoded));
            }
            if let Ok(decoded)
                = <GetBlockHashCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(IMulticall3Calls::GetBlockHash(decoded));
            }
            if let Ok(decoded)
                = <GetBlockNumberCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(IMulticall3Calls::GetBlockNumber(decoded));
            }
            if let Ok(decoded)
                = <GetChainIdCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(IMulticall3Calls::GetChainId(decoded));
            }
            if let Ok(decoded)
                = <GetCurrentBlockCoinbaseCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(IMulticall3Calls::GetCurrentBlockCoinbase(decoded));
            }
            if let Ok(decoded)
                = <GetCurrentBlockDifficultyCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(IMulticall3Calls::GetCurrentBlockDifficulty(decoded));
            }
            if let Ok(decoded)
                = <GetCurrentBlockGasLimitCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(IMulticall3Calls::GetCurrentBlockGasLimit(decoded));
            }
            if let Ok(decoded)
                = <GetCurrentBlockTimestampCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(IMulticall3Calls::GetCurrentBlockTimestamp(decoded));
            }
            if let Ok(decoded)
                = <GetEthBalanceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(IMulticall3Calls::GetEthBalance(decoded));
            }
            if let Ok(decoded)
                = <GetLastBlockHashCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(IMulticall3Calls::GetLastBlockHash(decoded));
            }
            if let Ok(decoded)
                = <TryAggregateCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(IMulticall3Calls::TryAggregate(decoded));
            }
            if let Ok(decoded)
                = <TryBlockAndAggregateCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(IMulticall3Calls::TryBlockAndAggregate(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IMulticall3Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                IMulticall3Calls::Aggregate(element) => element.encode(),
                IMulticall3Calls::Aggregate3(element) => element.encode(),
                IMulticall3Calls::Aggregate3Value(element) => element.encode(),
                IMulticall3Calls::BlockAndAggregate(element) => element.encode(),
                IMulticall3Calls::GetBasefee(element) => element.encode(),
                IMulticall3Calls::GetBlockHash(element) => element.encode(),
                IMulticall3Calls::GetBlockNumber(element) => element.encode(),
                IMulticall3Calls::GetChainId(element) => element.encode(),
                IMulticall3Calls::GetCurrentBlockCoinbase(element) => element.encode(),
                IMulticall3Calls::GetCurrentBlockDifficulty(element) => element.encode(),
                IMulticall3Calls::GetCurrentBlockGasLimit(element) => element.encode(),
                IMulticall3Calls::GetCurrentBlockTimestamp(element) => element.encode(),
                IMulticall3Calls::GetEthBalance(element) => element.encode(),
                IMulticall3Calls::GetLastBlockHash(element) => element.encode(),
                IMulticall3Calls::TryAggregate(element) => element.encode(),
                IMulticall3Calls::TryBlockAndAggregate(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IMulticall3Calls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IMulticall3Calls::Aggregate(element) => element.fmt(f),
                IMulticall3Calls::Aggregate3(element) => element.fmt(f),
                IMulticall3Calls::Aggregate3Value(element) => element.fmt(f),
                IMulticall3Calls::BlockAndAggregate(element) => element.fmt(f),
                IMulticall3Calls::GetBasefee(element) => element.fmt(f),
                IMulticall3Calls::GetBlockHash(element) => element.fmt(f),
                IMulticall3Calls::GetBlockNumber(element) => element.fmt(f),
                IMulticall3Calls::GetChainId(element) => element.fmt(f),
                IMulticall3Calls::GetCurrentBlockCoinbase(element) => element.fmt(f),
                IMulticall3Calls::GetCurrentBlockDifficulty(element) => element.fmt(f),
                IMulticall3Calls::GetCurrentBlockGasLimit(element) => element.fmt(f),
                IMulticall3Calls::GetCurrentBlockTimestamp(element) => element.fmt(f),
                IMulticall3Calls::GetEthBalance(element) => element.fmt(f),
                IMulticall3Calls::GetLastBlockHash(element) => element.fmt(f),
                IMulticall3Calls::TryAggregate(element) => element.fmt(f),
                IMulticall3Calls::TryBlockAndAggregate(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AggregateCall> for IMulticall3Calls {
        fn from(var: AggregateCall) -> Self {
            IMulticall3Calls::Aggregate(var)
        }
    }
    impl ::std::convert::From<Aggregate3Call> for IMulticall3Calls {
        fn from(var: Aggregate3Call) -> Self {
            IMulticall3Calls::Aggregate3(var)
        }
    }
    impl ::std::convert::From<Aggregate3ValueCall> for IMulticall3Calls {
        fn from(var: Aggregate3ValueCall) -> Self {
            IMulticall3Calls::Aggregate3Value(var)
        }
    }
    impl ::std::convert::From<BlockAndAggregateCall> for IMulticall3Calls {
        fn from(var: BlockAndAggregateCall) -> Self {
            IMulticall3Calls::BlockAndAggregate(var)
        }
    }
    impl ::std::convert::From<GetBasefeeCall> for IMulticall3Calls {
        fn from(var: GetBasefeeCall) -> Self {
            IMulticall3Calls::GetBasefee(var)
        }
    }
    impl ::std::convert::From<GetBlockHashCall> for IMulticall3Calls {
        fn from(var: GetBlockHashCall) -> Self {
            IMulticall3Calls::GetBlockHash(var)
        }
    }
    impl ::std::convert::From<GetBlockNumberCall> for IMulticall3Calls {
        fn from(var: GetBlockNumberCall) -> Self {
            IMulticall3Calls::GetBlockNumber(var)
        }
    }
    impl ::std::convert::From<GetChainIdCall> for IMulticall3Calls {
        fn from(var: GetChainIdCall) -> Self {
            IMulticall3Calls::GetChainId(var)
        }
    }
    impl ::std::convert::From<GetCurrentBlockCoinbaseCall> for IMulticall3Calls {
        fn from(var: GetCurrentBlockCoinbaseCall) -> Self {
            IMulticall3Calls::GetCurrentBlockCoinbase(var)
        }
    }
    impl ::std::convert::From<GetCurrentBlockDifficultyCall> for IMulticall3Calls {
        fn from(var: GetCurrentBlockDifficultyCall) -> Self {
            IMulticall3Calls::GetCurrentBlockDifficulty(var)
        }
    }
    impl ::std::convert::From<GetCurrentBlockGasLimitCall> for IMulticall3Calls {
        fn from(var: GetCurrentBlockGasLimitCall) -> Self {
            IMulticall3Calls::GetCurrentBlockGasLimit(var)
        }
    }
    impl ::std::convert::From<GetCurrentBlockTimestampCall> for IMulticall3Calls {
        fn from(var: GetCurrentBlockTimestampCall) -> Self {
            IMulticall3Calls::GetCurrentBlockTimestamp(var)
        }
    }
    impl ::std::convert::From<GetEthBalanceCall> for IMulticall3Calls {
        fn from(var: GetEthBalanceCall) -> Self {
            IMulticall3Calls::GetEthBalance(var)
        }
    }
    impl ::std::convert::From<GetLastBlockHashCall> for IMulticall3Calls {
        fn from(var: GetLastBlockHashCall) -> Self {
            IMulticall3Calls::GetLastBlockHash(var)
        }
    }
    impl ::std::convert::From<TryAggregateCall> for IMulticall3Calls {
        fn from(var: TryAggregateCall) -> Self {
            IMulticall3Calls::TryAggregate(var)
        }
    }
    impl ::std::convert::From<TryBlockAndAggregateCall> for IMulticall3Calls {
        fn from(var: TryBlockAndAggregateCall) -> Self {
            IMulticall3Calls::TryBlockAndAggregate(var)
        }
    }
    ///Container type for all return fields from the `aggregate` function with signature `aggregate((address,bytes)[])` and selector `0x252dba42`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct AggregateReturn {
        pub block_number: ::ethers::core::types::U256,
        pub return_data: ::std::vec::Vec<::ethers::core::types::Bytes>,
    }
    ///Container type for all return fields from the `aggregate3` function with signature `aggregate3((address,bool,bytes)[])` and selector `0x82ad56cb`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct Aggregate3Return {
        pub return_data: ::std::vec::Vec<Result>,
    }
    ///Container type for all return fields from the `aggregate3Value` function with signature `aggregate3Value((address,bool,uint256,bytes)[])` and selector `0x174dea71`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct Aggregate3ValueReturn {
        pub return_data: ::std::vec::Vec<Result>,
    }
    ///Container type for all return fields from the `blockAndAggregate` function with signature `blockAndAggregate((address,bytes)[])` and selector `0xc3077fa9`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct BlockAndAggregateReturn {
        pub block_number: ::ethers::core::types::U256,
        pub block_hash: [u8; 32],
        pub return_data: ::std::vec::Vec<Result>,
    }
    ///Container type for all return fields from the `getBasefee` function with signature `getBasefee()` and selector `0x3e64a696`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct GetBasefeeReturn {
        pub basefee: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getBlockHash` function with signature `getBlockHash(uint256)` and selector `0xee82ac5e`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct GetBlockHashReturn {
        pub block_hash: [u8; 32],
    }
    ///Container type for all return fields from the `getBlockNumber` function with signature `getBlockNumber()` and selector `0x42cbb15c`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct GetBlockNumberReturn {
        pub block_number: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getChainId` function with signature `getChainId()` and selector `0x3408e470`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct GetChainIdReturn {
        pub chainid: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getCurrentBlockCoinbase` function with signature `getCurrentBlockCoinbase()` and selector `0xa8b0574e`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct GetCurrentBlockCoinbaseReturn {
        pub coinbase: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `getCurrentBlockDifficulty` function with signature `getCurrentBlockDifficulty()` and selector `0x72425d9d`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct GetCurrentBlockDifficultyReturn {
        pub difficulty: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getCurrentBlockGasLimit` function with signature `getCurrentBlockGasLimit()` and selector `0x86d516e8`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct GetCurrentBlockGasLimitReturn {
        pub gaslimit: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getCurrentBlockTimestamp` function with signature `getCurrentBlockTimestamp()` and selector `0x0f28c97d`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct GetCurrentBlockTimestampReturn {
        pub timestamp: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getEthBalance` function with signature `getEthBalance(address)` and selector `0x4d2301cc`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct GetEthBalanceReturn {
        pub balance: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getLastBlockHash` function with signature `getLastBlockHash()` and selector `0x27e86d6e`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct GetLastBlockHashReturn {
        pub block_hash: [u8; 32],
    }
    ///Container type for all return fields from the `tryAggregate` function with signature `tryAggregate(bool,(address,bytes)[])` and selector `0xbce38bd7`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct TryAggregateReturn {
        pub return_data: ::std::vec::Vec<Result>,
    }
    ///Container type for all return fields from the `tryBlockAndAggregate` function with signature `tryBlockAndAggregate(bool,(address,bytes)[])` and selector `0x399542e9`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct TryBlockAndAggregateReturn {
        pub block_number: ::ethers::core::types::U256,
        pub block_hash: [u8; 32],
        pub return_data: ::std::vec::Vec<Result>,
    }
    ///`Call(address,bytes)`
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    pub struct Call {
        pub target: ::ethers::core::types::Address,
        pub call_data: ::ethers::core::types::Bytes,
    }
    ///`Call3(address,bool,bytes)`
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    pub struct Call3 {
        pub target: ::ethers::core::types::Address,
        pub allow_failure: bool,
        pub call_data: ::ethers::core::types::Bytes,
    }
    ///`Call3Value(address,bool,uint256,bytes)`
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    pub struct Call3Value {
        pub target: ::ethers::core::types::Address,
        pub allow_failure: bool,
        pub value: ::ethers::core::types::U256,
        pub call_data: ::ethers::core::types::Bytes,
    }
    ///`Result(bool,bytes)`
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    pub struct Result {
        pub success: bool,
        pub return_data: ::ethers::core::types::Bytes,
    }
}
