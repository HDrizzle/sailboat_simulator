//! Multiplayer sailboat simulation game, based off the old python version found at https://github.com/HDrizzle/sailboat_simulator_python

use nalgebra::{Point3, Point2, Vector3, Vector2, point, Matrix, Const, ArrayStorage, OPoint, Translation, Isometry3, UnitQuaternion};

pub mod client;
pub mod server;
pub mod resource_interface;
pub mod generic_ref;
pub mod intv2;
pub mod autopilot;

#[allow(unused)]
pub mod prelude {
	use super::*;
	// Name of this app
	pub const APP_NAME: &str = "Virtual Bike";
	// Types
	pub type Float = f32;
	pub type Int = i64;
	pub type UInt = u64;
	// Example of basic usage: https://rapier.rs/docs/user_guides/rust/introduction_to_nalgebra
	pub type P2 = Point2<Float>;
	pub type V2 = Vector2<Float>;
	pub type P3 = Point3<Float>;
	pub type V3 = Vector3<Float>;
	pub type Iso = Isometry2<Float>;
	pub const EPSILON: Float = 1.0e-6;// Arbitrary
	pub use std::f32::consts::PI;
	// Misc
	pub use crate::{
		generic_ref::{GenericRef, GenericQuery, GenericDataset},
		intv2::IntV2,
		autopilot::Autopilot
	};
}

// Ui main
fn ui_main() {

}