//! Server module

use crate::prelude::*;
use serde::{Deserialize, Serialize};

pub mod simulator;

/// All possible requests to the Renet server
#[derive(Serialize, Deserialize)]
pub enum RenetRequest {
	
}

/// All possible responses from the Renet server
#[derive(Serialize, Deserialize)]
pub enum RenetResponse {
	
}