use crate::{
	address_registrar,
	pallet::{self, *},
};
use frame_support::fail;
use frame_system::{ensure_signed, pallet_prelude::OriginFor};
use sp_runtime::DispatchResult;

use crate::{context::Context, types::OwnershipProof, Blockchain, ExternalAddress};

pub fn register_address_v2<T: Config>(
	ctx: &impl Context,
	origin: OriginFor<T>,
	blockchain: Blockchain,
	address: ExternalAddress,
	ownership_proof: OwnershipProof,
) -> DispatchResult {
	let who = ensure_signed(origin)?;
	let registrar: address_registrar::Registrar = ctx.registrar();

	if !registrar.is_blockchain_supported(&blockchain) {
		fail!(Error::<T>::UnsupportedBlockchain);
	}

	if !registrar.is_address_well_formed(&blockchain, &address) {
		fail!(Error::<T>::MalformedExternalAddress);
	}

	{
		let encoded = who.encode();
		let account = encoded.as_slice();

		if let Some(error) =
			registrar.verify_proof::<T>(&ownership_proof, account, &blockchain, &address)
		{
			fail!(error);
		};
	}

	if let Some(error) = registrar.insert_address::<T>(who.clone(), &blockchain, &address) {
		fail!(error);
	}

	Ok(())
}
