// Reverted so currently a no-op. Formerly the "register currency" migration.
use super::Migrate;
use frame_support::weights::Weight;
use sp_std::prelude::*;

pub(crate) struct Migration;

impl Migrate for Migration {
	fn pre_upgrade(&self) -> Vec<u8> {
		vec![]
	}

	fn migrate(&self) -> Weight {
		Weight::zero()
	}
	fn post_upgrade(&self, _ctx: Vec<u8>) {}
}
