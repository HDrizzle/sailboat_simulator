//! Module which contains all the JSON-compatible types loaded from the disk

use std::collections::HashMap;
use crate::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json;

// CONSTS
const RESOURCES_DIR: &str = "resources/";
const MAPS_DIR: &str = "maps/";
const BOAT_TYPES_DIR: &str = "boat_types/";
const SIMULATIONS_DIR: &str = "simulations/";
const GLOBAL_SETTINGS_FILE: &str = "global_settings";// No ".json" because it will be added automatically

/// Meant for when a resource can't be loaded
pub struct ResourceLoadError {
	resource_type: ResourceType,
	full_path: String,
	name: String,
	message: Option<String>,
	error_type: ResourceLoadErrorType
}

pub enum ResourceLoadErrorType {
	CannotFindFile,
	NoPermission,
	CannotDecode
}

// Resource types
pub enum ResourceType {
	Map,
	BoatType,
	Simulation,
	GlobalSettings
}

pub enum Resource {
	Map(MapSave),
	BoatType(BoatType),
	Simulation(SimulationSave),
	GlobalSettings(Settings)
}

impl ResourceType {
	pub fn encoding(&self) -> ResourceEncoding {
		self.type_info().0
	}
	pub fn load(&self, name: &str) -> Result<Resource> {
		let type_info = self.type_info();
		let full_path: String = match type_info.2 {
			true => format!("{}{}{}{}", RESOURCES_DIR, &type_info.1, name, type_info.0.extension()),// Within a folder
			false => format!("{}{}{}", RESOURCES_DIR, &type_info.1, type_info.0.extension())// Standalone file (ex: settings), don't need `name`
		};
	}
	/// (Resource encoding, path, whether the path is a folder (true) or file (false))
	fn type_info(&self) -> (ResourceEncoding, &'static str, bool) {
		match &self {
			Self::Map => (ResourceEncoding::JSON, MAPS_DIR, true),
			Self::BoatType => (ResourceEncoding::JSON, BOAT_TYPES_DIR, true),
			Self::Simulation => (ResourceEncoding::JSON, SIMULATIONS_DIR, true),
			Self::GlobalSettings => (ResourceEncoding::JSON, GLOBAL_SETTINGS_FILE, false),
		}
	}
}

pub enum ResourceEncoding {
	JSON,
	Plaintext,
	PNG
}

impl ResourceEncoding {
	pub fn extension(&self) -> String {
		match &self {
			Self::JSON => String::from(".json"),
			Self::Plaintext => String::from(".txt"),
			Self::PNG => String::from(".png")
		}
	}
	pub fn load<T>(&self) -> Result<T, String> {
		// TODO
	}
}

// Loading
pub fn load_boat_type(name: &str) -> Result<BoatType, String> {
	let raw_string: String = load_file_with_better_error(&(VEHICLES_DIR.to_owned() + name + "/" + VEHICLE_STATIC_JSON_FILENAME))?;
	let vehicle: VehicleStatic = serde_json::from_str(&raw_string)?;
	Ok(vehicle)
}

#[derive(Serialize, Deserialize)]
pub struct Settings {
	/// Simulation settings
	pub simulator: SimulationSettings,
	/// GUI & frontend settings
	pub gui: GuiSettings,
}

#[derive(Serialize, Deserialize)]
pub struct GuiSettings {
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
pub struct GuiColors {
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
pub struct UserContact {
	pub username: String,
	pub password: String,
	pub blocked: String
}

/// A map is sort of a template to base simulations off of, NOTE: A simulation is stored seperate from any map it may use
/// Maps are read-only and are not written to when a simulation is saved
#[derive(Serialize, Deserialize)]
pub struct MapSave {
	pub size: IntV2,
	pub global_default_start: V2,
	pub end: V2,
	pub landmasses: GenericDataset<LandmassSave>
}

#[derive(Serialize, Deserialize)]
pub struct LandmassSave {
	/// List of coordinates to make up coastline, it does NOT have to have the same coordinate at the start and end points
	pub coastline: Vec<V2>,
	/// If it has a name, and where to display it on the map
	pub name_and_representative_point_opt: Option<(V2, String)>,
	/// Color of specific landass, overrides default color
	pub color: [u8; 4]
}