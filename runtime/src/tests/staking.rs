use super::generate_account;
use crate::mock::runtime::{AccountId, Balances, BondingDuration, Runtime, RuntimeOrigin, Session};
use frame_support::assert_ok;
use frame_support::traits::Currency;
use pallet_staking::RewardDestination;
use runtime_utils::{ExtBuilder, RollTo, Trivial};
use std::default::Default;

type Staking = pallet_staking::Pallet<Runtime>;

#[test]
// test with different payees
fn stash_bonds_with_controller_to_stash() {
	let stash = generate_account("stash");
	let controller = generate_account("controller");
	let value = <Balances as Currency<AccountId>>::minimum_balance();

	ExtBuilder::<()>::default().build_sans_config().execute_with(|| {
		Trivial::<(Session, Runtime)>::roll_to(1);

		let _ = Balances::deposit_creating(&stash, 2 * value);

		let usable = Balances::usable_balance(&stash);
		assert_eq!(2 * value, usable);

		//in what era
		assert_ok!(Staking::bond(
			RuntimeOrigin::signed(stash.clone()),
			controller.clone().into(),
			value,
			RewardDestination::Stash,
		));

		let usable = Balances::usable_balance(&stash);
		assert_eq!(value, usable);

		//unbond
		assert_ok!(Staking::unbond(RuntimeOrigin::signed(controller.clone()), value));

		//unbondig defered until bonding eras pass
		let usable = Balances::usable_balance(&stash);
		assert_eq!(value, usable);

		// eras pass
		Trivial::<(Session, Runtime)>::roll_to(BondingDuration::get() + 1);
		//TODO end_era

		//unbondig defered until bonding eras pass
		let usable = Balances::usable_balance(&stash);
		assert_eq!(2 * value, usable);
	});

	//assert on balances
}

//only the controller can unbond
