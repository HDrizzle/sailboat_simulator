//! Copied from virtual_bike

use std::ops;
use crate::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Debug, Eq, Hash)]
pub struct IntV2(pub Int, pub Int);

impl IntV2 {
	pub fn mult(&self, other: Int) -> Self {
		Self(self.0 * other, self.1 * other)
	}
	pub fn to_v2(&self) -> V2 {
		V2::new(self.0 as Float, self.1 as Float)
	}
}

impl ops::Add<IntV2> for IntV2 {
	type Output = IntV2;

	fn add(self, other: Self) -> Self {
		Self(self.0 + other.0, self.1 + other.1)
	}
}

impl ops::Sub<IntV2> for IntV2 {
	type Output = IntV2;

	fn sub(self, other: IntV2) -> Self {
		Self(self.0 - other.0, self.1 - other.1)
	}
}

impl ops::Index<usize> for IntV2 {
	type Output = Int;
	fn index(&self, index: usize) -> &Self::Output {
		match index {
			0 => &self.0,
			1 => &self.1,
			n => panic!("IntV2 index must be 0 or 1, not {}", n)
		}
	}
}