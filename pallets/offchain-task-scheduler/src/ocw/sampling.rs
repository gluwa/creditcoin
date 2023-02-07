use core::marker::PhantomData;
use frame_support::dispatch::Encode;
use frame_support::Parameter;
use frame_system::offchain::SigningTypes;
use frame_system::Config as SystemConfig;
use sp_core::sr25519::Public;
use sp_runtime::traits::Zero;
use sp_runtime::Perquintill;

pub trait SamplingSize {
	fn size() -> Perquintill;
}

pub trait Sampling {
	type Proof: Encode + Parameter;
	type Id: Encode;
	type Size: SamplingSize;
	type AccountId;

	fn sample(
		id: &Self::Id,
		pubkey: impl AsRef<Public>,
	) -> Option<Result<Self::Proof, Self::Proof>>;

	fn prove_sampled(id: &Self::Id, pubkey: &Self::AccountId, proof: Self::Proof) -> Option<bool>;
}

pub struct AlwaysSampled<T>(PhantomData<T>);

impl<T: SigningTypes + SystemConfig> Sampling for AlwaysSampled<T> {
	type Proof = ();
	type Id = T::Hash;
	type Size = ();
	type AccountId = T::AccountId;

	fn sample(_id: &Self::Id, _pubkey: impl AsRef<Public>) -> Option<Result<(), ()>> {
		Some(Ok(()))
	}

	fn prove_sampled(
		_id: &Self::Id,
		_pubkey: &Self::AccountId,
		_proof: Self::Proof,
	) -> Option<bool> {
		Some(true)
	}
}

impl SamplingSize for () {
	fn size() -> Perquintill {
		Zero::zero()
	}
}
