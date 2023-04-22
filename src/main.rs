extern crate mandelbrot;
use image::{RgbImage, Rgb};
use rug::Float;
use std::cmp::{min, max};

fn main() {

	let precision = 77;

	let mut params1 = mandelbrot::Parameters{
		zoom: Float::with_val(precision, 150),
		low_x: Float::with_val(precision, -4.9999999999422825581e-2),
		low_y: Float::with_val(precision, -9.8688569448321587958e-1),
		radius_x: Float::with_val(precision, 1.5),
		radius_y: Float::with_val(precision, 1.5),
		quality: 2024,
		bound: 200.0_f64.powf(2.0),
	};
	
	let last_zoom = Float::with_val(precision, 610987272699921249000.50);

	let map = mandelbrot::initcolormap();

	let mut i = 0;

	loop {

		params1.low_x -= &params1.radius_x;
		params1.low_y -= &params1.radius_y;
		let data = mandelbrot::int_calculate(&params1, precision);
		
		params1.low_x += &params1.radius_x;
		params1.low_y += &params1.radius_y;

		params1.scale(10.0/9.0);
	
		render_image(data, &map, i);
		println!("{i} frames completed, zoom of {}", params1.zoom);
		if params1.zoom > last_zoom {
			break;
		}
		i+=1;
	}
}

fn custom_map() -> Vec<mandelbrot::ReturnColor> {
	let mut thing = vec!();
	
	let color = mandelbrot::return_color(0,0,0);
	let stop = 0.0;
	let cartographer = mandelbrot::cartographer(color, stop);
	thing.push(cartographer);

	let color = mandelbrot::return_color(255,255,255);
	let stop = 255.0;
	let cartographer = mandelbrot::cartographer(color, stop);
	thing.push(cartographer);

	mandelbrot::make_colormap(thing)
}

fn render_image(data: mandelbrot::Storage, map: &[mandelbrot::ReturnColor], order: usize) {
	let mut img = RgbImage::new(data.width as u32, data.height as u32);
	for x in 0..(data.width as u32) {
		for y in 0..(data.height as u32) {
			let num_temp = data.get_value(x as usize, y as usize).unwrap();
			let mut num = max(0, num_temp) as usize;
			num = min(map.len()-1, num);
			img.put_pixel(x, y, Rgb([map[num].r, map[num].g, map[num].b]));
		}
	}
	if order<10 {
		img.save(format!("00{order}.png")).unwrap();
	} else if order < 100 {
		img.save(format!("0{order}.png")).unwrap();
	} else {
		img.save(format!("{order}.png")).unwrap();
	}
}
