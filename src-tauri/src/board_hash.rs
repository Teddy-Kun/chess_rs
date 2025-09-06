use serde::Serialize;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

	pub fn iter(&self) -> BoardHashIter {
		BoardHashIter {
			remaining: self.hash,
		}
	}

	pub fn is_empty(&self) -> bool {
		self.hash == 0
	}

	pub fn union(&mut self, other: BoardHash) {
		self.hash |= other.hash;
	}
}

pub struct BoardHashIter {
	remaining: u64,
}

impl Iterator for BoardHashIter {
	type Item = u8;

	fn next(&mut self) -> Option<Self::Item> {
		if self.remaining == 0 {
			return None;
		}

		let tz = self.remaining.trailing_zeros(); // 0..=63
		self.remaining &= !(1_u64 << tz);
		Some(tz as u8)
	}
	fn size_hint(&self) -> (usize, Option<usize>) {
		let cnt = self.remaining.count_ones() as usize;
		(cnt, Some(cnt))
	}
}

impl ExactSizeIterator for BoardHashIter {}

impl IntoIterator for BoardHash {
	type Item = u8;
	type IntoIter = BoardHashIter;

	fn into_iter(self) -> Self::IntoIter {
		BoardHashIter {
			remaining: self.hash,
		}
	}
}

impl Serialize for BoardHash {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		let v: Vec<u8> = self.iter().collect();
		v.serialize(serializer)
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

		let v: Vec<u8> = hasher.into_iter().collect();

		assert_eq!(v.len(), 61)
	}
}
