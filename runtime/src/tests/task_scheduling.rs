use crate::mock::runtime::{Balances, Runtime, RuntimeOrigin, Staking, TaskScheduler};
use crate::KeyTypeId;
use frame_support::{
	assert_ok,
	traits::{tokens::currency::Currency, GenesisBuild},
};
use pallet_balances::GenesisConfig as BalancesGenesisConfig;
use pallet_staking_substrate::{GenesisConfig, StakerStatus};
use runtime_utils::{ExtBuilder, RollTo, SyncCryptoStore, WithWorkerHook};
use sp_staking::StakingInterface;

fn generate_bonded_stash(
	builder: &mut ExtBuilder,
) -> (sp_core::sr25519::Public, sp_core::sr25519::Public) {
	let keystore_id = pallet_offchain_task_scheduler::KEY_TYPE;
	let keystore = builder.keystore.as_ref().expect("A keystore");
	let stash_pubkey = keystore.sr25519_generate_new(keystore_id, None).unwrap();
	let controller_pubkey = keystore.sr25519_generate_new(keystore_id, None).unwrap();

	let genesis_config = BalancesGenesisConfig::<Runtime> {
		balances: vec![(stash_pubkey.into(), Balances::minimum_balance())],
	};

	genesis_config.assimilate_storage(&mut builder.storage).unwrap();

	let genesis_config = GenesisConfig::<Runtime> {
		stakers: vec![(
			stash_pubkey.into(),
			controller_pubkey.into(),
			Balances::minimum_balance(),
			StakerStatus::Idle,
		)],
		validator_count: 1,
		..Default::default()
	};

	genesis_config.assimilate_storage(&mut builder.storage).unwrap();

	(stash_pubkey, controller_pubkey)
}

#[test]
fn scheduler_doesnt_run_if_pubkey_cant_sign() {
	let logs = traced_test::trace();
	// for example see
	// https://github.com/gluwa/creditcoin-authority-manager/blob/43c61aa177146e4ecfde944678db851b941d4ae6/src/main.rs#L245-L265
	let builder = ExtBuilder::default().with_keystore();

	let _pubkey = builder
		.keystore
		.as_ref()
		.expect("A keystore")
		// keys where KeyID is not "gots" can't be used for OTS tasks
		.sr25519_generate_new(KeyTypeId(*b"ctcs"), Some("//CanNotSign"))
		.unwrap();

	builder.build_sans_config().execute_with(|| {
		WithWorkerHook::<TaskScheduler, Runtime>::roll_to(1);

		assert!(logs.contain("local keys []"));
		assert!(logs.contain("Not an authority, skipping offchain work"));
	});
}

#[test]
/// Needs improvement, add a pubkey to the keystore but dont bond it. Bond an account but dont add it to the keystore.
fn scheduler_doesnt_run_if_not_staked_or_cant_sign() {
	let logs = traced_test::trace();
	ExtBuilder::default().with_keystore().build_sans_config().execute_with(|| {
		WithWorkerHook::<TaskScheduler, Runtime>::roll_to(1);

		assert!(logs.contain("Not an authority, skipping offchain work"));
	});
}

#[test]
fn scheduler_runs_if_staked_and_can_sign() {
	let logs = traced_test::trace();
	let mut builder = ExtBuilder::default().with_keystore();

	//Can sign and is a staked controller; proceed
	let (_, _) = generate_bonded_stash(&mut builder);

	builder.build::<Runtime>().execute_with(|| {
		WithWorkerHook::<TaskScheduler, Runtime>::roll_to(1);

		assert!(!logs.contain("Not an authority, skipping offchain work"));
	});
}

#[test]
fn scheduler_doesnt_run_without_active_stake() {
	let logs = traced_test::trace();
	let mut builder = ExtBuilder::default().with_keystore();

	//Can sign and is a staked controller; proceed
	let (stash, controller) = generate_bonded_stash(&mut builder);

	builder.build::<Runtime>().execute_with(|| {
		assert_eq!(Staking::active_stake(&stash.into()).unwrap(), Balances::minimum_balance());

		WithWorkerHook::<TaskScheduler, Runtime>::roll_to(1);

		assert!(!logs.contain("Not an authority, skipping offchain work"));

		assert_ok!(Staking::unbond(
			RuntimeOrigin::signed(controller.into()),
			Balances::minimum_balance()
		));

		WithWorkerHook::<TaskScheduler, Runtime>::roll_to(2);

		assert!(logs.contain("Not an authority, skipping offchain work"));
	});
}
