use crate::{
	mock::{RuntimeOrigin as Origin, *},
	next_difficulty, DifficultyAdjustmentPeriod, DifficultyAndTimestamp,
	PreviousDifficultiesAndTimestamps, TargetBlockTime, WeightInfo,
};
use frame_support::{assert_noop, assert_ok};
use pallet_timestamp::{self as timestamp};
use sp_runtime::traits::BadOrigin;

#[test]
fn set_target_block_time_should_error_when_not_signed() {
	new_test_ext().execute_with(|| {
		assert_noop!(Difficulty::set_target_block_time(Origin::none(), 12345), BadOrigin);
	});
}

#[test]
fn set_target_block_time_should_error_when_signed_by_non_root() {
	new_test_ext().execute_with(|| {
		assert_noop!(Difficulty::set_target_block_time(Origin::signed(0), 12345), BadOrigin);
	});
}

#[test]
fn set_target_block_time_should_error_when_target_time_is_zero() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			Difficulty::set_target_block_time(Origin::root(), 0),
			crate::Error::<Test>::ZeroTargetTime
		);
	});
}

#[test]
fn set_target_block_time_should_update_storage() {
	new_test_ext().execute_with(|| {
		assert_ok!(Difficulty::set_target_block_time(Origin::root(), 789),);

		let value = TargetBlockTime::<Test>::get();
		assert_eq!(value, 789);
	});
}

#[test]
fn set_adjustment_period_should_error_when_not_signed() {
	new_test_ext().execute_with(|| {
		assert_noop!(Difficulty::set_adjustment_period(Origin::none(), 12345), BadOrigin);
	});
}

#[test]
fn set_adjustment_period_should_error_when_signed_by_non_root() {
	new_test_ext().execute_with(|| {
		assert_noop!(Difficulty::set_adjustment_period(Origin::signed(0), 12345), BadOrigin);
	});
}

#[test]
fn set_adjustment_period_should_error_when_period_is_zero() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			Difficulty::set_adjustment_period(Origin::root(), 0),
			crate::Error::<Test>::ZeroAdjustmentPeriod
		);
	});
}

#[test]
fn set_adjustment_period_should_error_when_period_is_negative() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			Difficulty::set_adjustment_period(Origin::root(), -3),
			crate::Error::<Test>::NegativeAdjustmentPeriod
		);
	});
}

#[test]
fn set_adjustment_period_should_update_storage() {
	new_test_ext().execute_with(|| {
		assert_ok!(Difficulty::set_adjustment_period(Origin::root(), 95),);

		let value = DifficultyAdjustmentPeriod::<Test>::get();
		assert_eq!(value, 95);
	});
}

#[test]
fn next_difficulty_should_return_initial_when_previous_is_too_short() {
	new_test_ext().execute_with(|| {
		let target_time = TargetBlockTime::<Test>::get();
		let initial = Difficulty::difficulty();
		let adjustment_period = DifficultyAdjustmentPeriod::<Test>::get();
		let previous = PreviousDifficultiesAndTimestamps::<Test>::get();
		assert!(previous.len() < 2);

		let via_getter = crate::Pallet::<Test>::previous_difficulties_and_timestamps();
		assert_eq!(via_getter, previous);

		let result = next_difficulty(&previous, target_time, initial, adjustment_period);
		assert_eq!(result, initial);
	});
}

#[test]
fn next_difficulty_should_return_next_when_previous_is_configured() {
	new_test_ext().execute_with(|| {
		let crate_under_test = crate::GenesisConfig::<Test>::default();

		let mut previous = crate::Pallet::<Test>::previous_difficulties_and_timestamps();
		previous
			.try_push(DifficultyAndTimestamp {
				difficulty: crate_under_test.initial_difficulty,
				timestamp: 5678,
			})
			.unwrap();

		previous
			.try_push(DifficultyAndTimestamp {
				difficulty: crate_under_test.initial_difficulty,
				timestamp: 1234,
			})
			.unwrap();
		assert_eq!(previous.len(), 2);
		PreviousDifficultiesAndTimestamps::<Test>::put(previous.clone());

		let result = next_difficulty(
			&previous,
			1_000_000, // to avoid division by zero errors
			crate_under_test.initial_difficulty,
			crate_under_test.difficulty_adjustment_period,
		);
		assert_ne!(result, crate_under_test.initial_difficulty);
	});
}

#[test]
fn exercise_on_timestamp_set_when_previous_is_too_short() {
	new_test_ext().execute_with(|| {
		let previous = PreviousDifficultiesAndTimestamps::<Test>::get();
		assert!(previous.len() < 2);

		<timestamp::Pallet<Test>>::set(Origin::none(), 123456).unwrap();
	});
}

#[test]
fn exercise_on_timestamp_set_when_previous_is_configured() {
	new_test_ext().execute_with(|| {
		let crate_under_test = crate::GenesisConfig::<Test>::default();

		let mut previous = PreviousDifficultiesAndTimestamps::<Test>::get();
		previous
			.try_push(DifficultyAndTimestamp {
				difficulty: crate_under_test.initial_difficulty,
				timestamp: 5678,
			})
			.unwrap();

		previous
			.try_push(DifficultyAndTimestamp {
				difficulty: crate_under_test.initial_difficulty,
				timestamp: 1234,
			})
			.unwrap();
		assert_eq!(previous.len(), 2);
		PreviousDifficultiesAndTimestamps::<Test>::put(previous);

		TargetBlockTime::<Test>::put(60000); // ms
		DifficultyAdjustmentPeriod::<Test>::put(100i64);
		<timestamp::Pallet<Test>>::set(Origin::none(), 98765).unwrap();

		// just exercise the getter function
		let _ = crate::Pallet::<Test>::difficulty();
	});
}

#[test]
fn exercise_weightinfo_functions() {
	let result = super::weights::WeightInfo::<Test>::set_target_block_time();
	assert!(result.ref_time() > 0);

	let result = super::weights::WeightInfo::<Test>::set_adjustment_period();
	assert!(result.ref_time() > 0);
}
