use pallet_difficulty::Difficulty;
use parity_scale_codec::{Decode, Encode};
use rand::{prelude::SmallRng, SeedableRng};
use sc_consensus_pow::{Error, PowAlgorithm};
use sc_keystore::LocalKeystore;
use sha3::{Digest, Sha3_256};
use sp_api::{BlockId, BlockT, ProvideRuntimeApi};
use sp_consensus_pow::{DifficultyApi, Seal as RawSeal};
use sp_core::{H256, U256};
use std::sync::Arc;

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

pub trait GetDifficulty<B>
where
	B: BlockT<Hash = H256>,
{
	fn difficulty(&self, parent: B::Hash) -> Result<Difficulty, Error<B>>;
}

impl<B, C> GetDifficulty<B> for C
where
	B: BlockT<Hash = H256>,
	C: ProvideRuntimeApi<B>,
	C::Api: DifficultyApi<B, Difficulty>,
{
	fn difficulty(&self, parent: <B as BlockT>::Hash) -> Result<Difficulty, Error<B>> {
		let parent_id = BlockId::<B>::hash(parent);
		self.runtime_api().difficulty(&parent_id).map_err(|err| {
			sc_consensus_pow::Error::Environment(format!(
				"Fetching difficulty from runtime failed: {err:?}"
			))
		})
	}
}

fn verify<B: BlockT<Hash = H256>>(
	_parent: &BlockId<B>,
	pre_hash: &H256,
	_pre_digest: Option<&[u8]>,
	seal: &RawSeal,
	difficulty: Difficulty,
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
// Here we implement the general PowAlgorithm trait for our concrete Sha3Algorithm
impl<B: BlockT<Hash = H256>, C> PowAlgorithm<B> for Sha3Algorithm<C>
where
	C: GetDifficulty<B>,
{
	type Difficulty = Difficulty;

	fn difficulty(&self, parent: B::Hash) -> Result<Self::Difficulty, Error<B>> {
		self.client.difficulty(parent)
	}

	fn verify(
		&self,
		_parent: &BlockId<B>,
		pre_hash: &H256,
		_pre_digest: Option<&[u8]>,
		seal: &RawSeal,
		difficulty: Self::Difficulty,
	) -> Result<bool, Error<B>> {
		verify(_parent, pre_hash, _pre_digest, seal, difficulty)
	}
}

pub fn mine<B, C>(
	_client: &C,
	_keystore: &LocalKeystore,
	pre_hash: &H256,
	_pre_digest: Option<&[u8]>,
	difficulty: Difficulty,
) -> Result<Option<RawSeal>, Error<B>>
where
	B: BlockT<Hash = H256>,
	C: GetDifficulty<B>,
{
	let mut rng = SmallRng::from_rng(&mut rand::thread_rng()).map_err(|e| {
		sc_consensus_pow::Error::Environment(format!("Initialize RNG failed for mining: {e:?}"))
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

#[cfg(test)]
mod test {
	use super::Difficulty;
	use sc_keystore::LocalKeystore;
	use sp_core::H256;
	use sp_runtime::{testing::Block, OpaqueExtrinsic};
	use std::sync::Arc;

	use crate::*;
	use assert_matches::assert_matches;

	type TestBlock = Block<OpaqueExtrinsic>;

	#[derive(PartialEq, Debug, Clone)]
	struct MockDifficulty {
		value: Difficulty,
	}

	impl MockDifficulty {
		fn new(value: impl Into<Difficulty>) -> Self {
			Self { value: value.into() }
		}
	}

	impl GetDifficulty<TestBlock> for MockDifficulty {
		fn difficulty(
			&self,
			_parent: <TestBlock as sp_api::BlockT>::Hash,
		) -> Result<Difficulty, sc_consensus_pow::Error<TestBlock>> {
			Ok(self.value)
		}
	}

	#[test]
	fn mine_works() {
		let mock = MockDifficulty::new(1);
		let keystore = LocalKeystore::in_memory();
		let pre_hash = H256::default();

		let pre_digest = None;
		let difficulty = 1.into();
		let result = super::mine::<TestBlock, MockDifficulty>(
			&Arc::new(mock),
			&keystore,
			&pre_hash,
			pre_digest,
			difficulty,
		)
		.unwrap();

		assert!(result.is_some());
	}

	#[test]
	fn mine_should_return_none_when_hash_doesnt_meet_difficulty() {
		let mock = MockDifficulty::new(1);
		let keystore = LocalKeystore::in_memory();
		let pre_hash = H256::default();

		let pre_digest = None;
		let difficulty = Difficulty::MAX;
		let result = super::mine::<TestBlock, MockDifficulty>(
			&Arc::new(mock),
			&keystore,
			&pre_hash,
			pre_digest,
			difficulty,
		)
		.unwrap();

		assert!(result.is_none());
	}

	#[test]
	fn hash_meets_difficulty_should_return_false_when_product_overflows() {
		let hash = H256::repeat_byte(u8::MAX);
		let difficulty = Difficulty::MAX;

		let result = hash_meets_difficulty(&hash, difficulty);
		assert!(!result);
	}

	#[test]
	fn hash_meets_difficulty_should_return_true_when_product_doesnt_overflow() {
		let hash = H256::repeat_byte(1u8);
		let difficulty = Difficulty::zero();

		let result = hash_meets_difficulty(&hash, difficulty);
		assert!(result);
	}

	#[test]
	fn sha3algorithm_can_clone() {
		let mock = MockDifficulty::new(1);
		let algorithm = Sha3Algorithm::new(Arc::new(mock));

		let cloning = algorithm.clone();
		assert_eq!(cloning.client, algorithm.client);
	}

	#[test]
	fn sha3algorithm_can_return_difficulty() {
		let mock = MockDifficulty::new(2);
		let algorithm = Sha3Algorithm::new(Arc::new(mock));

		let difficulty = algorithm.difficulty(H256::default()).unwrap();
		assert_eq!(difficulty, U256::from(2));
	}

	#[test]
	fn sha3algorithm_verify_works() {
		let mock = MockDifficulty::new(1);
		let algorithm = Sha3Algorithm::new(Arc::new(mock.clone()));

		let pre_hash = H256::default();
		let nonce = H256::default();

		let compute = Compute { difficulty: mock.value, pre_hash, nonce };
		let seal = compute.compute();
		let raw_seal = Seal::encode(&seal);

		let result =
			algorithm.verify(&BlockId::Number(1), &pre_hash, Some(&[]), &raw_seal, mock.value);
		assert_matches!(result, Ok(true));
	}

	#[test]
	fn compute_should_return_a_seal() {
		let compute =
			Compute { difficulty: 1.into(), pre_hash: H256::default(), nonce: H256::default() };

		let result = compute.compute();
		assert_matches!(result, Seal{ difficulty, work: _, nonce} => {
			assert_eq!(difficulty, 1.into());
			assert_eq!(nonce, H256::default());
		});
	}

	#[test]
	fn verify_should_return_false_when_rawseal_cant_be_decoded() {
		let result = verify::<TestBlock>(
			&BlockId::Number(1),
			&H256::default(),
			Some(&[]),
			&vec![], // empty vector should not decode
			Difficulty::zero(),
		);

		assert_matches!(result, Ok(false));
	}

	#[test]
	fn verify_should_return_false_when_work_doesnt_meet_difficulty() {
		let seal = Seal {
			difficulty: 0.into(),
			work: H256::repeat_byte(u8::MAX), // compared to difficulty
			nonce: H256::zero(),
		};
		let raw_seal = Seal::encode(&seal);

		let result = verify::<TestBlock>(
			&BlockId::Number(1),
			&H256::default(),
			Some(&[]),
			&raw_seal,
			Difficulty::MAX, // compared to seal.work
		);

		assert_matches!(result, Ok(false));
	}

	#[test]
	fn verify_should_return_false_when_computed_seal_doesnt_match_arguments() {
		let seal = Seal {
			difficulty: 0.into(),
			work: H256::repeat_byte(1u8), // will not overflow * difficulty
			nonce: H256::zero(),
		};
		let raw_seal = Seal::encode(&seal);

		let result = verify::<TestBlock>(
			&BlockId::Number(1),
			&H256::default(), // will cause different computed seal
			Some(&[]),
			&raw_seal,
			Difficulty::zero(), // will not overflow * seal.work
		);

		assert_matches!(result, Ok(false));
	}

	#[test]
	fn verify_should_return_true_when_computed_seal_matches_arguments() {
		let difficulty: Difficulty = 1.into();
		let pre_hash = H256::default();
		let nonce = H256::default();

		let compute = Compute { difficulty, pre_hash, nonce };
		let seal = compute.compute();
		let raw_seal = Seal::encode(&seal);

		let result =
			verify::<TestBlock>(&BlockId::Number(1), &pre_hash, Some(&[]), &raw_seal, difficulty);

		assert_matches!(result, Ok(true));
	}
}
