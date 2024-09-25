//! Module with a trait for implementing the 2/3-time-step-averaging numerical integration technique

use crate::prelude::*;

pub trait PhysicalIntegrator<STATIC, DYNAMIC>: Clone {
	/// Regular physics step
	fn partial_step(&mut self, dt: Float, static_: &STATIC, dynamic: &DYNAMIC);
	/// Average self with corresponding fields of `other`
	fn average_with_other_state(&mut self, other: Self, static_: &STATIC, dynamic: &DYNAMIC);
	/// Special method I made up, should prevent "bouncing"
	/// This is the only method that should be called outside of this trait
	fn full_step(&mut self, dt: Float, static_: &STATIC, dynamic: &DYNAMIC) {
		self.partial_step(dt * 0.666666666, static_, dynamic);
		let self_clone = self.clone();
		self.partial_step(dt * 0.666666666, static_, dynamic);
		self.average_with_other_state(self_clone, static_, dynamic);
	}
}