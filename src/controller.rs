pub trait Controller {
	fn get_output(&mut self, left: bool, right: bool, delta_time: f32) -> ControllerOutput;
}

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

pub struct SimpleController {}

impl SimpleController {
	pub fn new() -> Self {
		Self {}
	}
}

impl Controller for SimpleController {
	fn get_output(&mut self, left: bool, right: bool, _delta_time: f32) -> ControllerOutput {
		if left && !right {
			return ControllerOutput::new(0.0, 1.0);
		}
		if right && !left {
			return ControllerOutput::new(1.0, 0.0);
		}

		ControllerOutput::new(1.0, 1.0)
	}
}


// kd ist nicht zu empfehlen (kd = 0.0), da wir immer nur einen fehler von 1.0, 0.0 oder -1.0
// haben können (nur 2 Sensoren!). Somit ist die Änderung des Fehlers sehr ungenau
// und repräsentiert nicht die Änderung des Fehlers von der Linie
pub struct PIDController {
	kp: f64,
	ki: f64,
	kd: f64,
	prev_error: f64,
	integral: f64,
	last_left: bool,
	last_right: bool,
}

impl PIDController {
	pub fn new(kp: f64, ki: f64, kd: f64) -> Self {
		Self {
			kp,
			ki,
			kd,
			prev_error: 0.0,
			integral: 0.0,
			last_left: false,
			last_right: false,
		}
	}
}

impl Controller for PIDController {
	fn get_output(&mut self, left: bool, right: bool, delta_time: f32) -> ControllerOutput {
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

		let error = if left {
			1.0
		}
		else if right {
			-1.0
		}
		else {
			0.0
		};
		self.integral += error * (delta_time as f64);
        let derivative = (error - self.prev_error) / (delta_time as f64);
        let output = self.kp * error + self.ki * self.integral + self.kd * derivative;
        self.prev_error = error;
		if output > 0.0 {
			ControllerOutput::new(1.0 - output as f32, 1.0)
		}
		else {
			ControllerOutput::new(1.0, 1.0 + output as f32)
		}
	}
}