use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{traits::ConstU32, BoundedVec, RuntimeDebug};
use scale_info::TypeInfo;
use sp_runtime::traits::Hash as HashT;
use strum::EnumCount;

use crate::{ExternalAddress, LegacyTransferKind};

// as of EIP-155 the max chain ID is 9,223,372,036,854,775,771 which fits well within a u64
#[derive(
	Copy, Clone, RuntimeDebug, PartialEq, Eq, PartialOrd, Encode, Decode, TypeInfo, MaxEncodedLen,
)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
#[repr(transparent)]
pub struct EvmChainId(#[codec(compact)] u64);

impl From<u64> for EvmChainId {
	fn from(value: u64) -> Self {
		EvmChainId(value)
	}
}

impl EvmChainId {
	pub const fn new(value: u64) -> Self {
		EvmChainId(value)
	}
}

#[derive(
	Clone, RuntimeDebug, PartialEq, Eq, PartialOrd, Encode, Decode, TypeInfo, MaxEncodedLen,
)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct EvmInfo {
	pub chain_id: EvmChainId,
}

#[derive(
	Clone, RuntimeDebug, PartialEq, Eq, PartialOrd, Encode, Decode, TypeInfo, MaxEncodedLen,
)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub enum Blockchain {
	Evm(EvmInfo),
}

impl Blockchain {
	pub const fn evm(chain_id: EvmChainId) -> Blockchain {
		Blockchain::Evm(EvmInfo { chain_id })
	}
	// Chain IDs from chainlist.org and Gluwa's internal luniverse documentation
	pub const ETHEREUM: Blockchain = Blockchain::evm(EvmChainId::new(1));
	pub const RINKEBY: Blockchain = Blockchain::evm(EvmChainId::new(4));
	pub const LUNIVERSE_TESTNET: Blockchain = Blockchain::evm(EvmChainId::new(1635501961136826136));
	pub const LUNIVERSE: Blockchain = Blockchain::evm(EvmChainId::new(3158073271666164067));

	pub fn as_bytes(&self) -> &[u8] {
		match self {
			&Blockchain::ETHEREUM => b"ethereum",
			&Blockchain::RINKEBY => b"rinkeby",
			&(Blockchain::LUNIVERSE_TESTNET | Blockchain::LUNIVERSE) => b"luniverse",
			_ => todo!(),
		}
	}

	pub fn supports(&self, kind: &LegacyTransferKind) -> bool {
		match (self, kind) {
			(
				&Blockchain::ETHEREUM
				| &Blockchain::RINKEBY
				| &Blockchain::LUNIVERSE
				| &Blockchain::LUNIVERSE_TESTNET,
				LegacyTransferKind::Erc20(_)
				| LegacyTransferKind::Ethless(_)
				| LegacyTransferKind::Native,
			) => true,
			(_, _) => false,
		}
	}
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
pub enum TransferKind {
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

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn blockchain_as_bytes_back_compat() {
		assert_eq!(Blockchain::ETHEREUM.as_bytes(), b"ethereum");
		assert_eq!(Blockchain::RINKEBY.as_bytes(), b"rinkeby");
		assert_eq!(Blockchain::LUNIVERSE.as_bytes(), b"luniverse");
		assert_eq!(Blockchain::LUNIVERSE_TESTNET.as_bytes(), b"luniverse");
	}
}
