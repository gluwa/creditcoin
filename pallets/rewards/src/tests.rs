use crate::{mock::*, REWARD_HALF_LIFE};
use frame_support::{assert_noop, assert_ok};

#[test]
fn reward_amount_genesis() {
	assert_eq!(Rewards::reward_amount(0), 28_000_000_000_000_000_000);
}

#[test]
fn reward_amount_block_one() {
	assert_eq!(Rewards::reward_amount(1), 28_000_000_000_000_000_000);
}

#[test]
fn reward_amount_after_one_halflife() {
	assert_eq!(Rewards::reward_amount(REWARD_HALF_LIFE + 1), 950_000_000_000_000_000 * 28);
}

#[test]
fn reward_amount_limit() {
	assert_eq!(Rewards::reward_amount(u64::MAX), 0);
}
