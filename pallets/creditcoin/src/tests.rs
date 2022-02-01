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
	new_test_ext().execute_with(|| {
		let acct: <Test as frame_system::Config>::AccountId = Default::default();
		let blockchain = B("testblockchain").into_bounded();
		let address_value = B("someaddressvalue").into_bounded();
		let network = B("testnetwork").into_bounded();
		assert_ok!(Creditcoin::register_address(
			Origin::signed(acct.clone()),
			blockchain.clone(),
			address_value.clone(),
			network.clone()
		));
		let address_id = crate::AddressId::new::<Test>(&blockchain, &address_value, &network);
		let address = crate::Address { blockchain, value: address_value, network, sighash: acct };

		assert_eq!(Creditcoin::addresses(address_id), Some(address));
	});
}

#[test]
fn register_address_pre_existing() {
	new_test_ext().execute_with(|| {
		let acct: <Test as frame_system::Config>::AccountId = Default::default();
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

#[test]
fn add_ask_order_basic() {
	new_test_ext().execute_with(|| {
		let acct: <Test as frame_system::Config>::AccountId = Default::default();
		let blockchain = B("testblockchain").into_bounded();
		let address = B("testaddresid").into_bounded();
		let amount = 100;
		let interest = 10;
		let fee = B("testfee").into_bounded();
		let guid = B("testguid").into_bounded();
		let expiration = B("testexpiration").into_bounded();
		let maturity = B("testmaturity").into_bounded();
		let block = B("testblock").into_bounded();

		assert_ok!(Creditcoin::add_ask_order(
			Origin::signed(acct.clone()),
			address.clone(),
			amount.clone(),
			interest.clone(),
			maturity.clone(),
			fee.clone(),
			expiration.clone(),
			guid.clone()
		));

		let ask_order_id = crate::AskOrderId::new::<Test>(&expiration, &guid);
		let ask_order = crate::AskOrder {
			blockchain,
			address,
			amount,
			interest,
			maturity,
			fee,
			expiration,
			block,
			sighash: acct,
		};

		assert_eq!(Creditcoin::ask_orders(ask_order_id), Some(ask_order));
	});
}
