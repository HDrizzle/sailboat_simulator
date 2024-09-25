//! Pseudo-random wind generator

use crate::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct WindGeneratorSaveState {
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

pub type Wind = V2;