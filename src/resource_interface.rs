//! Module which contains all the JSON-compatible types loaded from the disk

use std::collections::HashMap;
use crate::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json;

const RESOURCES_DIR: &str = "resources";

#[derive(Serialize, Deserialize)]
struct Settings {
	/// Simulation settings
	pub simulator: SimulatorSettings,
	/// GUI & frontend settings
	pub gui: GuiSettings,
}

#[derive(Serialize, Deserialize)]
struct SimulatorSettings {
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
struct SimulatorSanityLimits {
	/// Max speed
	pub speed: Float,
	/// Max angular speed, degrees/sec
	pub angular_speed: Float
}

#[derive(Serialize, Deserialize)]
struct GuiSettings {
	/// Whether to floodfill land on main map, may reduce performance
	pub floodfil_land: bool,
	/// Whether to show the true wind vector
	pub show_true_wind: bool,
	/// Whether to show the timer
	pub show_timer: bool,
	/// Change if the scrolling to zoom is backward
	pub reverse_scrolling: bool,
	/// Whether to show the boats path on the minimap
	pub show_tracer: bool,
	/// Whether to show all of the vectors (doesn't effect apparent wind)
	pub show_vectors: bool,
	/// Whether to show the end position with the checkered flag icon
	pub show_end_flag: bool,
	/// Whether to show the results from the performance tracker
	pub show_performance: bool,
	/// Whether to show text on boats on the main image
	pub show_boat_labels: bool,
	/// Side to display the control pain on the simulation, true=right, false=left
	pub control_panel_on_right_side: bool,
	/// Color settings
	pub colors: GuiColors
}

/// GUI Color Settings, all colors stored as a [u8; 4] represeenting [Red, Green, Blue, Alpha]
#[derive(Serialize, Deserialize)]
struct GuiColors {
	pub true_wind: [u8; 4],
	pub app_wind: [u8; 4],
	pub drag_force: [u8; 4],
	pub velocity: [u8; 4],
	pub rudder_force: [u8; 4],
	pub sail_force: [u8; 4],
	pub net_sail_force: [u8; 4],
	pub ocean: [u8; 4],
	pub land: [u8; 4],
	pub text_fg: [u8; 4],
	pub text_bg: [u8; 4],
	pub tracer_path: [u8; 4],
	pub net_acceleration: [u8; 4]
}

#[derive(Serialize, Deserialize)]
struct UserContact {
	pub username: String,
	pub password: String,
	pub blocked: String
}

/// A map is sort of a template to base simulations off of, NOTE: A simulation is stored seperate from any map it may use
/// Maps are read-only and are not written to when a simulation is saved
#[derive(Serialize, Deserialize)]
struct MapSave {
	pub size: IntV2,
	pub global_default_start: V2,
	pub end: V2,
	pub landmasses: GenericDataset<LandmassSave>
}

#[derive(Serialize, Deserialize)]
struct LandmassSave {
	/// List of coordinates to make up coastline, it does NOT have to have the same coordinate at the start and end points
	pub coastline: Vec<V2>,
	/// If it has a name, and where to display it on the map
	pub name_and_representative_point_opt: Option<(V2, String)>,
	/// Color of specific landass, overrides default color
	pub color: [u8; 4]
}

/// Simulation "save-file"
#[derive(Serialize, Deserialize)]
struct SimulationSave {
	pub map_name: String,
	pub local_settings_opt: Option<SimulatorSettings>,
	pub paused: bool,
	pub password: Option<String>,
	pub clients: HashMap<String, SimulationClientSave>
}

#[derive(Serialize, Deserialize)]
struct SimulationClientSave {
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

#[derive(Serialize, Deserialize)]
struct BoatSaveState {
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

#[derive(Serialize, Deserialize)]
struct SailSaveState {
	/// Current angle relative to a line from the mast pointing straight aft, + = CCW, - = CW
	pub angle: Float,
	/// Maximum value of |angle|, always positive
	pub sheeting_angle: Float
}

#[derive(Serialize, Deserialize)]
struct WindGeneratorSaveState {
	/// Speed average, m/s
	pub speed_average: Float,
	/// Maximum gust, m/s
	pub max_gust: Float,
	/// Max speed variation in m/s^s
	pub max_speed_variation: Float,
	/// Max direction variation in degrees/s^2
	pub max_direction_variation: Float,
	/// Current speed
	pub speed: Float,
	/// Current direction
	pub direction: Float
}