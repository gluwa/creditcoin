///! give back fees
use frame_support::{
	traits::{Currency, ExistenceRequirement, Imbalance, IsType, OnUnbalanced, WithdrawReasons},
	unsigned::TransactionValidityError,
};
use frame_system::Config as SystemConfig;
use pallet_creditcoin::Config as CreditcoinConfig;
use pallet_transaction_payment::Config as TxPaymentConfig;
use pallet_transaction_payment::OnChargeTransaction;
use sp_runtime::{
	traits::{BlockNumberProvider, Saturating, UniqueSaturatedFrom, Zero},
	transaction_validity::InvalidTransaction,
};
use sp_std::marker::PhantomData;

const BLOCK_HOUR: u32 = 60;
const BLOCK_YEAR: u32 = BLOCK_HOUR * 24 * 365 + 6 * BLOCK_HOUR;

pub struct CurrencyFeeRedemptionAdapter<C, OU>(PhantomData<(C, OU)>);

impl<C, OU> CurrencyFeeRedemptionAdapter<C, OU> {
	fn bucketed_year_offset<T: SystemConfig>(at: &T::BlockNumber) -> T::BlockNumber {
		let base = at.saturating_add(BLOCK_YEAR.into());
		base - base % UniqueSaturatedFrom::unique_saturated_from(10u32)
	}
}

impl<T, C, OU> OnChargeTransaction<T> for CurrencyFeeRedemptionAdapter<C, OU>
where
	T: CreditcoinConfig + TxPaymentConfig,
	C: Currency<<T as SystemConfig>::AccountId>,
	C::Balance: IsType<T::Balance>,
	C::PositiveImbalance: Imbalance<C::Balance, Opposite = C::NegativeImbalance>,
	C::NegativeImbalance: Imbalance<C::Balance, Opposite = C::PositiveImbalance>,
	OU: OnUnbalanced<C::NegativeImbalance>,
{
	type Balance = C::Balance;
	type LiquidityInfo = Option<C::NegativeImbalance>;

	/// Note: The `fee` already includes the `tip`.
	fn withdraw_fee(
		who: &<T>::AccountId,
		_call: &<T as SystemConfig>::Call,
		_dispatch_info: &sp_runtime::traits::DispatchInfoOf<<T as SystemConfig>::Call>,
		fee: Self::Balance,
		tip: Self::Balance,
	) -> Result<Self::LiquidityInfo, TransactionValidityError> {
		if fee.is_zero() {
			return Ok(None);
		}

		let withdraw_reason = if tip.is_zero() {
			WithdrawReasons::TRANSACTION_PAYMENT
		} else {
			WithdrawReasons::TRANSACTION_PAYMENT | WithdrawReasons::TIP
		};

		match C::withdraw(who, fee, withdraw_reason, ExistenceRequirement::KeepAlive) {
			Ok(imbalance) => Ok(Some(imbalance)),
			Err(_) => Err(InvalidTransaction::Payment.into()),
		}
	}

	fn correct_and_deposit_fee(
		who: &<T>::AccountId,
		_dispatch_info: &sp_runtime::traits::DispatchInfoOf<<T as SystemConfig>::Call>,
		_post_info: &sp_runtime::traits::PostDispatchInfoOf<<T as SystemConfig>::Call>,
		corrected_fee: Self::Balance,
		tip: Self::Balance,
		already_withdrawn: Self::LiquidityInfo,
	) -> Result<(), TransactionValidityError> {
		if let Some(paid) = already_withdrawn {
			let refund_amount = paid.peek().saturating_sub(corrected_fee);
			let refund_imbalance = C::deposit_into_existing(who, refund_amount)
				.unwrap_or_else(|_| C::PositiveImbalance::zero());
			let adjusted_paid = paid
				.offset(refund_imbalance)
				.same()
				.map_err(|_| TransactionValidityError::Invalid(InvalidTransaction::Payment))?;
			let (tip, fee) = adjusted_paid.split(tip);

			if !fee.peek().is_zero() {
				let bn = {
					let at = frame_system::Pallet::<T>::current_block_number();
					Self::bucketed_year_offset::<T>(&at)
				};
				let amount = fee.peek().into();
				pallet_creditcoin::RetainedFees::<T>::mutate(bn, who, |acc| {
					if let Some(acc) = acc {
						acc.saturating_accrue(amount);
					} else {
						*acc = Some(amount);
					}
				});
			}

			OU::on_unbalanceds(Some(fee).into_iter().chain(Some(tip)));
		}
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{Balances, Creditcoin, ExistentialDeposit, Runtime, System};
	use assert_matches::assert_matches;
	use frame_support::{
		traits::Hooks,
		weights::{DispatchInfo, PostDispatchInfo},
	};
	use pallet_balances::NegativeImbalance;
	use sp_core::Pair;
	use sp_runtime::{traits::IdentifyAccount, AccountId32, MultiSigner};
	use std::default::Default;

	fn generate_account(seed: &str) -> AccountId32 {
		let seed = seed.bytes().cycle().take(32).collect::<Vec<_>>();
		let key_pair = sp_core::ecdsa::Pair::from_seed_slice(seed.as_slice()).unwrap();
		let pkey = key_pair.public();
		let signer: MultiSigner = pkey.into();
		signer.into_account()
	}

	fn existential_deposit() -> u128 {
		ExistentialDeposit::get()
	}
	struct ExtBuilder;

	impl ExtBuilder {
		pub fn build() -> sp_io::TestExternalities {
			let mut t = frame_system::GenesisConfig::default().build_storage::<Runtime>().unwrap();
			// hacky, without enough balance to keep the account alive, deposit_into_existing will no-op.
			pallet_balances::GenesisConfig::<Runtime> {
				balances: vec![(generate_account("Somebody"), ExistentialDeposit::get())],
			}
			.assimilate_storage(&mut t)
			.unwrap();
			t.into()
		}
	}
	fn correct_and_deposit_fee_with_passing_defaults(
		who: &AccountId32,
		already_withdrawn: u128,
		corrected_fee: u128,
	) {
		let dispatch_info = DispatchInfo::default();
		let post_info = PostDispatchInfo::default();
		let tip = Default::default();
		let already_withdrawn = Some(NegativeImbalance::new(already_withdrawn));

		<<Runtime as TxPaymentConfig>::OnChargeTransaction as OnChargeTransaction<
					Runtime,
				>>::correct_and_deposit_fee(
					who,
					&dispatch_info,
					&post_info,
					corrected_fee,
					tip,
					already_withdrawn,
				)
				.unwrap();
	}

	#[test]
	fn year_offset() {
		let test = || {
			let f = |i: u32| {
				CurrencyFeeRedemptionAdapter::<pallet_balances::Pallet<Runtime>,()>::bucketed_year_offset::<Runtime>(&i)
			};
			assert_eq!(BLOCK_YEAR, f(0));
			assert_eq!(BLOCK_YEAR, f(9));
			assert_eq!(BLOCK_YEAR + 10, f(10));
			assert_eq!(BLOCK_YEAR + 20, f(20));
			assert_eq!(BLOCK_YEAR + 90, f(99));
			assert_eq!(BLOCK_YEAR + 100, f(100));
		};
		ExtBuilder::build().execute_with(test);
	}

	#[test]
	fn fee_adapter_works() {
		let test = || {
			let acc = generate_account("Somebody");

			//offset fees by a block-year
			let year_offset = CurrencyFeeRedemptionAdapter::<pallet_balances::Pallet<Runtime>,()>::bucketed_year_offset::<Runtime>(&0);

			// Adapter no-ops if adjusted_fees is 0
			correct_and_deposit_fee_with_passing_defaults(&acc, 2, 0);
			assert!(
				pallet_creditcoin::RetainedFees::<Runtime>::get(&year_offset, &acc).is_none(),
				"0-fees are no-ops"
			);

			//Adapter inserts if fees > 0
			correct_and_deposit_fee_with_passing_defaults(&acc, 2, 1);
			let fee = pallet_creditcoin::RetainedFees::<Runtime>::get(&year_offset, &acc)
				.expect("fees inserted for redemption");
			assert_eq!(fee, 1u128);

			//Adapter aggregates
			correct_and_deposit_fee_with_passing_defaults(&acc, 2, 1);
			let fee = pallet_creditcoin::RetainedFees::<Runtime>::get(&year_offset, &acc)
				.expect("fees inserted for redemption");
			assert_eq!(fee, 2u128);
		};

		ExtBuilder::build().execute_with(test);
	}

	#[test]
	fn creditcoin_on_init_redeems() {
		let test = || {
			let acc = generate_account("Somebody");
			System::set_block_number(1);
			//offset fees by a block-year
			let year_offset = CurrencyFeeRedemptionAdapter::<pallet_balances::Pallet<Runtime>,()>::bucketed_year_offset::<Runtime>(&0);

			correct_and_deposit_fee_with_passing_defaults(&acc, 2, 1);

			let free = Balances::free_balance(&acc);
			assert_eq!(free, existential_deposit() + 1u128);
			//Give back excess fees worth one.
			Creditcoin::on_initialize(year_offset);
			let free = Balances::free_balance(&acc);
			assert_eq!(free, existential_deposit() + 2u128);

			let event = System::events().pop().expect("FeeRedemption").event;

			assert_matches!(
				event,
				crate::Event::Creditcoin(pallet_creditcoin::Event::<Runtime>::FeeRedemption(bn, account,amount)) => {
					assert_eq!(bn, year_offset);
					assert_eq!(account, acc);
					assert_eq!(amount, 1u128);
				}
			);
		};
		ExtBuilder::build().execute_with(test);
	}
}
