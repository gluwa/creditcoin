use crate::logger;
use crate::Config;
use crate::Pallet as Staking;
use core::marker::PhantomData;
use frame_support::traits::Defensive;
use frame_system::offchain::{AppCrypto, SigningTypes};
use frame_system::Config as SystemConfig;
use frame_system::Pallet as System;
use pallet_offchain_task_scheduler::{
	ocw::sampling::{Sampling, SamplingSize},
	Config as TaskSchedulerConfig,
};
use pallet_task_voting::{Config as TaskVotingConfig, Pallet as TaskVoting};
use primitives::vrf::finalize_randomness;
use primitives::vrf::prove_vrf;
use primitives::vrf::sortition::{is_selected, threshold};
use primitives::vrf::PublicKey;
use primitives::vrf::{make_transcript, vrf::generate_vrf, VRFOutput, VRFProof};
use sp_core::{sr25519::Public, H256};
use sp_runtime::traits::IdentifyAccount;
use sp_runtime::traits::UniqueSaturatedInto;
use sp_runtime::traits::Zero;
use sp_runtime::RuntimeAppPublic;

pub struct RandomSampling<T>(PhantomData<T>);

impl<T: TaskSchedulerConfig + SystemConfig + SigningTypes + Config + TaskVotingConfig> Sampling
	for RandomSampling<T>
where
	T::Hash: Into<H256>,
	T::Public: From<Public>,
	T::AccountId: AsRef<[u8]>,
{
	type Proof = (VRFOutput, VRFProof);
	type Id = T::Hash;
	type Size = TaskVoting<T>;
	type AccountId = T::AccountId;

	fn sample(
		id: &Self::Id,
		pubkey: impl AsRef<Public>,
	) -> Option<Result<Self::Proof, Self::Proof>> {
		let pre_hash = System::<T>::parent_hash().into();
		let key_type_id =
			<T::AuthorityId as AppCrypto<T::Public, T::Signature>>::RuntimeAppPublic::ID;
		let task_id = (*id).into();

		let (output, proof) = generate_vrf(key_type_id, pubkey.as_ref(), pre_hash, 0, task_id)
			.defensive_proof("Failed to generate vrf Proof")?;

		let transcript = make_transcript(0, pre_hash, task_id);

		let seed = {
			let pubkey = PublicKey::from_bytes(pubkey.as_ref())
				.expect("Sr25519::Public is Schnorr25519; qed");

			finalize_randomness(&pubkey, transcript, &output)
				.defensive_proof("Failed to finalize vrf")?
		};

		let threshold = {
			let acc: T::AccountId = T::Public::from(*pubkey.as_ref()).into_account();
			let Some(stake) =
			Staking::<T>::ledger(&acc).filter(|ledger | ledger.active >Zero::zero())  else {
				logger!(debug, "{:?} Cannot sample inactive controller", acc);
				return None;
			};

			let total_stake = {
				let era_info = Staking::<T>::active_era().defensive_proof("Active era not set")?;
				Staking::<T>::eras_total_stake(era_info.index)
			};
			let sample_size = Self::Size::size();

			threshold(
				sample_size,
				stake.active.unique_saturated_into(),
				total_stake.unique_saturated_into(),
			)
		};

		let sampled =
			if is_selected(&seed, threshold) { Ok((output, proof)) } else { Err((output, proof)) };
		Some(sampled)
	}

	fn prove_sampled(
		id: &Self::Id,
		account_id: &Self::AccountId,
		proof: Self::Proof,
	) -> Option<bool> {
		let pre_hash = System::<T>::parent_hash().into();
		let task_id = (*id).into();

		let seed = {
			let pubkey = PublicKey::from_bytes(account_id.as_ref())
				.expect("Sr25519::Public is Schnorr25519; qed");
			prove_vrf(pubkey, pre_hash, 0, task_id, proof.0, proof.1)
				.map_err(|e| {
					log::debug!(" {e:?}: Failed to prove {id:?} for {pubkey:?}");
				})
				.ok()?
		};

		let threshold = {
			let stake = Staking::<T>::ledger(&account_id)
				.filter(|ledger| ledger.active > Zero::zero())
				.defensive_proof("{account_id:?} Cannot sample inactive controller")?;
			let total_stake = {
				let era_info = Staking::<T>::active_era().defensive_proof("Active era not set")?;
				Staking::<T>::eras_total_stake(era_info.index)
			};
			let sample_size = Self::Size::size();

			threshold(
				sample_size,
				stake.active.unique_saturated_into(),
				total_stake.unique_saturated_into(),
			)
		};

		if is_selected(&seed, threshold) {
			Some(true)
		} else {
			Some(false)
		}
	}
}
