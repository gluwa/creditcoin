use frame_support::RuntimeDebug;
use scale_info::TypeInfo;
use sp_core::ConstU32;
use sp_io::MultiRemovalResults;
use sp_runtime::codec::{Decode, Encode, MaxEncodedLen};
use sp_runtime::traits::{One, Saturating};
use sp_runtime::WeakBoundedVec;
use sp_std::cmp;

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct StorageItemCleanupState<BlockNumber> {
	pub on_block: BlockNumber,
	// pub cursor: Option<Vec<u8>>,
	pub cursor: Option<WeakBoundedVec<u8, ConstU32<256>>>,
}

impl<BlockNumber> StorageItemCleanupState<BlockNumber> {
	pub fn new(on_block: BlockNumber) -> Self {
		Self { on_block, cursor: None }
	}

	pub fn cursor(&self) -> Option<&[u8]> {
		match self.cursor.as_ref() {
			Some(c) => Some(c.as_slice()),
			None => None,
		}
	}
}

impl<BlockNumber> StorageItemCleanupState<BlockNumber>
where
	BlockNumber: One + Saturating,
{
	pub fn updated_with(self, results: Option<MultiRemovalResults>) -> Self {
		match results {
			Some(results) => Self {
				on_block: if results.maybe_cursor.is_some() {
					self.on_block
				} else {
					self.on_block.saturating_add(BlockNumber::one())
				},
				cursor: results.maybe_cursor.map(|c| {
					WeakBoundedVec::force_from(c, Some("storage item cleanup state cursor"))
				}),
			},
			None => self,
		}
	}
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]

pub struct StorageCleanupState<BlockNumber> {
	pub ask_orders: StorageItemCleanupState<BlockNumber>,
	pub bid_orders: StorageItemCleanupState<BlockNumber>,
	pub offers: StorageItemCleanupState<BlockNumber>,
}

impl<BlockNumber> StorageCleanupState<BlockNumber>
where
	BlockNumber: Clone + Ord,
{
	pub fn new(on_block: BlockNumber) -> Self {
		Self {
			ask_orders: StorageItemCleanupState::new(on_block.clone()),
			bid_orders: StorageItemCleanupState::new(on_block.clone()),
			offers: StorageItemCleanupState::new(on_block),
		}
	}

	pub fn latest_block(&self) -> BlockNumber {
		cmp::max(
			cmp::max(self.ask_orders.on_block.clone(), self.bid_orders.on_block.clone()),
			self.offers.on_block.clone(),
		)
	}
}
