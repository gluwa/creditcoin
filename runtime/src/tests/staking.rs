use super::generate_account;
use crate::mock::runtime::{
	AccountId, Balances, BlocksPerEra, BondingDuration, Runtime, RuntimeOrigin, TaskVoting,
	Timestamp,
};
use frame_support::traits::Currency;
use frame_support::{assert_noop, assert_ok};
use pallet_staking::{era::EraInterface, Error, RewardDestination};
use pallet_staking::{pallet as staking, ActiveEraInfo, StakingInterface};
use runtime_utils::{roll_to, ExtBuilder, Trivial};
use std::default::Default;

type Staking = pallet_staking::Pallet<Runtime>;

const BLOCKS_UNTIL_UNBONDED: u32 = BlocksPerEra::get() * BondingDuration::get();

#[test]
fn stake_and_unstake_with_a_controller_and_stash_as_payee() {
	let stash = generate_account("stash");
	let controller = generate_account("controller");
	let value = <Balances as Currency<AccountId>>::minimum_balance();

	ExtBuilder::<()>::default().build_sans_config().execute_with(|| {
		let mut height = 1;
		roll_to::<Trivial, Runtime, ()>(height);
		Timestamp::set_timestamp(1);
		//bypass defensive first era.
		{
			staking::ActiveEra::<Runtime>::set(Some(ActiveEraInfo { index: 0, start: Some(0) }));
			staking::ErasStartSessionIndex::<Runtime>::set(0, Some(height));
		}

		height += BlocksPerEra::get();
		assert_eq!(height, Staking::next_era_start(1));
		roll_to::<Trivial, Runtime, TaskVoting>(height);

		//prime balance with enough funds to prevent dusting
		let _ = Balances::deposit_creating(&stash, 2 * value);
		let usable = Balances::usable_balance(&stash);
		assert_eq!(2 * value, usable);

		assert_ok!(Staking::bond(
			RuntimeOrigin::signed(stash.clone()),
			controller.clone().into(),
			value,
			RewardDestination::Stash,
		));
		assert_eq!(value, Staking::stake(&stash).expect("stash's stake").total);
		//usable balance dropped
		let usable = Balances::usable_balance(&stash);
		assert_eq!(value, usable);

		//unbond, efective after BondingPeriod has passed
		assert_ok!(Staking::unbond(RuntimeOrigin::signed(controller.clone()), value));
		assert_eq!(value, Staking::stake(&stash).expect("stash's stake").total);
		let usable = Balances::usable_balance(&stash);
		assert_eq!(value, usable);

		//Cant withdraw before minimum staking period has elapsed; NOOPs
		assert_ok!(Staking::withdraw_unbonded(RuntimeOrigin::signed(controller.clone()), 0));
		assert_eq!(value, Staking::stake(&stash).expect("stash's stake").total);
		let usable = Balances::usable_balance(&stash);
		assert_eq!(value, usable);

		//unbondig defered until bonding eras pass
		height += BLOCKS_UNTIL_UNBONDED;
		roll_to::<Trivial, Runtime, TaskVoting>(height);

		assert_ok!(Staking::withdraw_unbonded(RuntimeOrigin::signed(controller), 0));
		let usable = Balances::usable_balance(&stash);
		assert_eq!(2 * value, usable);
		assert_noop!(Staking::stake(&stash).map(|_| ()), Error::<Runtime>::NotStash);
	});
}
