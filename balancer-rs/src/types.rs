extern crate ethcontract;

pub use std::fmt;

pub type IERC20 = ethcontract::Address;
pub type Address = ethcontract::Address;
pub type Bytes32 = ethcontract::Bytes<[u8; 32]>;
pub type U256 = ethcontract::U256;
pub type Web3 = ethcontract::Web3<ethcontract::web3::transports::Http>;
