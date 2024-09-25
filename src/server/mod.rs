//! Server module

use crate::prelude::*;
use serde::{Deserialize, Serialize};
use rouille;// Simple HTTP Server

/// All possible requests to the Renet server
#[derive(Serialize, Deserialize)]
pub enum RenetRequest {
	
}

/// All possible responses from the Renet server
#[derive(Serialize, Deserialize)]
pub enum RenetResponse {
	
}

/// Main server
pub struct WorldServer {
    sim_name: String,
    sim: Simulation
}