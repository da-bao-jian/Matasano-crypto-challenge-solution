use hex;
use crate::xor::XOR;

pub fn xor_compare(str1: String, str2: String, result: &str) -> bool {

	let hex1 = hex::decode(
		str1.as_bytes()
	).expect("Decoding failed");

	let hex2 = hex::decode(
		str2.as_bytes()
	).expect("Decoding failed");

	hex::encode(hex1.xor(&hex2)) == result
	
}