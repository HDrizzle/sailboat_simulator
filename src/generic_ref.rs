//! Generic references to any type of game "object"
//! Copied from virtual_bike

use std::marker::PhantomData;
use serde::{Deserialize, Serialize};

/// Generic reference to things in the game
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug, Hash)]
pub struct GenericRef<T> {
	pub id: u64,
	pub unique_name_opt: Option<String>,
	#[serde(skip)]
	_phantom: PhantomData<T>
}

impl<T> GenericRef<T> {
	pub fn id(id: u64) -> Self {
		Self {
			id,
			unique_name_opt: None,
			_phantom: PhantomData{}
		}
	}
	pub fn to_query(&self) -> GenericQuery<T> {
		GenericQuery::id(self.id)
	}
	pub fn query_matches(&self, query: &GenericQuery<T>) -> bool {
		match query {
			GenericQuery::Id(id, _) => self.id == *id,
			GenericQuery::UniqueName(other_name, _) => match &self.unique_name_opt {
				Some(self_name) => other_name == self_name,
				None => false
			}
		}
	}
	/// WARNING: This method can do what this whole type is meant to avoid: using references in the wrong context. Use with caution.
	pub fn into_another_type<T2>(&self) -> GenericRef<T2> {
		GenericRef::<T2> {
			id: self.id,
			unique_name_opt: self.unique_name_opt.clone(),
			_phantom: PhantomData{}
		}
	}
	pub fn to_string(&self) -> String {
		match &self.unique_name_opt {
			Some(name) => format!("{} ({})", self.id, name),
			None => self.id.to_string()
		}
	}
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct CustomPhantomData<T> {
	_phantom: PhantomData<T>
}

impl<T> CustomPhantomData<T> {
	pub fn new() -> Self {
		Self{_phantom: PhantomData{}}
	}
}

impl<T> Default for CustomPhantomData<T> {
	fn default() -> Self {
		Self::new()
	}
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum GenericQuery<T> {
	Id(
		u64,
		#[serde(skip)]
		CustomPhantomData<T>
	),
	UniqueName(
		String,
		#[serde(skip)]
		CustomPhantomData<T>
	)
}

impl<T> GenericQuery<T> {
	pub fn id(id: u64) -> Self {
		Self::Id(id, CustomPhantomData::new())
	}
	pub fn unique_name(unique_name: String) -> Self {
		Self::UniqueName(unique_name, CustomPhantomData::new())
	}
}

impl<T> PartialEq for GenericQuery<T> {
	fn eq(&self, other: &Self) -> bool {
		match self {
			Self::Id(self_id, _) => if let Self::Id(other_id, _) = other {
				self_id == other_id
			}
			else {
				false
			},
			Self::UniqueName(self_id, _) => if let Self::UniqueName(other_id, _) = other {
				self_id == other_id
			}
			else {
				false
			}
		}
	}
}

impl<T> Default for GenericQuery<T> {
	fn default() -> Self {
		Self::id(0)
	}
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GenericDataset<T> {
	pub items: Vec<(GenericRef<T>, T)>
}

impl<T> GenericDataset<T> {
	pub fn new() -> Self {
		Self {
			items: Vec::new()
		}
	}
	pub fn get_item_index_with_query(&self, query: &GenericQuery<T>) -> Option<usize> {
		for (i, (ref_, _)) in self.items.iter().enumerate() {
			if ref_.query_matches(query) {
				return Some(i)
			}
		}
		// Default
		None
	}
	pub fn get_item_tuple(&self, query: &GenericQuery<T>) -> Option<(&GenericRef<T>, &T)> {
		match self.get_item_index_with_query(query) {
			Some(i) => Some((&self.items[i].0, &self.items[i].1)),
			None => None
		}
	}
	pub fn get_item_id(&self, query: &GenericQuery<T>) -> Option<u64> {
		match self.get_item_tuple(query) {
			Some((ref_, _)) => Some(ref_.id),
			None => None
		}
	}
}