use crate::{
	helpers::generate_external_address, types::OwnershipProof, Blockchain, Error, ExternalAddress,
};
use crate::{Address, AddressId, Addresses};
use base58::FromBase58;
use sp_core::ecdsa::Public;
use sp_io::crypto::secp256k1_ecdsa_recover_compressed;
use sp_io::hashing::{keccak_256, sha2_256};
use sp_runtime::BoundedVec;

/// The length of a bitcoin address in bytes
const BTC_MIN_LENGTH: usize = 25;

/// The length of an ethereu-like address in bytes
const ETH_ADDRESS_LENGTH: usize = 20;

/// ExternalAddressRegistrar is a generic type that allows users to crpyotgraphically verify ownership of off-chain addresses.
/// To register an external address you will need to supply an external address of variable length, specify a variant of the blockchain enumeration type, and supply a cryptographic proof that matches one of the supported signing methods.
pub trait ExternalAddressRegistrar {
	/// Check if the specified blockchain is supported by this registrar
	fn is_blockchain_supported(&self, blockchain: &Blockchain) -> bool;

	/// Check if the supplied external address is well-formed according to the rules specified by each blockchain.
	fn is_address_well_formed(&self, blockchain: &Blockchain, address: &ExternalAddress) -> bool;

	/// Verify a cryptographic proof that shows ownership of the specified external address and corresponding keypair
	fn verify_proof<T: crate::Config>(
		&self,
		proof: OwnershipProof,
		account_id: &[u8; 32],
		blockchain: &Blockchain,
		address: &ExternalAddress,
	) -> Option<crate::Error<T>>;

	/// Insert an address into onchain storage.
	fn insert_address<T: crate::Config>(
		&self,
		blockchain: &Blockchain,
		address: &ExternalAddress,
	) -> Option<crate::Error<T>>;
}

/// Default implementation of the ExternalAddressRegistrar. Designed to work with eth-like addresses using the older and insecure EthSign methodology as well as the newer and more secure PersonalSign.
#[derive(Default)]
pub struct Registrar {}

impl Registrar {
	pub fn is_blockchain_supported(&self, blockchain: &Blockchain) -> bool {
		match blockchain {
			Blockchain::Luniverse | Blockchain::Ethereum | Blockchain::Rinkeby => true,
			Blockchain::Bitcoin => false,
			Blockchain::Other(_) => false,
			_ => false,
		}
	}

	pub fn is_address_well_formed(
		&self,
		blockchain: &Blockchain,
		address: &ExternalAddress,
	) -> bool {
		match blockchain {
			Blockchain::Ethereum | Blockchain::Luniverse | Blockchain::Rinkeby => {
				eth_address_is_well_formed(address)
			},
			Blockchain::Bitcoin => btc_address_is_well_formed(address),
			Blockchain::Other(_) => false,
		}
	}

	pub fn verify_proof<T: crate::Config>(
		&self,
		proof: &OwnershipProof,
		account_id: &[u8],
		blockchain: &Blockchain,
		address: &ExternalAddress,
	) -> Option<crate::Error<T>> {
		let Some(response) = self.generate_response_proof(account_id, proof) else {
			return Some(Error::UnsupportedProofType)
		};

		let Some(public_key) = self.recover_public_key(proof.clone(), &response) else {
			return Some(Error::InvalidSignature);
		};

		let public_key = sp_core::ecdsa::Public::from_raw(public_key);
		let recreated_address = generate_external_address(blockchain, address, public_key);

		let Some(recreated_address) = recreated_address else {
			return Some(Error::EthSignExternalAddressGenerationFailed);
		};

		if &recreated_address != address {
			return Some(Error::OwnershipNotSatisfied);
		}

		None
	}

	fn recover_public_key(&self, proof: OwnershipProof, message: &[u8; 32]) -> Option<[u8; 33]> {
		match proof {
			OwnershipProof::Other => return None,
			OwnershipProof::EthSign(signature) | OwnershipProof::PersonalSign(signature) => {
				match secp256k1_ecdsa_recover_compressed(&signature.into(), &message) {
					Ok(public_key) => return Some(public_key),
					Err(_) => return None,
				}
			},
		}
	}

	fn generate_response_proof(
		&self,
		account_id: &[u8],
		challenge_proof: &OwnershipProof,
	) -> Option<[u8; 32]> {
		match challenge_proof {
			OwnershipProof::EthSign(_) => {
				Some(sp_io::hashing::blake2_256(sp_io::hashing::sha2_256(account_id).as_ref()))
			},
			OwnershipProof::PersonalSign(_) => {
				Some(eth_message(&sp_io::hashing::blake2_256(account_id)))
			},
			OwnershipProof::Other => None,
		}
	}

	pub fn insert_address<T: crate::Config>(
		&self,
		who: T::AccountId,
		blockchain: &Blockchain,
		address: &ExternalAddress,
	) -> Option<crate::Error<T>> {
		let address_id = AddressId::new::<T>(&blockchain, &address);

		if let Ok(account_id) = Addresses::<T>::try_get(&address_id) {
			if who == account_id.owner {
				return Some(Error::AddressAlreadyRegisteredByCaller);
			}

			return Some(Error::AddressAlreadyRegistered);
		}

		let entry = Address { blockchain: blockchain.clone(), value: address.clone(), owner: who };
		<Addresses<T>>::insert(address_id, entry);
		None
	}
}

fn btc_address_is_well_formed(address: &[u8]) -> bool {
	let address_str = if let Ok(s) = core::str::from_utf8(address) {
		s
	} else {
		return false;
	};

	// try to decode as bech32
	if bitcoin_bech32::WitnessProgram::from_address(address_str).is_ok() {
		return true;
	}

	// otherwise fall back to trying base58 check encoding
	let address_decoded = if let Ok(v) = address_str.from_base58() {
		v
	} else {
		return false;
	};

	if address_decoded.len() < BTC_MIN_LENGTH {
		return false;
	}

	let last4 = &address_decoded[address_decoded.len() - 4..];
	let hash = sha2_256(&sha2_256(&address_decoded[0..address_decoded.len() - 4]));
	let checksum = &hash[0..4];
	if last4 != checksum {
		return false;
	}

	true
}

// ether-like
fn eth_address_is_well_formed(address: &[u8]) -> bool {
	address.len() == ETH_ADDRESS_LENGTH
}

pub fn eth_message(message: &[u8; 32]) -> [u8; 32] {
	let mut bytes: Vec<u8> = vec![];
	let salt = b"\x19Ethereum Signed Message:\n32";

	bytes.extend_from_slice(salt);
	bytes.extend_from_slice(message);

	sp_io::hashing::keccak_256(&bytes)
}

pub trait PublicToAddress {
	type AddressType;
	fn try_extract_address_type(addr: &ExternalAddress) -> Option<Self::AddressType>;
	fn from_public(pkey: &Public) -> ExternalAddress;
}

pub struct EVMAddress;

impl PublicToAddress for EVMAddress {
	type AddressType = ();
	fn try_extract_address_type(addr: &ExternalAddress) -> Option<Self::AddressType> {
		if eth_address_is_well_formed(addr) {
			Some(())
		} else {
			None
		}
	}

	fn from_public(pkey: &Public) -> ExternalAddress {
		let pkey = libsecp256k1::PublicKey::parse_slice((*pkey).as_ref(), None)
			.expect("Public can't have invalid input length; qed")
			.serialize();
		//pkey uncompressed, 64 bytes
		let address_bytes = keccak_256(&pkey[1..])[12..].to_vec();
		BoundedVec::try_from(address_bytes).expect("20 bytes fit within bounds; qed")
	}
}

#[cfg(test)]
mod tests {
	use core::convert::{TryFrom, TryInto};
	use frame_support::BoundedVec;
	use sp_core::Pair;

	use super::*;

	#[test]
	fn eth_address_is_well_formed_works() {
		// length == 20
		assert!(eth_address_is_well_formed(
			hex::decode("5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed").unwrap().as_slice()
		));
		// length != 20
		assert!(!eth_address_is_well_formed(&[0u8; ETH_ADDRESS_LENGTH - 1][..]));
	}

	#[test]
	fn btc_address_is_well_formed_works() {
		// p2pkh
		assert!(btc_address_is_well_formed(b"1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa"));
		assert!(btc_address_is_well_formed(b"1BvBMSEYstWetqTFn5Au4m4GFg7xJaNVN2"));

		// bad checksums
		assert!(!btc_address_is_well_formed(b"1A1zP1eP5QGefi2DMPTfTL5SLmv7DiBEEF"));
		assert!(!btc_address_is_well_formed(b"1BvBMSEYstWetqTFn5Au4m4GFg7xJaBEEF"));

		// p2sh
		assert!(btc_address_is_well_formed(b"3J98t1WpEZ73CNmQviecrnyiWrnqRhWNLy"));
		assert!(btc_address_is_well_formed(b"3GRdnTq18LyNveWa1gQJcgp8qEnzijv5vR"));

		// bad checksums
		assert!(!btc_address_is_well_formed(b"3J98t1WpEZ73CNmQviecrnyiWrnqRhBEEF"));
		assert!(!btc_address_is_well_formed(b"3GRdnTq18LyNveWa1gQJcgp8qEnzijBEEF"));

		// p2wpkh/bech32
		assert!(btc_address_is_well_formed(b"bc1qnkyhslv83yyp0q0suxw0uj3lg9drgqq9c0auzc"));
		assert!(btc_address_is_well_formed(b"BC1QW508D6QEJXTDG4Y5R3ZARVARY0C5XW7KV8F3T4"));
		assert!(btc_address_is_well_formed(
			b"bc1pw508d6qejxtdg4y5r3zarvary0c5xw7kw508d6qejxtdg4y5r3zarvary0c5xw7kt5nd6y"
		));

		// bad checksums/invalid
		assert!(!btc_address_is_well_formed(b"bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t5"));
		assert!(!btc_address_is_well_formed(
			b"bc10w508d6qejxtdg4y5r3zarvary0c5xw7kw508d6qejxtdg4y5r3zarvary0c5xw7kw5rljs90"
		));
		assert!(!btc_address_is_well_formed(
			b"tc1p0xlxvlhemja6c4dqv22uapctqupfhlxm9h8z3k2e72q4k9hcz7vq5zuyut"
		));
	}

	#[test]
	fn btc_address_invalid_utf8() {
		assert!(!btc_address_is_well_formed(b"bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t5\xc3\x28"))
	}

	#[test]
	fn btc_address_too_short() {
		// successfully base58 decodes, but is too short
		assert!(!btc_address_is_well_formed(b"1A1zi2DMPTfTL5SLmv7DivfNa"))
	}

	#[test]
	fn address_is_well_formed_works() {
		let r = Registrar {};

		let ethereum = Blockchain::Ethereum;
		let bitcoin = Blockchain::Bitcoin;
		let rinkeby = Blockchain::Rinkeby;
		let luniverse = Blockchain::Luniverse;
		let other = Blockchain::Other(BoundedVec::try_from(b"other".to_vec()).unwrap());

		let eth_addr = hex::decode("5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed")
			.unwrap()
			.try_into()
			.unwrap();
		let btc_addr = b"1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa".to_vec().try_into().unwrap();

		assert!(r.is_address_well_formed(&ethereum, &eth_addr));
		assert!(r.is_address_well_formed(&rinkeby, &eth_addr));
		assert!(r.is_address_well_formed(&luniverse, &eth_addr));
		assert!(r.is_address_well_formed(&bitcoin, &btc_addr));
		assert!(!r.is_address_well_formed(&other, &eth_addr));
		assert!(!r.is_address_well_formed(&other, &btc_addr));
	}

	#[test]
	#[allow(non_snake_case)]
	fn EVMAddress_roundtrip() {
		let pair = sp_core::ecdsa::Pair::from_seed_slice(
			&hex::decode("9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60")
				.unwrap(),
		)
		.unwrap();
		let public = pair.public();
		assert_eq!(
			public,
			Public::from_full(
				&hex::decode("8db55b05db86c0b1786ca49f095d76344c9e6056b2f02701a7e7f3c20aabfd913ebbe148dd17c56551a52952371071a6c604b3f3abe8f2c8fa742158ea6dd7d4").unwrap()[..],
			).unwrap(),
		);

		let address = ExternalAddress::try_from(
			hex::decode("09231da7b19A016f9e576d23B16277062F4d46A8").unwrap(),
		)
		.unwrap();
		let address2 = EVMAddress::from_public(&public);
		assert!(address == address2);
	}

	#[test]
	fn test_registrar_is_blockchain_supported() {
		let r = Registrar {};

		assert_eq!(r.is_blockchain_supported(&Blockchain::Ethereum), true);
		assert_eq!(r.is_blockchain_supported(&Blockchain::Luniverse), true);
		assert_eq!(r.is_blockchain_supported(&Blockchain::Rinkeby), true);
		assert_eq!(r.is_blockchain_supported(&Blockchain::Bitcoin), false);
		assert_eq!(r.is_blockchain_supported(&Blockchain::Other(BoundedVec::default())), false);
	}
}
