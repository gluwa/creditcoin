use base58::FromBase58;
use sp_io::hashing::{keccak_256, sha2_256};
use sp_std::prelude::*;

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

#[derive(Copy, Clone)]
enum AsciiCase {
	None,
	Lower,
	Upper,
}

const HEX_PREFIX: &[u8] = b"0x";
const ETH_ADDRESS_LENGTH: usize = 42;
const ETH_ADDRESS_VALUE_LENGTH: usize = ETH_ADDRESS_LENGTH - HEX_PREFIX.len();

fn eth_address_is_well_formed(address: &[u8]) -> bool {
	if !address.starts_with(HEX_PREFIX) || address.len() != ETH_ADDRESS_LENGTH {
		return false;
	}

	let address_value = &address[HEX_PREFIX.len()..];

	if eth_address_is_checksummed(address_value) {
		eth_address_checksum_valid(address_value)
	} else {
		true
	}
}

fn eth_address_is_checksummed(address: &[u8]) -> bool {
	let mut case = AsciiCase::None;
	for byte in address {
		if !byte.is_ascii_hexdigit() {
			return false;
		}
		if byte.is_ascii_digit() {
			continue;
		} else if byte.is_ascii_uppercase() {
			match case {
				AsciiCase::None => case = AsciiCase::Upper,
				AsciiCase::Upper => continue,
				AsciiCase::Lower => return true,
			}
		} else if byte.is_ascii_lowercase() {
			match case {
				AsciiCase::None => case = AsciiCase::Lower,
				AsciiCase::Lower => continue,
				AsciiCase::Upper => return true,
			}
		}
	}
	false
}

fn eth_address_checksum(address: &[u8]) -> Option<Vec<u8>> {
	if address.len() != ETH_ADDRESS_VALUE_LENGTH {
		return None;
	}

	let mut checksummed_address = Vec::with_capacity(ETH_ADDRESS_VALUE_LENGTH);

	let address_lowercase = address.to_ascii_lowercase();

	let address_hash = keccak_256(&*address_lowercase);
	let address_hash_hex = hex::encode(&address_hash);

	for (i, byte) in address_lowercase.iter().enumerate() {
		if byte.is_ascii_digit() {
			checksummed_address.push(*byte);
		} else {
			let hashed_byte_value = u8::from_str_radix(&address_hash_hex[i..i + 1], 16)
				.expect("We just encoded this string as hex; qed");
			if hashed_byte_value > 7 {
				checksummed_address.push(byte.to_ascii_uppercase());
			} else {
				checksummed_address.push(*byte);
			}
		}
	}

	Some(checksummed_address)
}

fn eth_address_checksum_valid(address: &[u8]) -> bool {
	if address.len() != ETH_ADDRESS_VALUE_LENGTH {
		return false;
	}

	let checksummed_address = if let Some(addr) = eth_address_checksum(address) {
		addr
	} else {
		return false;
	};

	address == checksummed_address
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn eth_address_is_checksummed_works() {
		assert!(eth_address_is_checksummed(b"5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed"));
		assert!(eth_address_is_checksummed(b"5AAEB6053F3E94C9B9A09F33669435E7EF1BEAEd"));
		assert!(!eth_address_is_checksummed(b"5AAEB6053F3E94C9B9A09F33669435E7EF1BEAED"));
		assert!(!eth_address_is_checksummed(b"5aaeb6053f3e94c9b9a09f33669435e7ef1beaed"));
		assert!(!eth_address_is_checksummed(b"0000000000000000000000000000000000000000"));
		assert!(!eth_address_is_checksummed(b"zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz"));
	}

	#[test]
	fn eth_address_checksum_works() {
		assert_eq!(
			eth_address_checksum(b"5Aaeb6053F3e94c9b9a09f33669435e7ef1beaeD"),
			Some(b"5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed".to_vec())
		);
		assert_eq!(
			eth_address_checksum(b"Fb6916095ca1df60bb79ce92ce3ea74c37c5d359"),
			Some(b"fB6916095ca1df60bB79Ce92cE3Ea74c37c5d359".to_vec())
		);
		assert_eq!(
			eth_address_checksum(b"0000000000000000000000000000000000000000"),
			Some(b"0000000000000000000000000000000000000000".to_vec())
		);
		assert_eq!(eth_address_checksum(&[]), None);
	}

	#[test]
	fn eth_address_checksum_valid_works() {
		assert!(eth_address_checksum_valid(b"5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed"));
		assert!(eth_address_checksum_valid(b"fB6916095ca1df60bB79Ce92cE3Ea74c37c5d359"));
		assert!(eth_address_checksum_valid(b"dbF03B407c01E7cD3CBea99509d93f8DDDC8C6FB"));
		assert!(eth_address_checksum_valid(b"D1220A0cf47c7B9Be7A2E6BA89F429762e7b9aDb"));
	}

	#[test]
	fn eth_address_is_well_formed_works() {
		// normal addresses (checksummed)
		assert!(eth_address_is_well_formed(b"0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed"));
		assert!(eth_address_is_well_formed(b"0xfB6916095ca1df60bB79Ce92cE3Ea74c37c5d359"));
		assert!(eth_address_is_well_formed(b"0xdbF03B407c01E7cD3CBea99509d93f8DDDC8C6FB"));
		assert!(eth_address_is_well_formed(b"0xD1220A0cf47c7B9Be7A2E6BA89F429762e7b9aDb"));

		// all uppercase
		assert!(eth_address_is_well_formed(b"0x52908400098527886E0F7030069857D2E4169EE7"));
		assert!(eth_address_is_well_formed(b"0x8617E340B3D01FA5F11F306F4090FD50E238070D"));

		// all lowercase
		assert!(eth_address_is_well_formed(b"0xde709f2102306220921060314715629080e2fb77"));
		assert!(eth_address_is_well_formed(b"0x27b1fdb04752bbc536007a920d24acb045561c26"));

		// normal but incorrect
		assert!(!eth_address_is_well_formed(b"0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAeD"));
		assert!(!eth_address_is_well_formed(b"0xfB6916095ca1Df60bB79Ce92cE3Ea74c37c5d359"));
		assert!(!eth_address_is_well_formed(b"0xdbF03B407c01e7cD3CBea99509d93f8DDDC8C6Fb"));
		assert!(!eth_address_is_well_formed(b"0xD1220A0cF47c7B9Be7A2E6BA89F429762e7B9aDb"));
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
}
