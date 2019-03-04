extern crate csv;

use std::env;
use std::error::Error;

use sampler::write_data;
use sampler::basics;

use source_simulator::Sample;
use source_simulator::Point;
//use source_simulator::Edge;
use source_simulator::Geometric;

// Simulation space constants
const STARTING_POINT: i32 = 0;
const ENDING_POINT: i32 = 100;
// Medium constant
const MEDIUM_CONSTANT: f32 = 111.0;
// Threshold
const THRESHOLD: f32 = 0.001;

fn main() {
	//let mut grid = read_grid().unwrap();
	let file_path = get_first_arg().unwrap_or(String::from("data.csv"));
	let samples = read_samples(file_path);
	//println!("{:?}", get_probability_distribution(samples.unwrap()));
	write_data(String::from("PD.csv"), get_probability_distribution(samples.unwrap()).unwrap());

}


fn read_samples(file_path: String) -> Result<Vec<Sample>, Box<Error>> {

	let mut rdr = csv::ReaderBuilder::new()
		.has_headers(true)
		.from_path(file_path)?;

	let mut samples = Vec::new();

	for result in rdr.records() {
		let mut record = result?;
		let dosage: f32 = record.get(3).unwrap().parse().unwrap();
		let mut record = record.iter().take(3).map(|val| val.parse::<i32>().unwrap());
		samples.push(Sample::new( 
					Point::new(record.next().unwrap(), record.next().unwrap(), record.next().unwrap()),
					dosage));
	}

	Ok(samples)
}

fn read_grid(file_path: String) -> Result<Vec<Vec<Vec<f32>>>, Box<Error>> {

	let mut rdr = csv::ReaderBuilder::new()
		.has_headers(false)
		.from_path(file_path)?;

	let mut grid: Vec<Vec<Vec<f32>>> = Vec::new();
	let mut plane: Vec<Vec<f32>> = Vec::new();

	for (index, row) in rdr.records().enumerate() {
		plane.push(row.unwrap().iter().map(|val| val.to_string().parse::<f32>().unwrap()).collect());
		if index as i32 % (ENDING_POINT-STARTING_POINT) == 0 {
			grid.push(plane);
			plane = Vec::new();
		}
	}
	
	Ok(grid)
}

fn get_probability_distribution(samples: Vec<Sample>) -> Result<Vec<Vec<Vec<f32>>>, Box<Error>> {
	let mut p_d = Vec::new();
	let mut max = 0.0f32;
	let mut max_point = Point::new(0, 0, 0);

	for x in STARTING_POINT..ENDING_POINT {
		let mut planes = Vec::new();
		for y in STARTING_POINT..ENDING_POINT {
			let mut row = Vec::new();
			for z in STARTING_POINT..ENDING_POINT {

				let mut intensity = 0.0f32;

				let point = Point::new(x, y, z);
				for sample in samples.iter() {
					
					let radius = 0.0f32;
					//println!("{}\n", radius);
					let epsilon = (point.distance_from(sample.get_position()) - radius).abs();
					if epsilon <= THRESHOLD {
						intensity += 1.0;
						if intensity > max {
							max = intensity;
							max_point = Point::new(x, y, z);
							println!("{:?}", max_point);
						}
					}
				}

				row.push(intensity);
			}
			planes.push(row);
			row = Vec::new();
		}
		p_d.push(planes);
		planes = Vec::new();
	}


	p_d = p_d.iter().map(|plane| 
		plane.iter().map(|row| 
			row.iter().map(|intensity| intensity / max
				).collect()
			).collect()
		).collect();

	// let mut intermediate_value = basics(p_d.into_iter().flatten().flatten().collect::<Vec<f32>>(), max).unwrap();	

	// let mut p_d = Vec::new();
	// for x in (STARTING_POINT..ENDING_POINT) {
	// 	let mut plane = Vec::new();
	// 	for y in (STARTING_POINT..ENDING_POINT) {
	// 		let mut row = Vec::new();
	// 		for z in (STARTING_POINT..ENDING_POINT) {
	// 			row.push(intermediate_value[z as usize]);
	// 		}
	// 		plane.push(row);
	// 	}
	// 	p_d.push(plane);
	// }

	//println!("{:?}", p_d.into_iter().flatten().flatten().collect::<Vec<f32>>());	

	//println!("The source is (Probably) at : {:?}", max_point);	
	
	Ok(p_d)
	//Ok(Vec::new())
}

fn get_first_arg() -> Result<String, Box<Error>> {
	match env::args().nth(1) {
		None => Err (From::from("Expected 1 argument but got none")),
		Some(file_path) => Ok(file_path),
	}
}