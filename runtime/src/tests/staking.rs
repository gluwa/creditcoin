use super::generate_account;
use crate::mock::runtime::{
	AccountId, Balances, BlocksPerEra, BondingDuration, Runtime, RuntimeOrigin, TaskVoting,
	Timestamp,
};
use assert_matches::assert_matches;
use frame_support::traits::Currency;
use frame_support::{assert_noop, assert_ok};
use pallet_staking::{era::EraInterface, pallet::Forcing, Error, RewardDestination};
use pallet_staking::{pallet as staking, ActiveEraInfo, StakingInterface};
use runtime_utils::{roll_to, ExtBuilder, Trivial};
use sp_runtime::{
	traits::{BadOrigin, LookupError},
	MultiAddress,
};
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

#[test]
fn force_no_eras_should_be_signed() {
	ExtBuilder::<()>::default().build_sans_config().execute_with(|| {
		assert_noop!(Staking::force_no_eras(RuntimeOrigin::none()), BadOrigin);
	})
}

#[test]
fn force_no_eras_should_be_signed_by_root() {
	let controller = generate_account("not-root");

	ExtBuilder::<()>::default().build_sans_config().execute_with(|| {
		assert_noop!(Staking::force_no_eras(RuntimeOrigin::signed(controller)), BadOrigin);
	})
}

#[test]
fn force_no_eras_should_work() {
	ExtBuilder::<()>::default().build_sans_config().execute_with(|| {
		// deliberately put a different value in storage
		staking::ForceEra::<Runtime>::put(Forcing::ForceNew);

		assert_ok!(Staking::force_no_eras(RuntimeOrigin::root()));
		assert_eq!(Staking::force_era(), Forcing::ForceNone);
	})
}

#[test]
fn force_new_era_should_be_signed() {
	ExtBuilder::<()>::default().build_sans_config().execute_with(|| {
		assert_noop!(Staking::force_new_era(RuntimeOrigin::none()), BadOrigin);
	})
}

#[test]
fn force_new_era_should_be_signed_by_root() {
	let controller = generate_account("not-root");

	ExtBuilder::<()>::default().build_sans_config().execute_with(|| {
		assert_noop!(Staking::force_new_era(RuntimeOrigin::signed(controller)), BadOrigin);
	})
}

#[test]
fn force_new_era_should_work() {
	ExtBuilder::<()>::default().build_sans_config().execute_with(|| {
		// deliberately put a different value in storage
		staking::ForceEra::<Runtime>::put(Forcing::ForceNone);

		assert_ok!(Staking::force_new_era(RuntimeOrigin::root()));
		assert_eq!(Staking::force_era(), Forcing::ForceNew);
	})
}

#[test]
fn force_new_era_always_should_be_signed() {
	ExtBuilder::<()>::default().build_sans_config().execute_with(|| {
		assert_noop!(Staking::force_new_era_always(RuntimeOrigin::none()), BadOrigin);
	})
}

#[test]
fn force_new_era_always_should_be_signed_by_root() {
	let controller = generate_account("not-root");

	ExtBuilder::<()>::default().build_sans_config().execute_with(|| {
		assert_noop!(Staking::force_new_era_always(RuntimeOrigin::signed(controller)), BadOrigin);
	})
}

#[test]
fn force_new_era_always_should_work() {
	ExtBuilder::<()>::default().build_sans_config().execute_with(|| {
		// deliberately put a different value in storage
		staking::ForceEra::<Runtime>::put(Forcing::ForceNone);

		assert_ok!(Staking::force_new_era_always(RuntimeOrigin::root()));
		assert_eq!(Staking::force_era(), Forcing::ForceAlways);
	})
}

#[test]
fn bond_should_be_signed() {
	let controller = generate_account("controller");
	let value = <Balances as Currency<AccountId>>::minimum_balance();

	ExtBuilder::<()>::default().build_sans_config().execute_with(|| {
		assert_noop!(
			Staking::bond(
				RuntimeOrigin::none(),
				controller.into(),
				value,
				RewardDestination::Stash,
			),
			BadOrigin
		);
	});
}

#[test]
fn bond_should_fail_when_already_bonded() {
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

		assert_noop!(
			Staking::bond(
				RuntimeOrigin::signed(stash),
				controller.into(),
				value,
				RewardDestination::Stash,
			),
			Error::<Runtime>::AlreadyBonded
		);
	});
}

#[test]
fn bond_should_fail_when_controller_not_found() {
	let stash = generate_account("stash");
	let controller = MultiAddress::Raw(vec![1, 2, 3, 4]);
	let value = <Balances as Currency<AccountId>>::minimum_balance();

	ExtBuilder::<()>::default().build_sans_config().execute_with(|| {
		assert_noop!(
			Staking::bond(
				RuntimeOrigin::signed(stash),
				controller.into(),
				value,
				RewardDestination::Stash,
			),
			LookupError
		);
	});
}

#[test]
fn bond_should_fail_when_value_is_dust() {
	let stash = generate_account("stash");
	let controller = generate_account("controller");
	// value is less than minimum
	let value = <Balances as Currency<AccountId>>::minimum_balance() - 1;

	ExtBuilder::<()>::default().build_sans_config().execute_with(|| {
		assert_noop!(
			Staking::bond(
				RuntimeOrigin::signed(stash),
				controller.into(),
				value,
				RewardDestination::Stash,
			),
			Error::<Runtime>::InsufficientBond
		);
	});
}

#[test]
fn bond_should_fail_when_referenced_stash_is_not_funded() {
	let stash = generate_account("stash");
	let controller = generate_account("controller");
	let value = <Balances as Currency<AccountId>>::minimum_balance();

	ExtBuilder::<()>::default().build_sans_config().execute_with(|| {
		assert_noop!(
			Staking::bond(
				RuntimeOrigin::signed(stash),
				controller.into(),
				value,
				RewardDestination::Stash,
			),
			Error::<Runtime>::BadState
		);
	});
}

#[test]
fn bond_should_fail_when_controller_has_already_paired_with_a_stash() {
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

		let stash = generate_account("another-stash");
		assert_noop!(
			Staking::bond(
				RuntimeOrigin::signed(stash),
				controller.into(),
				value,
				RewardDestination::Stash,
			),
			Error::<Runtime>::AlreadyPaired
		);
	});
}

#[test]
fn unbond_should_be_signed() {
	ExtBuilder::<()>::default().build_sans_config().execute_with(|| {
		assert_noop!(Staking::unbond(RuntimeOrigin::none(), 1), BadOrigin);
	});
}

#[test]
fn unbond_should_fail_when_controller_account_did_not_bond_previously() {
	let controller = generate_account("controller");

	ExtBuilder::<()>::default().build_sans_config().execute_with(|| {
		assert_noop!(
			Staking::unbond(RuntimeOrigin::signed(controller), 1),
			Error::<Runtime>::NotController
		);
	});
}

#[test]
fn set_payee_should_be_signed() {
	ExtBuilder::<()>::default().build_sans_config().execute_with(|| {
		assert_noop!(
			Staking::set_payee(RuntimeOrigin::none(), RewardDestination::Stash),
			BadOrigin
		);
	});
}

#[test]
fn set_payee_should_fail_when_controller_account_did_not_bond_previously() {
	let controller = generate_account("controller");

	ExtBuilder::<()>::default().build_sans_config().execute_with(|| {
		assert_noop!(
			Staking::set_payee(RuntimeOrigin::signed(controller), RewardDestination::Stash),
			Error::<Runtime>::NotController
		);
	});
}

#[test]
fn set_payee_works() {
	let stash = generate_account("stash");
	let controller = generate_account("controller");
	let beneficiary = generate_account("beneficiary");
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

		assert_ok!(Staking::bond(
			RuntimeOrigin::signed(stash.clone()),
			controller.clone().into(),
			value,
			RewardDestination::Stash,
		));
		// ATM the payee is the destination passed as argument in bond()
		let payee = Staking::payee(&stash);
		assert_eq!(payee, RewardDestination::Stash);

		// we can configure somebody else to receive the payout
		assert_ok!(Staking::set_payee(
			RuntimeOrigin::signed(controller),
			RewardDestination::Account(beneficiary.clone())
		));
		let payee = Staking::payee(&stash);
		assert_eq!(payee, RewardDestination::Account(beneficiary));
	});
}

#[test]
fn set_controller_should_be_signed() {
	let controller = generate_account("controller");

	ExtBuilder::<()>::default().build_sans_config().execute_with(|| {
		assert_noop!(
			Staking::set_controller(
				RuntimeOrigin::none(),
				sp_runtime::MultiAddress::Id(controller)
			),
			BadOrigin
		);
	});
}

#[test]
fn set_controller_should_fail_when_stash_account_did_not_bond_previously() {
	let stash = generate_account("stash");
	let controller = generate_account("controller");

	ExtBuilder::<()>::default().build_sans_config().execute_with(|| {
		assert_noop!(
			Staking::set_controller(
				RuntimeOrigin::signed(stash),
				sp_runtime::MultiAddress::Id(controller)
			),
			Error::<Runtime>::NotStash
		);
	});
}

#[test]
fn set_controller_should_fail_when_trying_to_use_a_controller_which_is_already_in_use() {
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

		assert_ok!(Staking::bond(
			RuntimeOrigin::signed(stash.clone()),
			controller.clone().into(),
			value,
			RewardDestination::Stash,
		));

		assert_noop!(
			Staking::set_controller(
				RuntimeOrigin::signed(stash),
				sp_runtime::MultiAddress::Id(controller)
			),
			Error::<Runtime>::AlreadyPaired
		);
	});
}

#[test]
fn set_controller_should_work() {
	let stash = generate_account("stash");
	let controller = generate_account("controller");
	let new_controller = generate_account("new-controller");
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

		assert_ok!(Staking::bond(
			RuntimeOrigin::signed(stash.clone()),
			controller.clone().into(),
			value,
			RewardDestination::Stash,
		));
		assert_eq!(controller, Staking::bonded(stash.clone()).unwrap());
		assert_matches!(Staking::ledger(&controller), Some(_));
		assert_matches!(Staking::ledger(&new_controller), None);

		assert_ok!(Staking::set_controller(
			RuntimeOrigin::signed(stash.clone()),
			sp_runtime::MultiAddress::Id(new_controller.clone())
		));

		assert_eq!(new_controller, Staking::bonded(stash).unwrap());
		assert_matches!(Staking::ledger(&controller), None);
		assert_matches!(Staking::ledger(&new_controller), Some(_));
	});
}

#[test]
fn set_invulnerables_should_be_signed() {
	ExtBuilder::<()>::default().build_sans_config().execute_with(|| {
		assert_noop!(Staking::set_invulnerables(RuntimeOrigin::none(), vec![]), BadOrigin);
	})
}

#[test]
fn set_invulnerables_should_be_signed_by_root() {
	let controller = generate_account("not-root");

	ExtBuilder::<()>::default().build_sans_config().execute_with(|| {
		assert_noop!(
			Staking::set_invulnerables(RuntimeOrigin::signed(controller), vec![]),
			BadOrigin
		);
	})
}

#[test]
fn set_invulnerables_should_work() {
	let alex = generate_account("Alex");
	let bob = generate_account("Bob");

	ExtBuilder::<()>::default().build_sans_config().execute_with(|| {
		assert_eq!(Staking::invulnerables(), vec![]);

		assert_ok!(Staking::set_invulnerables(
			RuntimeOrigin::root(),
			vec![alex.clone(), bob.clone()]
		));
		assert_eq!(Staking::invulnerables(), vec![alex.clone(), bob.clone()]);

		// making another will overwrite the list of invulnerable accounts
		let charlie = generate_account("charlie");
		assert_ok!(Staking::set_invulnerables(RuntimeOrigin::root(), vec![charlie.clone()]));
		assert_eq!(Staking::invulnerables(), vec![charlie]);
	})
}

#[test]
fn set_staking_configs_should_be_signed() {
	ExtBuilder::<()>::default().build_sans_config().execute_with(|| {
		assert_noop!(
			Staking::set_staking_configs(RuntimeOrigin::none(), staking::ConfigOp::Noop),
			BadOrigin
		);
	})
}

#[test]
fn set_staking_configs_should_be_signed_by_root() {
	let controller = generate_account("not-root");

	ExtBuilder::<()>::default().build_sans_config().execute_with(|| {
		assert_noop!(
			Staking::set_staking_configs(
				RuntimeOrigin::signed(controller),
				staking::ConfigOp::Noop
			),
			BadOrigin
		);
	})
}

#[test]
fn set_staking_configs_works_with_noop() {
	ExtBuilder::<()>::default().build_sans_config().execute_with(|| {
		assert_eq!(staking::MinStakerBond::<Runtime>::get(), 0);

		assert_ok!(Staking::set_staking_configs(RuntimeOrigin::root(), staking::ConfigOp::Noop));

		assert_eq!(staking::MinStakerBond::<Runtime>::get(), 0);
	})
}

#[test]
fn set_staking_configs_works_with_set() {
	ExtBuilder::<()>::default().build_sans_config().execute_with(|| {
		assert_eq!(staking::MinStakerBond::<Runtime>::get(), 0);

		assert_ok!(Staking::set_staking_configs(RuntimeOrigin::root(), staking::ConfigOp::Set(9)));

		assert_eq!(staking::MinStakerBond::<Runtime>::get(), 9);
	})
}

#[test]
fn set_staking_configs_works_with_remove() {
	ExtBuilder::<()>::default().build_sans_config().execute_with(|| {
		// setup
		staking::MinStakerBond::<Runtime>::put(8);

		assert_ok!(Staking::set_staking_configs(RuntimeOrigin::root(), staking::ConfigOp::Remove));

		assert_eq!(staking::MinStakerBond::<Runtime>::get(), 0);
	})
}
