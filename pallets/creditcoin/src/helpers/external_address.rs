use crate::{Blockchain, ExternalAddress};
use base58::FromBase58;
use core::convert::TryFrom;
use frame_support::BoundedVec;
use sp_core::ecdsa::Public;
use sp_io::hashing::keccak_256;
use sp_io::hashing::sha2_256;

pub fn generate_external_address(
	blockchain: &Blockchain,
	// TODO: refactor: remove `reference` as a parameter for generate_external_address
	// It's very suspicious to be giving the external address to the function that's meant to generate it.
	reference: &ExternalAddress,
	public_key: Public,
) -> Option<ExternalAddress> {
	match blockchain {
		Blockchain::Luniverse | Blockchain::Ethereum | Blockchain::Rinkeby
			if EVMAddress::try_extract_address_type(reference).is_some() =>
		{
			Some(EVMAddress::from_public(&public_key))
		},
		Blockchain::Bitcoin => None,
		Blockchain::Other(_) => None,
		_ => None,
	}
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
}
