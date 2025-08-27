#[derive(Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct BoardHash {
	hash: u64,
}

impl BoardHash {
	pub fn new() -> BoardHash {
		BoardHash { hash: 0 }
	}

	pub fn contains(&self, v: u8) -> bool {
		(self.hash & 1 << (v % 64)) != 0
	}

	pub fn insert(&mut self, v: u8) {
		self.hash |= 1 << (v % 64)
	}

	pub fn remove(&mut self, v: u8) {
		// Create a mask with a 1 at position 'v',
		// then invert it to get a 0 at position 'v' and 1s everywhere else
		self.hash &= !(1 << (v % 64));
	}
}

#[cfg(test)]
mod tests {
	use crate::board_hash::BoardHash;

	#[test]
	fn test_board_hash() {
		let mut hasher = BoardHash::new();

		for i in 0..62 {
			hasher.insert(i);
			assert_eq!(hasher.contains(i), true);
		}

		assert_eq!(hasher.contains(63), false);

		hasher.remove(1);

		assert_eq!(hasher.contains(1), false);
	}
}
