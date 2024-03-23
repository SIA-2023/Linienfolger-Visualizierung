use ggez::Context;
use ggez::graphics::{Canvas,Color,Mesh,DrawParam,DrawMode};
use ggez::glam::Vec2;

use std::time::Instant;

use crate::controller::ControllerOutput;
use crate::path::Path;

pub struct Car {
	pub position: Vec2,
	orientation: f32,

	pub left_sensor_on_line: bool,
	pub right_sensor_on_line: bool,
	last_sensor_update: Instant,

	last_controller_output: ControllerOutput,
	pub debug_color: Color,
}

const CAR_SPEED: f32 = 300.0;
pub const WHEEL_DISTANCE: f32 = 240.0;
pub const SENSOR_DISTANCE: f32 = 100.0;
pub const SENSOR_RADIUS: f32 = 20.0;
pub const UPDATE_INTERVAL: f32 = 1.0 / 20.0; // 20 fps

const LEFT_SENSOR_OFFSET: Vec2 = Vec2::new(WHEEL_DISTANCE / 2.0, -SENSOR_DISTANCE / 2.0);
const RIGHT_SENSOR_OFFSET: Vec2 = Vec2::new(WHEEL_DISTANCE / 2.0, SENSOR_DISTANCE / 2.0);

impl Car {
	pub fn new(debug_color: Color) -> Self {
		Self {
			position: Vec2::new(0.0, 100.0),
			orientation: 0.0,

			left_sensor_on_line: false,
			right_sensor_on_line: false,
			last_sensor_update: Instant::now(),

			last_controller_output: ControllerOutput::new(0.0, 0.0),
			debug_color,
		}
	}

	pub fn update(&mut self, delta_time: f32, path: &Path, mut controller_output: ControllerOutput) {
		let now = Instant::now();
		if (now - self.last_sensor_update).as_secs_f32() > UPDATE_INTERVAL {
			self.left_sensor_on_line = path.intersects_circle(self.position + rotated_by(LEFT_SENSOR_OFFSET, self.orientation), SENSOR_RADIUS);
			self.right_sensor_on_line = path.intersects_circle(self.position + rotated_by(RIGHT_SENSOR_OFFSET, self.orientation), SENSOR_RADIUS);
			self.last_sensor_update = now;
			controller_output.left_motor = controller_output.left_motor.clamp(0.0, 1.0);
			controller_output.right_motor = controller_output.right_motor.clamp(0.0, 1.0);
			self.last_controller_output = controller_output;
		}

		let left_motor = self.last_controller_output.left_motor * CAR_SPEED;
		let right_motor = self.last_controller_output.right_motor * CAR_SPEED;

		let delta_orientation = ((left_motor - right_motor) / WHEEL_DISTANCE) * delta_time;
		self.orientation += delta_orientation;
		let velocity = (left_motor + right_motor) / 2.0;
		self.position += Vec2::new(f32::cos(self.orientation), f32::sin(self.orientation)) * velocity * delta_time;
	}

	pub fn draw(&self, ctx: &mut Context, canvas: &mut Canvas) {
		// body
		canvas.draw(&ggez::graphics::Quad,
		DrawParam::new()
			.dest(self.position)
			.rotation(self.orientation)
			.scale([WHEEL_DISTANCE; 2])
			.offset([0.5, 0.5])
			.color(self.debug_color),
		);

		// pedals
		canvas.draw(&ggez::graphics::Quad,
			DrawParam::new()
				.dest(self.position)
				.scale([90.0, 30.0])
				.rotation(self.orientation)
				.offset([0.0, 5.0])
				.color(Color::from_rgb(50, 50, 50)),
		);
		canvas.draw(&ggez::graphics::Quad,
			DrawParam::new()
				.dest(self.position)
				.scale([90.0  * self.last_controller_output.left_motor, 30.0])
				.rotation(self.orientation)
				.offset([0.0, 5.0])
				.color(Color::WHITE),
		);
		canvas.draw(&ggez::graphics::Quad,
			DrawParam::new()
				.dest(self.position)
				.scale([90.0, 30.0])
				.rotation(self.orientation)
				.offset([0.0, -4.0])
				.color(Color::from_rgb(50, 50, 50)),
		);
		canvas.draw(&ggez::graphics::Quad,
			DrawParam::new()
				.dest(self.position)
				.scale([90.0 * self.last_controller_output.right_motor, 30.0])
				.rotation(self.orientation)
				.offset([0.0, -4.0])
				.color(Color::WHITE),
		);

		// sensors
		let sensor_circle = Mesh::new_circle(ctx, DrawMode::fill(), [0.0, 0.0], SENSOR_RADIUS, 1.0, Color::WHITE).unwrap();
		canvas.draw(&sensor_circle,
			DrawParam::new()
			.dest(self.position + rotated_by(LEFT_SENSOR_OFFSET, self.orientation))
			.color(if self.left_sensor_on_line { Color::RED } else { Color::GREEN })
		);
		canvas.draw(&sensor_circle,
			DrawParam::new()
			.dest(self.position + rotated_by(RIGHT_SENSOR_OFFSET, self.orientation))
			.color(if self.right_sensor_on_line { Color::RED } else { Color::GREEN })
		);
	}
}



// angle in radians
fn rotated_by(v: Vec2, angle: f32) -> Vec2 {
	let cos = f32::cos(angle);
    let sin = f32::sin(angle);

    Vec2::new(cos * v.x - sin * v.y, sin * v.x + cos * v.y)
}