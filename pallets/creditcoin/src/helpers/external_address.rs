use crate::{Blockchain, ExternalAddress};
use alloc::string::String;
use base58::FromBase58;
use core::convert::TryFrom;
use frame_support::BoundedVec;
use sp_core::ecdsa::Public;
use sp_io::hashing::keccak_256;
use sp_io::hashing::sha2_256;
use sp_std::boxed::Box;
use sp_std::vec::Vec;

pub fn external_address_generator(
	blockchain: &Blockchain,
	reference: &ExternalAddress,
) -> Option<Box<dyn Fn(Public) -> ExternalAddress>> {
	match blockchain {
		Blockchain::Luniverse | Blockchain::Ethereum | Blockchain::Rinkeby => {
			match Etherlike::is_address(reference) {
				Some(Etherlike::Simple) => Some(Box::new(Etherlike::from_public)),
				Some(Etherlike::Eip55) => Some(Box::new(Etherlike::from_public_checksummed)),
				_ => None,
			}
		},
		Blockchain::Bitcoin => None,
		Blockchain::Other(_) => None,
	}
}

pub trait PublictoAddress {
	type AddressType;
	fn is_address(addr: &ExternalAddress) -> Option<Self::AddressType>;
	fn from_public(pkey: Public) -> ExternalAddress;
}

pub enum Etherlike {
	Simple,
	Eip55,
}

impl PublictoAddress for Etherlike {
	type AddressType = Self;
	///Needs tests!
	fn is_address(addr: &ExternalAddress) -> Option<Self::AddressType> {
		if !eth_address_is_well_formed(addr) {
			return None;
		}

		for b in hex::encode(addr).as_bytes() {
			if b.is_ascii_uppercase() {
				return Some(Self::Eip55);
			}
		}
		Some(Self::Simple)
	}

	fn from_public(pkey: Public) -> ExternalAddress {
		let pkey = libsecp256k1::PublicKey::parse_slice(pkey.as_ref(), None)
			.expect("Public can't have invalid input length; qed")
			.serialize();
		//pkey uncompressed, 64 bytes
		let a = keccak_256(&pkey[1..])[12..].to_vec();
		BoundedVec::try_from(a).expect("20 bytes fit within bounds; qed")
	}
}

impl Etherlike {
	///https://github.com/ethereum/EIPs/blob/master/EIPS/eip-55.md
	/// Takes a 20-byte binary address as input
	pub fn to_checksum_address(lowercase_addr: ExternalAddress) -> ExternalAddress {
		let lowercase_addr = hex::encode(lowercase_addr);

		let byte_digest = {
			//Treat the lowercase hex address as ascii/utf-8 for keccak256 hashing
			let digest = keccak_256(lowercase_addr.as_bytes());
			hex::encode(digest)
		};
		let z = lowercase_addr.as_bytes().iter().zip(byte_digest.as_bytes().iter());
		let transform = z
			.map(
				|(&h, &m)| {
					if h.is_ascii_alphabetic() && m >= b"8"[0] {
						h.to_ascii_uppercase()
					} else {
						h
					}
				},
			)
			.collect::<Vec<_>>();

		let x = String::from_utf8(transform).unwrap();
		let x = hex::decode(x).unwrap();

		ExternalAddress::try_from(x).unwrap()
	}

	pub fn from_public_checksummed(pkey: Public) -> ExternalAddress {
		Self::to_checksum_address(Self::from_public(pkey))
	}
}

pub fn address_is_well_formed(blockchain: &Blockchain, address: &ExternalAddress) -> bool {
	match blockchain {
		Blockchain::Bitcoin => btc_address_is_well_formed(address),
		Blockchain::Ethereum | Blockchain::Luniverse | Blockchain::Rinkeby => {
			eth_address_is_well_formed(address)
		},
		Blockchain::Other(_) => false,
	}
}

// bitcoin

const BTC_MIN_LENGTH: usize = 25;

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

const ETH_ADDRESS_LENGTH: usize = 20;

fn eth_address_is_well_formed(address: &[u8]) -> bool {
	address.len() == ETH_ADDRESS_LENGTH
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

		assert!(address_is_well_formed(&ethereum, &eth_addr));
		assert!(address_is_well_formed(&rinkeby, &eth_addr));
		assert!(address_is_well_formed(&luniverse, &eth_addr));
		assert!(address_is_well_formed(&bitcoin, &btc_addr));
		assert!(!address_is_well_formed(&other, &eth_addr));
		assert!(!address_is_well_formed(&other, &btc_addr));
	}

	#[test]
	fn etherlike_roundtrip() {
		let pair = sp_core::ecdsa::Pair::from_seed_slice(
			&hex::decode("9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60")
				.unwrap(),
		)
		.unwrap();
		let public = pair.public();
		assert_eq!(
			public.clone(),
			Public::from_full(
				&hex::decode("8db55b05db86c0b1786ca49f095d76344c9e6056b2f02701a7e7f3c20aabfd913ebbe148dd17c56551a52952371071a6c604b3f3abe8f2c8fa742158ea6dd7d4").unwrap()[..],
			).unwrap(),
		);

		let address = ExternalAddress::try_from(
			hex::decode("09231da7b19A016f9e576d23B16277062F4d46A8".to_lowercase()).unwrap(),
		)
		.unwrap();
		let address2 = Etherlike::from_public(public.clone());
		assert!(address == address2);
	}

	fn builder_to_checksum_address(checksum_addr: &str) {
		let checksum_addr = checksum_addr.to_lowercase();
		let v = ExternalAddress::try_from(hex::decode(checksum_addr.clone()).unwrap().to_vec())
			.unwrap();
		let checked = Etherlike::to_checksum_address(v);
		assert_eq!(&checked[..], hex::decode(checksum_addr).unwrap());
	}

	#[test]
	fn to_checksum_address() {
		builder_to_checksum_address("5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed");
		builder_to_checksum_address("fB6916095ca1df60bB79Ce92cE3Ea74c37c5d359");
		builder_to_checksum_address("dbF03B407c01E7cD3CBea99509d93f8DDDC8C6FB");
		builder_to_checksum_address("D1220A0cf47c7B9Be7A2E6BA89F429762e7b9aDb");
	}
}
