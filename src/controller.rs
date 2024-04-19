pub trait Controller {
	fn get_output(&mut self, left: bool, right: bool, delta_time_ms: f32) -> ControllerOutput;

	fn get_name(&self) -> String;
}

#[derive(Copy, Clone)]
pub struct ControllerOutput {
	// 0.0 - 1.0
	pub left_motor: f32,
	// 0.0 - 1.0
	pub right_motor: f32,
}
impl ControllerOutput {
	pub fn new(left_motor: f32, right_motor: f32) -> Self {
		Self {
			left_motor,
			right_motor,
		}
	}
}

pub struct SimpleController {
	last_left: bool,
	last_right: bool,
}

impl SimpleController {
	pub fn new() -> Self {
		Self {
			last_left: false,
			last_right: false,
		}
	}
}

impl Controller for SimpleController {
	fn get_output(&mut self, left: bool, right: bool, _delta_time_ms: f32) -> ControllerOutput {
		if !left && !right {
			if self.last_left {
				return ControllerOutput::new(0.0, 1.0);
			}
			else if self.last_right {
				return ControllerOutput::new(1.0, 0.0);
			}
			else {
				return ControllerOutput::new(1.0, 1.0);
			}
		}
		else {
			self.last_left = false;
			self.last_right = false;
			if left {
				self.last_left = true;
			}
			if right {
				self.last_right = true;
			}
		}

		if left && !right {
			return ControllerOutput::new(0.0, 1.0);
		}
		if right && !left {
			return ControllerOutput::new(1.0, 0.0);
		}

		ControllerOutput::new(1.0, 1.0)
	}

	fn get_name(&self) -> String {
		"SimpleController".to_string()
	}
}


// kd ist nicht zu empfehlen (kd = 0.0), da wir immer nur einen fehler von 1.0, 0.0 oder -1.0
// haben können (nur 2 Sensoren!). Somit ist die Änderung des Fehlers sehr ungenau
// und repräsentiert nicht die Änderung des Fehlers von der Linie
// max_i verhindert, dass integral zu groß wird, sodass sich der output nie ändert
pub struct PIDController {
	kp: f64,
	ki: f64,
	kd: f64,
	max_i: f64,
	prev_error: f64,
	integral: f64,
	last_left: bool,
	last_right: bool,
}

impl PIDController {
	pub fn new(kp: f64, ki: f64, kd: f64, max_i: f64) -> Self {
		Self {
			kp,
			ki,
			kd,
			max_i,
			prev_error: 0.0,
			integral: 0.0,
			last_left: false,
			last_right: false,
		}
	}

	fn update_pid(&mut self, left: bool, right: bool, delta_time_ms: f32) -> f64 {
		let error = if left {
			1.0
		}
		else if right {
			-1.0
		}
		else {
			0.0
		};
		self.integral += error * (delta_time_ms as f64);
		self.integral = self.integral.clamp(-self.max_i, self.max_i);
        let derivative = (error - self.prev_error) / (delta_time_ms as f64);
		let output = self.kp * error + self.ki * self.integral + self.kd * derivative;
        self.prev_error = error;
		output
	}
}

impl Controller for PIDController {
	fn get_output(&mut self, left: bool, right: bool, delta_time_ms: f32) -> ControllerOutput {
		if !left && !right {
			let _ = self.update_pid(self.last_left, self.last_right, delta_time_ms);
			if self.last_left {
				return ControllerOutput::new(0.0, 1.0);
			}
			else if self.last_right {
				return ControllerOutput::new(1.0, 0.0);
			}
			else {
				return ControllerOutput::new(1.0, 1.0);
			}
		}
		else {
			self.last_left = false;
			self.last_right = false;
			if left {
				self.last_left = true;
			}
			if right {
				self.last_right = true;
			}
		}

		let output = self.update_pid(left, right, delta_time_ms);
		if output > 0.0 {
			ControllerOutput::new(1.0 - output as f32, 1.0)
		}
		else {
			ControllerOutput::new(1.0, 1.0 + output as f32)
		}
	}

	fn get_name(&self) -> String {
		format!("PIDController: kp={}, ki={}, kd={}, max_i={}", self.kp, self.ki, self.kd, self.max_i)
	}
}