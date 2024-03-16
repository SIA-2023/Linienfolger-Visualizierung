use ggez::Context;
use ggez::graphics::{Canvas,DrawParam,Mesh,Color};
use ggez::glam::Vec2;
use crate::car::{SENSOR_DISTANCE, SENSOR_RADIUS};

pub struct Path {
	points: Vec<Vec2>,
}

const LINE_WIDTH: f32 = SENSOR_DISTANCE - 2.0 * SENSOR_RADIUS;

impl Path {
	pub fn new(points: Vec<Vec2>) -> Self {
		Self {
            points,
        }
	}

	fn distance_to_point(line_point1: &Vec2, line_point2: &Vec2, point: &Vec2) -> f32 {
		let line_length_squared = line_point1.distance_squared(*line_point2);

		if line_length_squared == 0.0 {
			return point.distance(*line_point1);
		}

		let t = f32::max(0.0, f32::min(1.0, ((point.x - line_point1.x) * (line_point2.x - line_point1.x) + (point.y - line_point1.y) * (line_point2.y - line_point1.y)) / line_length_squared));

		let closest_x = line_point1.x + t * (line_point2.x - line_point1.x);
		let closest_y = line_point1.y + t * (line_point2.y - line_point1.y);

		point.distance(Vec2::new(closest_x, closest_y))
	}

	pub fn intersects_circle(&self, center: Vec2, radius: f32) -> bool {
		for i in 0..self.points.len() - 1 {
			let point1 = &self.points[i];
			let point2 = &self.points[i+1];

			if Self::distance_to_point(point1, point2, &center) <= (radius + (LINE_WIDTH / 2.0)) {
				return true;
			}
		}

		false
	}

	pub fn draw(&self, ctx: &mut Context, canvas: &mut Canvas) {
		let lines = Mesh::new_line(ctx, &self.points, LINE_WIDTH, Color::WHITE).unwrap();

		canvas.draw(&lines, DrawParam::new());
	}
}