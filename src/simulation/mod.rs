//! Simulation module

use std::collections::HashMap;
use crate::prelude::*;
use serde::{Deserialize, Serialize};

pub mod physical_integrator;
pub mod boat;
pub mod wind;
pub mod user;
pub mod flat_object_physics;

#[derive(Serialize, Deserialize)]
pub struct SimulationSettings {
	/// Whether to save the simulation when it is quit or when there is an error.
	pub save_sims: bool,
	/// Maximum amount to step simulation if it is running slow
	pub max_time_step: Float,
	/// Limit on how fast the autopilot can move the rudder, in degrees/second
	pub max_rudder_movement: Float,
	/// Resolution of the path tracer, boat has to this far away from the previous point to record a new point
	pub tracer_resulution: Float,
	/// Whether boat tracer is enabled
	pub tracer_enabled: bool,
	/// Time for a client to not be responding for them to be considered disconnected
	pub client_timeout: Float,
	/// Upper limits to prevent the simulation from getting out of control
	pub sanity_limits: SimulatorSanityLimits
}

#[derive(Serialize, Deserialize)]
pub struct SimulatorSanityLimits {
	/// Max speed
	pub speed: Float,
	/// Max angular speed, degrees/sec
	pub angular_speed: Float
}

/// Simulation "save-file"
#[derive(Serialize, Deserialize)]
pub struct SimulationSave {
	pub map_name: String,
	pub local_settings_opt: Option<SimulationSettings>,
	pub paused: bool,
	pub password: Option<String>,
	pub clients: HashMap<String, SimulationClientSave>
}

#[derive(Serialize, Deserialize)]
pub struct SimulationClientSave {
	pub has_finished: bool,
	pub paused: bool,
	pub tracer_list: Vec<V2>,
	pub time_since_reset: Float,
	pub autopilot_enabled: bool,
	pub autopilot_state: crate::autopilot::AutopilotSave,
	pub boat_start: BoatSaveState,
	pub boat: BoatSaveState,
	pub wind: WindGeneratorSaveState,
	/// Time since latest global reset
	pub time: Float,
	/// Best time on the course
	pub best_time: Float
}

/// Main simulation class
pub struct Simulation {
    save_state: SimulationSave
}

impl Simulation {
    pub fn load(save: SimulationSave) -> Result<Self, String> {

    }
}