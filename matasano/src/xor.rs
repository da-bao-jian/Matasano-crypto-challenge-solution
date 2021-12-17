pub trait XOR {
	fn xor(&self, _: &Self) -> Vec<u8>;
}

impl XOR for Vec<u8> {
	fn xor(&self, snd_hex: &Self) -> Vec<u8> {
		let mut result = self.clone();
		for chunk in result.chunks_mut(snd_hex.len()) {
			let length = chunk.len();
			for (hex1, hex2) in chunk.iter_mut().zip(snd_hex[..length].iter()) {
				*hex1 ^= hex2;
			}
		}
		result
	}
}