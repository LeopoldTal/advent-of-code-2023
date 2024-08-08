use std::array;

use crate::hash::{get_hash, SIZE};

/// World's stupidest hashmap implementation.
#[derive(Debug, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BucketList {
	buckets: [Bucket; SIZE],
}

impl BucketList {
	/// Creates an empty map.
	pub fn new() -> Self {
		BucketList {
			buckets: array::from_fn(|_| Bucket::new()),
		}
	}

	/// Lists the values in one bucket.
	pub fn get_bucket_values(&self, bucket_index: usize) -> Vec<usize> {
		self.buckets[bucket_index].get_values()
	}

	/// Sets a key-value pair, overwriting the value if the key exists.
	pub fn set(&mut self, key: &str, value: usize) {
		self.buckets[get_hash(key)].set(key, value);
	}

	/// Removes a key-value pair. Does nothing if the key doesn't exist.
	pub fn remove(&mut self, key: &str) {
		self.buckets[get_hash(key)].remove(key);
	}

	/// Summarises all the values.
	pub fn get_power(&self) -> usize {
		self.buckets
			.iter()
			.enumerate()
			.map(|(bucket_index, bucket)| {
				bucket
					.get_values()
					.into_iter()
					.enumerate()
					.map(|(slot_index, value)| (bucket_index + 1) * (slot_index + 1) * value)
					.sum::<usize>()
			})
			.sum()
	}
}

/// A bucket for a given hash.
#[derive(Debug, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Bucket {
	slots: Vec<KeyValue>,
}

impl Bucket {
	fn new() -> Self {
		Self { slots: vec![] }
	}

	/// Gets the values of the pairs in the bucket, in order.
	fn get_values(&self) -> Vec<usize> {
		self.slots.iter().map(|(_, v)| *v).collect()
	}

	/// Adds a key-value pair: replaces the existing pair if any, otherwise appends.
	fn set(&mut self, key: &str, value: usize) {
		let pair: KeyValue = (String::from(key), value);
		if let Some(index) = self.slots.iter().position(|(k, _)| k == key) {
			self.slots[index] = pair;
		} else {
			self.slots.push(pair);
		}
	}

	/// Removes a key-value pair. Does nothing if the key doesn't exist.
	pub fn remove(&mut self, key: &str) {
		self.slots.retain(|(k, _)| k != key);
	}
}

type KeyValue = (String, usize);

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_empty() {
		let list = BucketList::new();
		for bucket_index in 0..SIZE {
			assert_eq!(list.get_bucket_values(bucket_index), vec![]);
		}
	}

	#[test]
	fn test_add() {
		let mut list = BucketList::new();
		list.set("pc", 42);
		assert_eq!(list.get_bucket_values(0), vec![]);
		assert_eq!(list.get_bucket_values(1), vec![]);
		assert_eq!(list.get_bucket_values(2), vec![]);
		assert_eq!(list.get_bucket_values(3), vec![42]);
		assert_eq!(list.get_bucket_values(4), vec![]);
	}

	#[test]
	fn test_replace() {
		let mut list = BucketList::new();
		list.set("qp", 42);
		assert_eq!(list.get_bucket_values(1), vec![42]);
		list.set("qp", 23);
		assert_eq!(list.get_bucket_values(1), vec![23]);
	}

	#[test]
	fn test_remove() {
		let mut list = BucketList::new();
		list.set("pc", 42);
		assert_eq!(list.get_bucket_values(3), vec![42]);
		list.remove("pc");
		assert_eq!(list.get_bucket_values(3), vec![]);
	}

	#[test]
	fn test_add_in_order() {
		let mut list = BucketList::new();
		list.set("rn", 1);
		list.set("cm", 3);
		list.set("", 2);
		assert_eq!(list.get_bucket_values(0), vec![1, 3, 2]);
	}

	#[test]
	fn test_replace_preserves_order() {
		let mut list = BucketList::new();
		list.set("pc", 1);
		list.set("ot", 2);
		list.set("ab", 3);
		list.set("ot", 999);
		assert_eq!(list.get_bucket_values(3), vec![1, 999, 3]);
	}

	#[test]
	fn test_remove_preserves_order() {
		let mut list = BucketList::new();
		list.set("pc", 1);
		list.set("ot", 2);
		list.set("ab", 3);
		list.remove("ot");
		assert_eq!(list.get_bucket_values(3), vec![1, 3]);
	}
}
