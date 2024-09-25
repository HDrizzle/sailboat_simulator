//! For handling a user's boat, autopilot, and other things

use crate::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserInput {
    boat: Option<BoatInputs>,
    autopilot: Option<AutopilotInputs>
}