use crate::mock::*;
use bstr::B;
use frame_support::{assert_noop, assert_ok, traits::Get, BoundedVec};

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

#[test]
fn register_address_basic() {
	ExtBuilder::default().build_and_execute(|| {
		let acct: AccountId = AccountId::new([0; 32]);
		let blockchain = B("testblockchain").into_bounded();
		let value = B("someaddressvalue").into_bounded();
		let network = B("testnetwork").into_bounded();
		assert_ok!(Creditcoin::register_address(
			Origin::signed(acct.clone()),
			blockchain.clone(),
			value.clone(),
			network.clone()
		));
		let address_id = crate::AddressId::new::<Test>(&blockchain, &value, &network);
		let address = crate::Address { blockchain, value, network, sighash: acct };

		assert_eq!(Creditcoin::addresses(address_id), Some(address));
	});
}

#[test]
fn register_address_pre_existing() {
	ExtBuilder::default().build_and_execute(|| {
		let acct: <Test as frame_system::Config>::AccountId = AccountId::new([0; 32]);
		let blockchain = B("testblockchain").into_bounded();
		let address = B("someaddressvalue").into_bounded();
		let network = B("testnetwork").into_bounded();
		assert_ok!(Creditcoin::register_address(
			Origin::signed(acct.clone()),
			blockchain.clone(),
			address.clone(),
			network.clone()
		));

		assert_noop!(
			Creditcoin::register_address(
				Origin::signed(acct.clone()),
				blockchain,
				address,
				network
			),
			crate::Error::<Test>::AddressAlreadyRegistered
		);
	})
}
