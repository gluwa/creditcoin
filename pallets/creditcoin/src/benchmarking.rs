//! Benchmarking setup for pallet-template

use super::*;

#[allow(unused)]
use crate::Pallet as Creditcoin;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;
use frame_system::RawOrigin;

#[extend::ext]
impl<'a, S> &'a [u8]
where
	S: Get<u32>,
{
	fn try_into_bounded(self) -> Result<BoundedVec<u8, S>, ()> {
		core::convert::TryFrom::try_from(self.to_vec())
	}
	fn into_bounded(self) -> BoundedVec<u8, S> {
		core::convert::TryFrom::try_from(self.to_vec()).unwrap()
	}
}

benchmarks! {
	register_address {
		let caller: T::AccountId = whitelisted_caller();
		let b in 0..256;
		let e in 0..256;
		let n in 0..256;
		let blockchain = vec![b'b'; b as usize];
		let external_address = vec![b'a'; e as usize];
		let network = vec![b'c'; n as usize];
	}: _(RawOrigin::Signed(caller), Blockchain::Other(blockchain.into_bounded()), external_address.into_bounded())
}

impl_benchmark_test_suite!(Creditcoin, crate::mock::new_test_ext(), crate::mock::Test);
