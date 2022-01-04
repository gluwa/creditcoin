use crate::mock::*;
use bstr::B;
use frame_support::{assert_noop, assert_ok};

#[test]
fn register_address_basic() {
	new_test_ext().execute_with(|| {
		let acct: <Test as frame_system::Config>::AccountId = Default::default();
		let blockchain = B("testblockchain");
		let value = B("someaddressvalue");
		let network = B("testnetwork");
		assert_ok!(Creditcoin::register_address(
			Origin::signed(acct.clone()),
			blockchain.into(),
			value.into(),
			network.into()
		));
		let address_id = crate::AddressId::new::<Test>(blockchain, value, network);
		let address = crate::Address {
			blockchain: blockchain.into(),
			value: value.into(),
			network: network.into(),
			sighash: acct,
		};

		assert_eq!(Creditcoin::addresses(address_id), Some(address));
	})
}

#[test]
fn register_address_pre_existing() {
	new_test_ext().execute_with(|| {
		let acct: <Test as frame_system::Config>::AccountId = Default::default();
		let blockchain = B("testblockchain");
		let address = B("someaddressvalue");
		let network = B("testnetwork");
		assert_ok!(Creditcoin::register_address(
			Origin::signed(acct.clone()),
			blockchain.into(),
			address.into(),
			network.into()
		));

		assert_noop!(
			Creditcoin::register_address(
				Origin::signed(acct.clone()),
				blockchain.into(),
				address.into(),
				network.into()
			),
			crate::Error::<Test>::AddressAlreadyRegistered
		);
	})
}
