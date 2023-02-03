use crate::mock::runtime::{Balances, Runtime, RuntimeOrigin, Staking, TaskScheduler};
use frame_support::{
	assert_ok,
	traits::{tokens::currency::Currency, GenesisBuild},
};
use pallet_balances::GenesisConfig as BalancesGenesisConfig;
use pallet_staking::StakingInterface;
use pallet_staking::{GenesisConfig, StakerStatus};
use runtime_utils::{ExtBuilder, RollTo, SyncCryptoStore, WithWorkerHook};

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
#[tracing_test::traced_test]
/// Needs improvement, add a pubkey to the keystore but dont bond it. Bond an account but dont add it to the keystore.
fn scheduler_doesnt_run_if_not_staked_or_cant_sign() {
	ExtBuilder::default().with_keystore().build_sans_config().execute_with(|| {
		WithWorkerHook::<TaskScheduler, Runtime>::roll_to(1);

		assert!(logs_contain("Not an authority, skipping offchain work"));
	});
}

#[test]
#[tracing_test::traced_test]
fn scheduler_runs_if_staked_and_can_sign() {
	let mut builder = ExtBuilder::default().with_keystore();

	//Can sign and is a staked controller; proceed
	let (_, _) = generate_bonded_stash(&mut builder);

	builder.build::<Runtime>().execute_with(|| {
		WithWorkerHook::<TaskScheduler, Runtime>::roll_to(1);

		assert!(!logs_contain("Not an authority, skipping offchain work"));
	});
}

#[test]
#[tracing_test::traced_test]
fn scheduler_doesnt_run_without_active_stake() {
	let mut builder = ExtBuilder::default().with_keystore();

	//Can sign and is a staked controller; proceed
	let (_, controller) = generate_bonded_stash(&mut builder);

	builder.build::<Runtime>().execute_with(|| {
		assert_eq!(Staking::active_stake(&controller.into()).unwrap(), Balances::minimum_balance());

		WithWorkerHook::<TaskScheduler, Runtime>::roll_to(1);

		assert!(!logs_contain("Not an authority, skipping offchain work"));

		assert_ok!(Staking::unbond(
			RuntimeOrigin::signed(controller.into()),
			Balances::minimum_balance()
		));

		WithWorkerHook::<TaskScheduler, Runtime>::roll_to(2);

		assert!(logs_contain("Not an authority, skipping offchain work"));
	});
}
