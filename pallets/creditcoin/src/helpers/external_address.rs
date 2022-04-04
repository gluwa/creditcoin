use base58::FromBase58;
use sp_io::hashing::sha2_256;
use crate::{Blockchain, ExternalAddress};

pub fn address_is_well_formed(blockchain: &Blockchain, address: &ExternalAddress) -> bool {
	match blockchain {
		Blockchain::Bitcoin => btc_address_is_well_formed(address),
		Blockchain::Ethereum | Blockchain::Luniverse | Blockchain::Rinkeby => {
			eth_address_is_well_formed(address)
		},
		Blockchain::Other(_) => true,
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
	use frame_support::BoundedVec;
	use core::convert::{TryInto, TryFrom};

	use super::*;

	#[test]
	fn eth_address_is_well_formed_works() {
		// length == 20
		assert!(eth_address_is_well_formed(hex::decode("5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed").unwrap().as_slice()));
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

		let eth_addr = hex::decode("5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed").unwrap().try_into().unwrap();
		let btc_addr = b"1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa".to_vec().try_into().unwrap();

		assert!(address_is_well_formed(&ethereum, &eth_addr));
		assert!(address_is_well_formed(&rinkeby, &eth_addr));
		assert!(address_is_well_formed(&luniverse, &eth_addr));
		assert!(address_is_well_formed(&bitcoin, &btc_addr));
		assert!(address_is_well_formed(&other, &eth_addr));
		assert!(address_is_well_formed(&other, &btc_addr));
	}
}
