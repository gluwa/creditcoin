#![cfg_attr(not(feature = "std"), no_std)]

use parity_scale_codec::Codec;
extern crate alloc;
use alloc::vec::Vec;

sp_api::decl_runtime_apis! {
	pub trait TaskApi<AccountId: Codec> {
		fn offchain_nonce_key(acc: &AccountId) -> Vec<u8>;
	}
}
