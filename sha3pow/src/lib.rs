use parity_scale_codec::{Decode, Encode};
use rand::{prelude::SmallRng, SeedableRng};
use sc_client_api::{AuxStore, HeaderBackend};
use sc_consensus_pow::{Error, PowAlgorithm};
use sc_keystore::LocalKeystore;
use sha3::{Digest, Sha3_256};
use sp_api::{BlockId, BlockT, ProvideRuntimeApi};
use sp_consensus_pow::{DifficultyApi, Seal as RawSeal};
use sp_core::{H256, U256};
use std::sync::Arc;

pub mod app {
	use sp_application_crypto::{app_crypto, ecdsa};
	use sp_core::crypto::KeyTypeId;

	pub const ID: KeyTypeId = KeyTypeId(*b"ctcm");
	app_crypto!(ecdsa, ID);
}

pub type Difficulty = U256;

/// Determine whether the given hash satisfies the given difficulty.
/// The test is done by multiplying the two together. If the product
/// overflows the bounds of U256, then the product (and thus the hash)
/// was too high.
fn hash_meets_difficulty(hash: &H256, difficulty: Difficulty) -> bool {
	let num_hash = U256::from(&hash[..]);
	let (_, overflowed) = num_hash.overflowing_mul(difficulty);

	!overflowed
}

/// A Seal struct that will be encoded to a Vec<u8> as used as the
/// `RawSeal` type.
#[derive(Clone, PartialEq, Eq, Encode, Decode, Debug)]
pub struct Seal {
	pub difficulty: Difficulty,
	pub work: H256,
	pub nonce: H256,
}

/// A not-yet-computed attempt to solve the proof of work. Calling the
/// compute method will compute the hash and return the seal.
#[derive(Clone, PartialEq, Eq, Encode, Decode, Debug)]
pub struct Compute {
	pub difficulty: Difficulty,
	pub pre_hash: H256,
	pub nonce: H256,
}

impl Compute {
	pub fn compute(self) -> Seal {
		let work = H256::from_slice(Sha3_256::digest(&self.encode()[..]).as_slice());

		Seal { nonce: self.nonce, difficulty: self.difficulty, work }
	}
}

/// A complete PoW Algorithm that uses Sha3 hashing.
/// Needs a reference to the client so it can grab the difficulty from the runtime.
pub struct Sha3Algorithm<C> {
	client: Arc<C>,
}

impl<C> Sha3Algorithm<C> {
	pub fn new(client: Arc<C>) -> Self {
		Self { client }
	}
}

// Manually implement clone. Deriving doesn't work because
// it'll derive impl<C: Clone> Clone for Sha3Algorithm<C>. But C in practice isn't Clone.
impl<C> Clone for Sha3Algorithm<C> {
	fn clone(&self) -> Self {
		Self::new(self.client.clone())
	}
}

// Here we implement the general PowAlgorithm trait for our concrete Sha3Algorithm
impl<B: BlockT<Hash = H256>, C> PowAlgorithm<B> for Sha3Algorithm<C>
where
	C: ProvideRuntimeApi<B>,
	C::Api: DifficultyApi<B, Difficulty>,
{
	type Difficulty = Difficulty;

	fn difficulty(&self, parent: B::Hash) -> Result<Self::Difficulty, Error<B>> {
		let parent_id = BlockId::<B>::hash(parent);
		self.client.runtime_api().difficulty(&parent_id).map_err(|err| {
			sc_consensus_pow::Error::Environment(format!(
				"Fetching difficulty from runtime failed: {:?}",
				err
			))
		})
	}

	fn verify(
		&self,
		_parent: &BlockId<B>,
		pre_hash: &H256,
		_pre_digest: Option<&[u8]>,
		seal: &RawSeal,
		difficulty: Self::Difficulty,
	) -> Result<bool, Error<B>> {
		// Try to construct a seal object by decoding the raw seal given
		let seal = match Seal::decode(&mut &seal[..]) {
			Ok(seal) => seal,
			Err(_) => return Ok(false),
		};

		// See whether the hash meets the difficulty requirement. If not, fail fast.
		if !hash_meets_difficulty(&seal.work, difficulty) {
			return Ok(false);
		}

		// Make sure the provided work actually comes from the correct pre_hash
		let compute = Compute { difficulty, pre_hash: *pre_hash, nonce: seal.nonce };

		if compute.compute() != seal {
			return Ok(false);
		}

		Ok(true)
	}
}

pub fn mine<B, C>(
	_client: &C,
	_keystore: &LocalKeystore,
	pre_hash: &H256,
	_pre_digest: Option<&[u8]>,
	difficulty: Difficulty,
) -> Result<Option<RawSeal>, sc_consensus_pow::Error<B>>
where
	B: sp_api::BlockT<Hash = H256>,
	C: HeaderBackend<B> + AuxStore + ProvideRuntimeApi<B>,
{
	let mut rng = SmallRng::from_rng(&mut rand::thread_rng()).map_err(|e| {
		sc_consensus_pow::Error::Environment(format!("Initialize RNG failed for mining: {:?}", e))
	})?;

	let nonce = H256::random_using(&mut rng);
	let compute = Compute { difficulty, pre_hash: *pre_hash, nonce };

	let seal = compute.compute();

	if hash_meets_difficulty(&seal.work, difficulty) {
		Ok(Some(seal.encode()))
	} else {
		Ok(None)
	}
}
