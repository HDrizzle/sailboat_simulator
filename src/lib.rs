//! Multiplayer sailboat simulation game, based off the old python version found at https://github.com/HDrizzle/sailboat_simulator_python

use nalgebra::{Point3, Point2, Vector3, Vector2, point, Matrix, Const, ArrayStorage, OPoint, Translation, Isometry2, UnitQuaternion};
use dialoguer;

pub mod client;
pub mod server;
pub mod resource_interface;
pub mod generic_ref;
pub mod intv2;
pub mod autopilot;
pub mod simulation;

#[allow(unused)]
pub mod prelude {
	use nalgebra::UnitComplex;

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
		autopilot::{Autopilot, AutopilotInputs},
		resource_interface,
		server::WorldServer,
		simulation::{Simulation, SimulationSave, SimulationClientSave, SimulationSettings, boat::{Boat, BoatSaveState, BoatType, BoatInputs}, wind::{WindGeneratorSaveState, Wind}}
	};
	// Random functions
	/// Average 2 isometries
	/// TODO: Test if averaging angles across 360 deg doesn't return 180 deg
	pub fn average_iso(iso1: &Iso, iso2: &Iso) -> Iso {
		Iso::from_parts(
			Translation{
				vector: (iso1.translation.vector + iso2.translation.vector) / 2.0
			},
			UnitComplex::from_complex(iso1.rotation.complex() + iso2.rotation.complex())// TODO: test
		)
	}
	pub fn to_string_err<T, E: ToString>(result: Result<T, E>) -> Result<T, String> {
		match result {
			Ok(t) => Ok(t),
			Err(e) => Err(e.to_string())
		}
	}
	pub fn to_string_err_with_message<T, E: ToString>(result: Result<T, E>, message: &str) -> Result<T, String> {
		match result {
			Ok(t) => Ok(t),
			Err(e) => Err(format!("Message: {}, Error: {}", message, e.to_string()))
		}
	}
	pub fn prompt(s: &str) -> String {
		dialoguer::Input::new()
			.with_prompt(s)
			.interact_text()
			.unwrap()
	}
}

// Ui main
pub fn ui_main() {

}