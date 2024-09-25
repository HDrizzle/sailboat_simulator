//! Class describing a flat object in fluid, such as a rudder or sail, may be simplified

use crate::prelude::*;

use super::physical_integrator::PhysicalIntegrator;

pub struct FlatObjectStaticData {
    /// Area
    pub area: Float,
    /// Distance from pivot to center of pressure
    pub center_of_pressure: Float,
    /// Moment
    pub moment: Float,
    /// Angular drag
    pub angular_drag: Float,
    /// Fluid density
    pub fluid_density: Float
}

pub struct FlatObjectDynamicData {
    /// wrt. the pivot
    pub global_velocity: V2,
    pub global_angular_velocity: V2
}

#[derive(Clone)]
pub struct FlatObjectPhysicalIntegrator {
    pub angle: Float,
    pub angular_velocity: Float
}

impl PhysicalIntegrator<FlatObjectStaticData, FlatObjectDynamicData> for FlatObjectPhysicalIntegrator {
    fn partial_step(&mut self, dt: Float, static_: &FlatObjectStaticData, dynamic: &FlatObjectDynamicData) {
        // TODO
    }
    fn average_with_other_state(&mut self, other: Self, static_: &FlatObjectStaticData, dynamic: &FlatObjectDynamicData) {
        // TODO
    }
}