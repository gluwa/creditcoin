// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Tests for the module.

use crate::Pallet;
use frame_support::traits::Currency;

use pallet_balances::Pallet as Balances;
use pallet_session::Pallet as Session;
use pallet_staking_substrate::mock::*;
use pallet_staking_substrate::{EraRewardPoints, Payee, RewardDestination};
use sp_runtime::{assert_eq_error_rate, Perbill};
use sp_std::prelude::*;
use substrate_test_utils::assert_eq_uvec;

use super::Event;

#[test]
fn rewards_should_work() {
	ExtBuilder::default().nominate(true).session_per_era(3).build_and_execute(|| {
		let init_balance_11 = Balances::total_balance(&11);
		let init_balance_21 = Balances::total_balance(&21);
		let init_balance_101 = Balances::total_balance(&101);

		// Set payees
		Pallet::<Test>::insert(11, RewardDestination::Controller);
		Payee::<Test>::insert(21, RewardDestination::Controller);
		Payee::<Test>::insert(101, RewardDestination::Controller);

		Pallet::<Test>::reward_by_ids(vec![(11, 50)]);
		Pallet::<Test>::reward_by_ids(vec![(11, 50)]);
		// This is the second validator of the current elected set.
		Pallet::<Test>::reward_by_ids(vec![(21, 50)]);

		// Compute total payout now for whole duration of the session.
		let total_payout_0 = current_total_payout_for_duration(reward_time_per_era());
		let maximum_payout = maximum_payout_for_duration(reward_time_per_era());

		start_session(1);
		assert_eq_uvec!(Session::validators(), vec![11, 21]);

		assert_eq!(Balances::total_balance(&11), init_balance_11);
		assert_eq!(Balances::total_balance(&21), init_balance_21);
		assert_eq!(Balances::total_balance(&101), init_balance_101);
		assert_eq!(
			Pallet::eras_reward_points(active_era()),
			EraRewardPoints {
				total: 50 * 3,
				individual: vec![(11, 100), (21, 50)].into_iter().collect(),
			}
		);
		let part_for_11 = Perbill::from_rational::<u32>(1000, 1125);
		let part_for_21 = Perbill::from_rational::<u32>(1000, 1375);
		let part_for_101_from_11 = Perbill::from_rational::<u32>(125, 1125);
		let part_for_101_from_21 = Perbill::from_rational::<u32>(375, 1375);

		start_session(2);
		start_session(3);

		assert_eq!(active_era(), 1);
		assert_eq!(mock::RewardRemainderUnbalanced::get(), maximum_payout - total_payout_0,);
		assert_eq!(
			*mock::staking_events().last().unwrap(),
			Event::EraPaid {
				era_index: 0,
				validator_payout: total_payout_0,
				remainder: maximum_payout - total_payout_0
			}
		);
		mock::make_all_reward_payment(0);

		assert_eq_error_rate!(
			Balances::total_balance(&11),
			init_balance_11 + part_for_11 * total_payout_0 * 2 / 3,
			2,
		);
		assert_eq_error_rate!(
			Balances::total_balance(&21),
			init_balance_21 + part_for_21 * total_payout_0 * 1 / 3,
			2,
		);
		assert_eq_error_rate!(
			Balances::total_balance(&101),
			init_balance_101
				+ part_for_101_from_11 * total_payout_0 * 2 / 3
				+ part_for_101_from_21 * total_payout_0 * 1 / 3,
			2
		);

		assert_eq_uvec!(Session::validators(), vec![11, 21]);
		Pallet::<Test>::reward_by_ids(vec![(11, 1)]);

		// Compute total payout now for whole duration as other parameter won't change
		let total_payout_1 = current_total_payout_for_duration(reward_time_per_era());

		mock::start_active_era(2);
		assert_eq!(
			mock::RewardRemainderUnbalanced::get(),
			maximum_payout * 2 - total_payout_0 - total_payout_1,
		);
		assert_eq!(
			*mock::staking_events().last().unwrap(),
			Event::EraPaid {
				era_index: 1,
				validator_payout: total_payout_1,
				remainder: maximum_payout - total_payout_1
			}
		);
		mock::make_all_reward_payment(1);

		assert_eq_error_rate!(
			Balances::total_balance(&11),
			init_balance_11 + part_for_11 * (total_payout_0 * 2 / 3 + total_payout_1),
			2,
		);
		assert_eq_error_rate!(
			Balances::total_balance(&21),
			init_balance_21 + part_for_21 * total_payout_0 * 1 / 3,
			2,
		);
		assert_eq_error_rate!(
			Balances::total_balance(&101),
			init_balance_101
				+ part_for_101_from_11 * (total_payout_0 * 2 / 3 + total_payout_1)
				+ part_for_101_from_21 * total_payout_0 * 1 / 3,
			2
		);
	});
}
