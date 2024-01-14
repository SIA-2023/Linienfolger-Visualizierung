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

pub struct ConservativeController {}

impl ConservativeController {
	pub fn new() -> Self {
		Self {}
	}
}

impl Controller for ConservativeController {
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


pub struct TimeCorrectingController {
	time_off_line: f32,
}

impl TimeCorrectingController {
	pub fn new() -> Self {
		Self {
			time_off_line: 0.0,
		}
	}
}

impl Controller for TimeCorrectingController {
	fn get_output(&mut self, left: bool, right: bool, delta_time: f32) -> ControllerOutput {
		if !left || !right {
			self.time_off_line += delta_time;
		}
		else {
			self.time_off_line = 0.0;
		}

		let max_time = 0.5;

		// 0..1
		let error = self.time_off_line.clamp(0.0, max_time) / max_time;

		if left && !right {
			return ControllerOutput::new(1.0 - error, 1.0);
		}
		if right && !left {
			return ControllerOutput::new(1.0, 1.0 - error);
		}

		ControllerOutput::new(1.0, 1.0)
	}
}