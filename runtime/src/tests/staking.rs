use super::generate_account;
use crate::mock::runtime::{
	AccountId, Balances, BlocksPerEra, BondingDuration, Runtime, RuntimeOrigin, Session,
};
use frame_support::assert_ok;
use frame_support::traits::Currency;
use pallet_staking_substrate::Error;
use pallet_staking_substrate::RewardDestination;
use sp_staking::StakingInterface;
use runtime_utils::{ExtBuilder, RollTo, Trivial};
use std::default::Default;

type Staking = pallet_staking_substrate::Pallet<Runtime>;

const BLOCKS_UNTIL_UNBONDED: u32 = BlocksPerEra::get() * BondingDuration::get();

#[test]
fn active_era_transitions() {
	ExtBuilder::default().build_sans_config().execute_with(|| {
		assert!(Staking::active_era().is_none());
		let mut height = 1;
		Trivial::<Session, Runtime>::roll_to(height);
		assert_eq!(2, Staking::eras_start_session_index(0).unwrap());
		height += BlocksPerEra::get();
		Trivial::<Session, Runtime>::roll_to(height + 1);
		assert_eq!(1, Staking::active_era().unwrap().index);
	});
}

#[test]
fn stake_and_unstake_with_a_controller_and_stash_as_payee() {
	let stash = generate_account("stash");
	let controller = generate_account("controller");
	let value = <Balances as Currency<AccountId>>::minimum_balance();

	ExtBuilder::default().build_sans_config().execute_with(|| {
		let mut height = 1;
		Trivial::<Session, Runtime>::roll_to(height);

		// prime balance to create the account
		let _ = Balances::deposit_creating(&stash, value);
		let usable = Balances::usable_balance(&stash);
		assert_eq!(value, usable);

		// lock stash's funds under a controller
		assert_ok!(Staking::bond(
			RuntimeOrigin::signed(stash.clone()),
			controller.clone().into(),
			value,
			RewardDestination::Stash,
		));

		// stake is found and usable balance dropped
		assert_eq!(value, Staking::total_stake(&stash).unwrap());
		assert_eq!(value, Staking::active_stake(&stash).unwrap());
		let usable = Balances::usable_balance(&stash);
		assert_eq!(0, usable);

		// unbond, efective after BondingPeriod has passed
		assert_ok!(Staking::unbond(RuntimeOrigin::signed(controller.clone()), value));
		assert_eq!(value, Staking::total_stake(&stash).unwrap());
		assert_eq!(0, Staking::active_stake(&stash).unwrap());
		let usable = Balances::usable_balance(&stash);
		assert_eq!(0, usable);

		// Cant withdraw before minimum staking period has elapsed; NOOPs
		assert_ok!(Staking::withdraw_unbonded(RuntimeOrigin::signed(controller.clone()), 0));
		assert_eq!(value, Staking::total_stake(&stash).unwrap());
		assert_eq!(0, Staking::active_stake(&stash).unwrap());
		let usable = Balances::usable_balance(&stash);
		assert_eq!(0, usable);

		// unbonding defered until bonding eras pass
		height += BLOCKS_UNTIL_UNBONDED;

		Trivial::<Session, Runtime>::roll_to(height);

		assert_ok!(Staking::withdraw_unbonded(RuntimeOrigin::signed(controller), 0));
		assert_eq!(Staking::total_stake(&stash).unwrap_err(), Error::<Runtime>::NotStash.into());
		assert_eq!(Staking::active_stake(&stash).unwrap_err(), Error::<Runtime>::NotStash.into());
		let usable = Balances::usable_balance(&stash);
		assert_eq!(value, usable);
	});
}
