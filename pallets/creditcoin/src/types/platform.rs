use core::convert::TryFrom;
use frame_support::{traits::ConstU32, BoundedVec, RuntimeDebug};
use parity_scale_codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_runtime::traits::Hash as HashT;
use strum::EnumCount;

use crate::ExternalAddress;

// as of EIP-155 the max chain ID is 9,223,372,036,854,775,771 which fits well within a u64
#[derive(
	Copy, Clone, RuntimeDebug, PartialEq, Eq, PartialOrd, Encode, Decode, TypeInfo, MaxEncodedLen,
)]
#[repr(transparent)]
pub struct EvmChainId(#[codec(compact)] u64);

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

#[derive(
	Clone, RuntimeDebug, PartialEq, Eq, PartialOrd, Encode, Decode, TypeInfo, MaxEncodedLen,
)]
pub struct EvmInfo {
	pub chain_id: EvmChainId,
}

#[derive(
	Clone, RuntimeDebug, PartialEq, Eq, PartialOrd, Encode, Decode, TypeInfo, MaxEncodedLen,
)]
pub enum NewBlockchain {
	Evm(EvmInfo),
}

#[derive(
	Copy,
	Clone,
	RuntimeDebug,
	PartialEq,
	Eq,
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

#[derive(
	Clone, RuntimeDebug, PartialEq, Eq, PartialOrd, Encode, Decode, TypeInfo, MaxEncodedLen,
)]
pub enum EvmCurrencyType {
	SmartContract(ExternalAddress, EvmSupportedTransferKinds),
}

#[derive(
	Clone, RuntimeDebug, PartialEq, Eq, PartialOrd, Encode, Decode, TypeInfo, MaxEncodedLen,
)]
pub enum Currency {
	Evm(EvmCurrencyType, EvmInfo),
}

#[derive(
	Clone, RuntimeDebug, PartialEq, Eq, PartialOrd, Encode, Decode, TypeInfo, MaxEncodedLen,
)]
pub enum NewTransferKind {
	Evm(EvmTransferKind),
}

#[derive(
	Clone, RuntimeDebug, PartialEq, Eq, PartialOrd, Encode, Decode, TypeInfo, MaxEncodedLen,
)]
pub struct CurrencyId<Hash>(Hash);

impl<H> CurrencyId<H> {
	pub fn new<T>(currency: &Currency) -> Self
	where
		T: frame_system::Config,
		<T as frame_system::Config>::Hashing: sp_runtime::traits::Hash<Output = H>,
	{
		match currency {
			Currency::Evm(EvmCurrencyType::SmartContract(address, _), evm_info) => {
				let encoded = (address, evm_info.chain_id).encode();
				CurrencyId(T::Hashing::hash(&encoded))
			},
		}
	}
}
