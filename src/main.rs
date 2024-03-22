use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{Canvas, Color, DrawParam, Text, TextLayout};
use ggez::conf::{WindowMode, WindowSetup};
use ggez::event::{self, EventHandler};
use ggez::glam::Vec2;

use std::time::Instant;

mod controller;
use controller::{Controller, SimpleController, PIDController};

mod car;
use car::Car;
use car::WHEEL_DISTANCE;

mod path;
use path::Path;

fn main() {
	let (ctx, event_loop) = ContextBuilder::new("Linienfolger-Visualisierung", "Peanutt42")
	.window_mode(WindowMode::default().dimensions(1600.0, 1200.0).resizable(true))
	.window_setup(WindowSetup::default().title("Linienfolger-Visualisierung"))
		.build()
		.expect("could not create GGEZ context");

	event::run(ctx, event_loop, Visualisierung::new());
}

struct Visualisierung {
	last_update_time: Instant,
	path: Path,

	car_controller_map: Vec<(Car, Box<dyn Controller>)>,
}

impl Visualisierung {
	fn new() -> Self {
		Self {
			last_update_time: Instant::now(),
			path: Path::new(PATH_POINTS.into()),
			car_controller_map: vec![
				(Car::new(Color::RED), Box::new(SimpleController::new())),
				(Car::new(Color::GREEN), Box::new(PIDController::new(0.5, 0.9, 0.0))),
			],
		}
	}
}

impl EventHandler for Visualisierung {
	fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
		let now = Instant::now();
		let delta_time = (now - self.last_update_time).as_secs_f32();
		self.last_update_time = now;
		
		for (car, controller) in self.car_controller_map.iter_mut() {
			let output = controller.get_output(car.left_sensor_on_line, car.right_sensor_on_line, delta_time);

			car.update(delta_time, &self.path, output);
		}

		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
		let mut canvas = Canvas::from_frame(ctx, Color::BLACK);

		self.path.draw(ctx, &mut canvas);

		for (car, _controller) in self.car_controller_map.iter() {
			car.draw(ctx, &mut canvas);
		}
		
		for (car, controller) in self.car_controller_map.iter() {
			let mut text = Text::new(controller.get_name());
			text.set_layout(TextLayout::center());
			text.set_scale(25.0);
			canvas.draw(
				&text, 
			DrawParam::new()
				.dest(car.position + Vec2::new(0.0, 0.75 * WHEEL_DISTANCE))
				.color(car.debug_color)
			);
		}

		canvas.finish(ctx)
	}
}



const PATH_POINTS: [Vec2; 92] = [
Vec2::new(116.0 * 2.0, 68.0 * 2.0),
Vec2::new(317.0 * 2.0, 82.0 * 2.0),
Vec2::new(368.0 * 2.0, 29.0 * 2.0),
Vec2::new(454.0 * 2.0, 8.0 * 2.0),
Vec2::new(598.0 * 2.0, 35.0 * 2.0),
Vec2::new(653.0 * 2.0, 77.0 * 2.0),
Vec2::new(700.0 * 2.0, 158.0 * 2.0),
Vec2::new(694.0 * 2.0, 236.0 * 2.0),
Vec2::new(659.0 * 2.0, 275.0 * 2.0),
Vec2::new(562.0 * 2.0, 317.0 * 2.0),
Vec2::new(487.0 * 2.0, 320.0 * 2.0),
Vec2::new(497.0 * 2.0, 388.0 * 2.0),
Vec2::new(440.0 * 2.0, 386.0 * 2.0),
Vec2::new(383.0 * 2.0, 433.0 * 2.0),
Vec2::new(421.0 * 2.0, 514.0 * 2.0),
Vec2::new(368.0 * 2.0, 553.0 * 2.0),
Vec2::new(233.0 * 2.0, 527.0 * 2.0),
Vec2::new(220.0 * 2.0, 383.0 * 2.0),
Vec2::new(169.0 * 2.0, 311.0 * 2.0),
Vec2::new(58.0 * 2.0, 256.0 * 2.0),
Vec2::new(40.0 * 2.0, 170.0 * 2.0),
Vec2::new(59.0 * 2.0, 91.0 * 2.0),
Vec2::new(115.0 * 2.0, 68.0 * 2.0),
Vec2::new(116.0 * 2.0, 68.0 * 2.0),
Vec2::new(317.0 * 2.0, 82.0 * 2.0),
Vec2::new(368.0 * 2.0, 29.0 * 2.0),
Vec2::new(454.0 * 2.0, 8.0 * 2.0),
Vec2::new(598.0 * 2.0, 35.0 * 2.0),
Vec2::new(653.0 * 2.0, 77.0 * 2.0),
Vec2::new(700.0 * 2.0, 158.0 * 2.0),
Vec2::new(694.0 * 2.0, 236.0 * 2.0),
Vec2::new(659.0 * 2.0, 275.0 * 2.0),
Vec2::new(562.0 * 2.0, 317.0 * 2.0),
Vec2::new(487.0 * 2.0, 320.0 * 2.0),
Vec2::new(497.0 * 2.0, 388.0 * 2.0),
Vec2::new(440.0 * 2.0, 386.0 * 2.0),
Vec2::new(383.0 * 2.0, 433.0 * 2.0),
Vec2::new(421.0 * 2.0, 514.0 * 2.0),
Vec2::new(368.0 * 2.0, 553.0 * 2.0),
Vec2::new(233.0 * 2.0, 527.0 * 2.0),
Vec2::new(220.0 * 2.0, 383.0 * 2.0),
Vec2::new(169.0 * 2.0, 311.0 * 2.0),
Vec2::new(58.0 * 2.0, 256.0 * 2.0),
Vec2::new(40.0 * 2.0, 170.0 * 2.0),
Vec2::new(59.0 * 2.0, 91.0 * 2.0),
Vec2::new(115.0 * 2.0, 68.0 * 2.0),
Vec2::new(116.0 * 2.0, 68.0 * 2.0),
Vec2::new(317.0 * 2.0, 82.0 * 2.0),
Vec2::new(368.0 * 2.0, 29.0 * 2.0),
Vec2::new(454.0 * 2.0, 8.0 * 2.0),
Vec2::new(598.0 * 2.0, 35.0 * 2.0),
Vec2::new(653.0 * 2.0, 77.0 * 2.0),
Vec2::new(700.0 * 2.0, 158.0 * 2.0),
Vec2::new(694.0 * 2.0, 236.0 * 2.0),
Vec2::new(659.0 * 2.0, 275.0 * 2.0),
Vec2::new(562.0 * 2.0, 317.0 * 2.0),
Vec2::new(487.0 * 2.0, 320.0 * 2.0),
Vec2::new(497.0 * 2.0, 388.0 * 2.0),
Vec2::new(440.0 * 2.0, 386.0 * 2.0),
Vec2::new(383.0 * 2.0, 433.0 * 2.0),
Vec2::new(421.0 * 2.0, 514.0 * 2.0),
Vec2::new(368.0 * 2.0, 553.0 * 2.0),
Vec2::new(233.0 * 2.0, 527.0 * 2.0),
Vec2::new(220.0 * 2.0, 383.0 * 2.0),
Vec2::new(169.0 * 2.0, 311.0 * 2.0),
Vec2::new(58.0 * 2.0, 256.0 * 2.0),
Vec2::new(40.0 * 2.0, 170.0 * 2.0),
Vec2::new(59.0 * 2.0, 91.0 * 2.0),
Vec2::new(115.0 * 2.0, 68.0 * 2.0),
Vec2::new(116.0 * 2.0, 68.0 * 2.0),
Vec2::new(317.0 * 2.0, 82.0 * 2.0),
Vec2::new(368.0 * 2.0, 29.0 * 2.0),
Vec2::new(454.0 * 2.0, 8.0 * 2.0),
Vec2::new(598.0 * 2.0, 35.0 * 2.0),
Vec2::new(653.0 * 2.0, 77.0 * 2.0),
Vec2::new(700.0 * 2.0, 158.0 * 2.0),
Vec2::new(694.0 * 2.0, 236.0 * 2.0),
Vec2::new(659.0 * 2.0, 275.0 * 2.0),
Vec2::new(562.0 * 2.0, 317.0 * 2.0),
Vec2::new(487.0 * 2.0, 320.0 * 2.0),
Vec2::new(497.0 * 2.0, 388.0 * 2.0),
Vec2::new(440.0 * 2.0, 386.0 * 2.0),
Vec2::new(383.0 * 2.0, 433.0 * 2.0),
Vec2::new(421.0 * 2.0, 514.0 * 2.0),
Vec2::new(368.0 * 2.0, 553.0 * 2.0),
Vec2::new(233.0 * 2.0, 527.0 * 2.0),
Vec2::new(220.0 * 2.0, 383.0 * 2.0),
Vec2::new(169.0 * 2.0, 311.0 * 2.0),
Vec2::new(58.0 * 2.0, 256.0 * 2.0),
Vec2::new(40.0 * 2.0, 170.0 * 2.0),
Vec2::new(59.0 * 2.0, 91.0 * 2.0),
Vec2::new(115.0 * 2.0, 68.0 * 2.0),
];