use crate::{Config, Pallet};
use pallet_offchain_task_scheduler::ocw::sampling::SamplingSize;
use sp_runtime::traits::Get;
use sp_runtime::Perquintill;

impl<T: Config> SamplingSize for Pallet<T> {
	fn size() -> sp_runtime::Perquintill {
		Pallet::<T>::sample_size()
	}
}

pub struct GetOne;

impl Get<Perquintill> for GetOne {
	fn get() -> Perquintill {
		Perquintill::one()
	}
}
