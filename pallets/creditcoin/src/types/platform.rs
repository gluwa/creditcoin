use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{traits::ConstU32, BoundedVec, RuntimeDebug};
use scale_info::TypeInfo;
use strum::EnumCount;

use crate::ExternalAddress;

// as of EIP-155 the max chain ID is 9,223,372,036,854,775,771 which fits well within a u64
#[derive(
	Copy, Clone, RuntimeDebug, PartialEq, PartialOrd, Encode, Decode, TypeInfo, MaxEncodedLen,
)]
pub struct EvmChainId(u64);

impl From<u64> for EvmChainId {
	fn from(value: u64) -> Self {
		EvmChainId(value)
	}
}

impl EvmChainId {
	pub fn new(value: u64) -> Self {
		EvmChainId(value)
	}
}

#[derive(Clone, RuntimeDebug, PartialEq, PartialOrd, Encode, Decode, TypeInfo, MaxEncodedLen)]
pub struct EvmInfo {
	chain_id: EvmChainId,
}

#[derive(Clone, RuntimeDebug, PartialEq, PartialOrd, Encode, Decode, TypeInfo, MaxEncodedLen)]
enum Blockchain {
	Evm(EvmInfo),
}

#[derive(
	Copy,
	Clone,
	RuntimeDebug,
	PartialEq,
	PartialOrd,
	Encode,
	Decode,
	TypeInfo,
	MaxEncodedLen,
	EnumCount,
)]
pub enum EvmTransferKind {
	Erc20,
	Ethless,
}

pub type EvmSupportedTransferKinds =
	BoundedVec<EvmTransferKind, ConstU32<{ EvmTransferKind::COUNT as u32 }>>;

#[derive(Clone, RuntimeDebug, PartialEq, PartialOrd, Encode, Decode, TypeInfo, MaxEncodedLen)]
pub enum EvmCurrencyType {
	SmartContract(ExternalAddress, EvmSupportedTransferKinds),
}

#[derive(Clone, RuntimeDebug, PartialEq, PartialOrd, Encode, Decode, TypeInfo, MaxEncodedLen)]
pub enum Currency {
	Evm(EvmCurrencyType, EvmInfo),
}

#[derive(Clone, RuntimeDebug, PartialEq, PartialOrd, Encode, Decode, TypeInfo, MaxEncodedLen)]
enum TransferKind {
	Evm(EvmTransferKind),
}
