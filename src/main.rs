extern crate mandelbrot;
use image::{RgbImage, Rgb};
use rug::Float;
use std::cmp::{min, max};
use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {

	let (mut params1, increase, last_zoom, precision) = read_things();

	let map = mandelbrot::initcolormap();

	let mut i = 0;

	loop {

		params1.low_x -= &params1.radius_x;
		params1.low_y -= &params1.radius_y;
		let data = mandelbrot::int_calculate(&params1, precision);
		
		params1.low_x += &params1.radius_x;
		params1.low_y += &params1.radius_y;

		params1.scale(increase);
	
		render_image(data, &map, i);
		println!("{i} frames completed, zoom of {}", params1.zoom);
		if params1.zoom > last_zoom {
			break;
		}
		i+=1;
	}

	println!("ffmpeg -framerate 30 -pattern_type glob -i '*.png' -c:v libx264 -pix_fmt yuv420p -b:v 1M out.mp4");
}

fn read_things() -> (mandelbrot::Parameters, f64, Float, u32) {
	/*
	* return params, increase by, final zoom, precision
	*/
	
	let file = File::open("./params.nat").expect("couldnt find file");
	let mut reader = BufReader::new(file);

	let mut floats: Vec<f64> = vec!();

	let mut strs: Vec<String> = vec!("".to_owned(); 10);

	for i in 0..10 {
		reader.read_line(&mut strs[i]).unwrap();
		strs[i] = strs[i].trim().to_owned();
		floats.push(strs[i].parse::<f64>().unwrap());
	}

	let prec = floats[4] as u32;

	let params = mandelbrot::Parameters{
		zoom: Float::with_val(prec, Float::parse(&strs[7]).unwrap()),
		low_x: Float::with_val(prec, Float::parse(&strs[0]).unwrap()),
		low_y: Float::with_val(prec, Float::parse(&strs[1]).unwrap()),
		radius_x: Float::with_val(prec, floats[2]),
		radius_y: Float::with_val(prec, floats[3]),
		quality: floats[5] as usize,
		bound: floats[6],
	};

	for i in 0..10 {
		println!("{}", strs[i]);
	}

	let final_zoom = Float::with_val(prec, floats[8]);

	return (params, floats[9], final_zoom, prec);
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

			let temp = map[num];

			img.put_pixel(x, y, Rgb([temp.r, temp.g, temp.b]));
		}
	}
	if order<10 {
		img.save(format!("images/000{order}.png")).unwrap();
	} else if order < 100 {
		img.save(format!("images/00{order}.png")).unwrap();
	} else if order < 1000{
		img.save(format!("images/0{order}.png")).unwrap();
	} else {
		img.save(format!("images/{order}.png")).unwrap();
	}
}
