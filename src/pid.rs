use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Pid {
	k_p: f64,
	k_i: f64,
	k_d: f64,

	#[serde(skip)]
	last: f64,
	#[serde(skip)]
	integral: f64,
}

impl Pid {
	pub fn new(k_p: f64, k_i: f64, k_d: f64) -> Pid {
		Pid {
			k_p, k_i, k_d,
			last: 0f64,
			integral: 0f64,
		}
	}

	pub fn set_last(&mut self, last: f64) {
		self.last = last;
	}

	pub fn update(&mut self, input: f64) -> f64 {
		let last = std::mem::replace(&mut self.last, input);
		self.integral += input;

		self.k_p * input
			+ self.k_i * self.integral
			+ self.k_d * (last - input)
	}
}