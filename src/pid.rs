use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct Pid {
	pub(crate) center: f64,
	pub(crate) k_p: f64,
	pub(crate) k_i: f64,
	pub(crate) k_d: f64,

	#[serde(skip)]
	pub(crate) last_error: f64,
	#[serde(skip)]
	pub(crate) integral: f64,
}

impl Pid {
	pub(crate) fn update(&mut self, input: f64) -> f64 {
		let error = input - self.center;
		let last_error = std::mem::replace(&mut self.last_error, error);
		self.integral += error;

		self.k_p * error
			+ self.k_i * self.integral
			+ self.k_d * (last_error - error)
	}
}