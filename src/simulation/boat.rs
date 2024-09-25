//! Physics simulation and graphics for boats

use crate::prelude::*;
use serde::{Deserialize, Serialize};
use std::rc::Rc;

use super::physical_integrator::PhysicalIntegrator;

// CONSTS
const DEFAULT_SHEETING_ANGLE: Float = 90.0;

/// Specification for a type of boat
#[derive(Serialize, Deserialize)]
pub struct BoatType {
	/// List of coordinates making up perimiter of hull
	pub perimeter: Vec<V2>,
	/// Distance from the center of the boat to the hull's center of lateral resistance without the rudder (+ is forward, - is aft)
	pub center_of_lateral_resistance: Float,
	/// Forward drag coefficient of the hull
	pub forward_drag: Float,
	/// Sideways drag coefficient of the hull
	pub sideways_drag: Float,
	/// Air drag (excluding sails, would include rest of rigging)
	pub air_drag: Float,
	/// Deepest part of hull
	pub max_draft: Float,
	/// Distance from center of boat to where the rudder pivot is (negative for most boats)
	pub rudder_pivot: Float,
	/// Rudder area
	pub rudder_area: Float,
	/// Distance from the rudder pivot to the rudder center of effort
	pub rudder_center_of_effort: Float,
	/// Length of the rudder, from the pivot aft
	pub rudder_len: Float,
	/// Net mass of entire boat
	pub mass: Float,
	/// Angular moment of entire boat
	pub moment: Float,
	/// Angular drag coefficient of the hull (without rudder)
	pub angular_drag: Float,
	/// Maximum hull hit-points
	pub max_hull_hp: Float,
	/// Maximum rudder hit-points
	pub max_rudder_hp: Float,
	/// Angle between the boat and the wind (apparent) where the boat is fastest upwind, used by the autopilot
	pub upwind_max_wind_angle: Float,
	/// Angle between the boat's momentum and the wind (apparent) where the boat is fastest upwind, used by the autopilot
	pub upwind_max_total_leeway: Float,
	/// Configuration of all the sails
	pub sails: GenericDataset<SailStatic>
}

#[derive(Serialize, Deserialize)]
pub struct SailStatic {
	/// Area
	pub area: Float,
	/// Distance from the tack to the center of effort (from above)
	pub center_of_effort: Float,
	/// Distance between the tack and the center of the boat (+ is forward, - is aft)
	pub tack: Float,
	/// Length of the foot of the sail (only used for display purposes)
	pub foot_len: Float
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SailSaveState {
	/// Current angle relative to a line from the mast pointing straight aft, + = CCW, - = CW
	pub angle: Float,
	/// Maximum value of |angle|, always positive
	pub sheeting_angle: Float
}

impl SailSaveState {
	pub fn average_with_other_state(&mut self, other: &Self) {
		self.angle = (self.angle + other.angle) / 2.0;
	}
}

impl Default for SailSaveState {
	fn default() -> Self {
		Self {
			angle: 0.0,
			sheeting_angle: DEFAULT_SHEETING_ANGLE
		}
	}
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BoatSaveState {
	/// Name of static boat type file (without .json)
	pub type_name: String,
	/// Displacement, angle
	pub pos: Iso,
	/// Velocity, angular velocity
	pub vel: Iso,
	/// Rudder angle
	pub rudder_angle: Float,
	/// Rudder hit-points
	pub rudder_hp: bool,
	/// Hull hit-points
	pub hull_hp: Float,
	/// Sails
	pub sails: GenericDataset<SailSaveState>
}

impl BoatSaveState {
	pub fn average_with_other_state(&mut self, other: Self) {
		self.pos = average_iso(&self.pos, &other.pos);
		self.vel = average_iso(&self.vel, &other.vel);
		self.rudder_angle = (self.rudder_angle + other.rudder_angle) / 2.0;// TODO: check: This will only be reliable if the angle doesn't go across 0
		// iterate over sails
		for (ref_, sail) in self.sails.items.iter_mut() {
			sail.average_with_other_state(other.sails.get_item_tuple(&ref_.to_query()).expect(&format!("Could not corresponding sail to average with during physics integrator update, reference={:?}", ref_)).1);
		}
	}
}

#[derive(Clone)]
struct BoatPhysicalIntegrator {
	state: BoatSaveState
}

impl PhysicalIntegrator<BoatStatic, Wind> for BoatPhysicalIntegrator {
	fn partial_step(&mut self, dt: Float, static_: &BoatStatic, dynamic: &Wind) {
		// Update sails
		// TODO
		// Get all translational forces
		// TODO
		// Get all torques
		// TODO
		// Update translational velocity
		// TODO
		// Update angular velocity
		// TODO
		// Update translation
		// TODO
		// Update angule
		// TODO
	}
	fn average_with_other_state(&mut self, other: Self, static_: &BoatStatic, dynamic: &Wind) {
		self.state.average_with_other_state(other.state);
	}
}

pub struct Boat {
	/// Boat type
	/// Incase there are many boats of the same type, it is more efficient to store one reference to the boat type, so reference counter is used
	static_: Rc<BoatStatic>,
	physics: BoatPhysicalIntegrator,
	rudder_input_opt: Option<Float>
}

impl Boat {
	pub fn update(&mut self, wind: &Wind, inputs: BoatInputs) {
		// TODO
	}
}

#[derive(Serialize, Deserialize)]
pub struct BoatInputs {
	pub rudder_control: Option<Float>,
	pub sheeting_angles: Vec<(GenericQuery<SailSaveState>, Float)>
}

impl Default for BoatInputs {
	fn default() -> Self {
		Self {
			rudder_control: None,
			sheeting_angles: Vec::new()
		}
	}
}

impl BoatInputs {
	/// Update current self with new input
	pub fn merge(&mut self, other: &Self, sails: &GenericDataset<SailSaveState>) {
		// TODO
	}
}