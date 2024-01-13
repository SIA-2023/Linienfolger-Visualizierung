use ggez::Context;
use ggez::graphics::{Canvas,Color,Mesh,DrawParam,DrawMode,Image};
use ggez::glam::Vec2;

use std::time::Instant;

use crate::controller::ControllerOutput;
use crate::path::Path;

pub struct Car {
	position: Vec2,
	orientation: f32,

	pub left_sensor_on_line: bool,
	pub right_sensor_on_line: bool,
	last_sensor_update: Instant,

	texture: Image,
}

const CAR_SPEED: f32 = 150.0;
const WHEEL_DISTANCE: f32 = 120.0;
const SENSOR_DISTANCE: f32 = 60.0;
const SENSOR_RADIUS: f32 = 10.0;
const SENSOR_UPDATE_INTERVAL: f32 = 1.0 / 15.0; // 15 fps

const LEFT_SENSOR_OFFSET: Vec2 = Vec2::new(WHEEL_DISTANCE / 2.0, -SENSOR_DISTANCE / 2.0);
const RIGHT_SENSOR_OFFSET: Vec2 = Vec2::new(WHEEL_DISTANCE / 2.0, SENSOR_DISTANCE / 2.0);

impl Car {
	pub fn new(ctx: &mut Context) -> Self {
		Self {
			position: Vec2::new(0.0, 100.0),
			orientation: 0.0,

			left_sensor_on_line: false,
			right_sensor_on_line: false,
			last_sensor_update: Instant::now(),

			texture: Image::from_bytes(ctx, include_bytes!("../images/car.png")).unwrap(),
		}
	}

	pub fn update(&mut self, delta_time: f32, path: &Path, controller_output: &ControllerOutput) {
		let left_motor = controller_output.left_motor * CAR_SPEED;
		let right_motor = controller_output.right_motor * CAR_SPEED;

		let delta_orientation = ((left_motor - right_motor) / WHEEL_DISTANCE) * delta_time;
		self.orientation += delta_orientation;
		let velocity = (left_motor + right_motor) / 2.0;
		self.position = self.position + (Vec2::new(f32::cos(self.orientation), f32::sin(self.orientation)) * velocity * delta_time);
	
		// test sensors
		let now = Instant::now();
		if (now - self.last_sensor_update).as_secs_f32() > SENSOR_UPDATE_INTERVAL {
			self.left_sensor_on_line = path.intersects_circle(self.position + rotated_by(LEFT_SENSOR_OFFSET, self.orientation), SENSOR_RADIUS);
			self.right_sensor_on_line = path.intersects_circle(self.position + rotated_by(RIGHT_SENSOR_OFFSET, self.orientation), SENSOR_RADIUS);
			self.last_sensor_update = now;
		}
	}

	pub fn draw(&self, ctx: &mut Context, canvas: &mut Canvas) {
		canvas.draw(&self.texture,
		DrawParam::new()
			.dest(self.position)
			.rotation(self.orientation)
			.scale([WHEEL_DISTANCE / self.texture.width() as f32, WHEEL_DISTANCE / self.texture.height() as f32])
			.offset([0.5, 0.5])
			.color(ggez::graphics::Color::WHITE),
		);

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