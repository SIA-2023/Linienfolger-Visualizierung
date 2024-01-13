pub trait Controller {
	fn get_output(&mut self, left: bool, right: bool) -> ControllerOutput;
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
	fn get_output(&mut self, left: bool, right: bool) -> ControllerOutput {
		if left && right || !left && !right {
			return ControllerOutput::new(1.0, 1.0);
		}
	
		if left && !right {
			return ControllerOutput::new(0.0, 1.0);
		}
		if right && !left {
			return ControllerOutput::new(1.0, 0.0);
		}

		ControllerOutput::new(1.0, 1.0)
	}
}