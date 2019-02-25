extern crate rand;
extern crate csv;

use std::env;
use std::error::Error;
use csv::WriterBuilder;
use rand::Rng;

use source_simulator::Simulator;
use source_simulator::Space;
use source_simulator::Source;
use source_simulator::Point;
use source_simulator::Edge;
use source_simulator::Geometric;

// Simulation space constants
const STARTING_POINT: i32 = 0;
const ENDING_POINT: i32 = 100;

struct PointGeometry {
	vertices: Vec<Point>,
	edges: Vec<Edge>,
}
impl PointGeometry {
	fn new() -> PointGeometry {
		PointGeometry { vertices: Vec::new(), edges: Vec::new() }
	}
	fn from(vertices: Vec<Point>, edges: Vec<Edge>) -> PointGeometry {
		PointGeometry {
			vertices,
			edges,
		}
	}
	fn add_vertex(&mut self, new_vertex: Point) {
		self.vertices.push(new_vertex);
	} 
}

impl Geometric for PointGeometry {
	fn get_vertices(&self) -> &Vec<Point> {
		&self.vertices
	}
	fn get_edges(&self) -> &Vec<Edge> {
		&self.edges
	}
	fn add_vertex(&mut self, vertex: Point) {
		panic!("This is a point source");
	}
	fn add_edge(&mut self, edge: Edge) {
		panic!("This is a point source");
	}
}

fn main() {
	
	// Creation of the space
	let space_start = Point::new(STARTING_POINT, STARTING_POINT, STARTING_POINT);
	let space_end = Point::new(ENDING_POINT, ENDING_POINT, ENDING_POINT);
	let space = Space::new(space_start, space_end);
	
	// Simulator for point source
	let mut point_source_simulator = Simulator::new(space, 1);
	
	let source_position = gen_random_point();
	//println!("Source Position : {:?}", source_position);
	// Creation of Point Geometry
	let mut point_geometry = PointGeometry::new();
	point_geometry.add_vertex(source_position.clone());
	
	// Creation of the radiation source assuming Point Geometry
	let point_source = Source::new(source_position, Box::new(point_geometry));

	// Adding the source to the simulator
	point_source_simulator.add_source(point_source);
	
	// Write the whole grid to a csv file
	//write_full_grid(point_source_simulator);
    
    let random_point = gen_random_point();
    println!("Sample position : {:?}", random_point);
    println!("Simulated Dose : {}", point_source_simulator.get_dosage_at(random_point));
    
}

// Generating random values for the position of the point
fn gen_random_point() -> Point {
	let mut rng = rand::thread_rng();
	let random_x = rng.gen_range(STARTING_POINT, ENDING_POINT);
	let random_y = rng.gen_range(STARTING_POINT, ENDING_POINT);
	let random_z = rng.gen_range(STARTING_POINT, ENDING_POINT);
	Point::new(random_x, random_y, random_z)
}

fn write_full_grid(point_source_simulator: Simulator) {
	let mut grid = Vec::new();
    for x in STARTING_POINT..ENDING_POINT {
    	let mut plane = Vec::new();
    	for y in STARTING_POINT..ENDING_POINT {
    		let mut row = Vec::new();
    		for z in STARTING_POINT..ENDING_POINT {
    			let grid_point = Point::new(x, y, z);
    			let val = point_source_simulator.get_dosage_at(grid_point);
    			row.push(val);
    		}
    		plane.push(row);
    	}
    	grid.push(plane);
    }
    write_data(get_first_arg().unwrap_or(String::from("data.csv")), grid);
}

fn write_data(file_path: String, grid: Vec<Vec<Vec<f32>>>) -> Result<(), Box<Error>> {
	let mut writer = WriterBuilder::new().from_path(file_path)?;
	
	for plane in grid.iter() {
		for row in plane.iter() {
			writer.write_record(row.iter().map(|val| val.to_string()).collect::<Vec<String>>())?;	
			writer.flush()?;
		}
	}
	Ok(())
}

fn write_random_samples(file_path: String, simulator: Simulator, number_of_samples: u32) -> Result<(), Box<Error>> {
	let mut writer = WriterBuilder::new().from_path(file_path)?;
	for _n in 0..number_of_samples {
		let random_point = gen_random_point();
		
	}
	Ok(())
} 

fn get_first_arg() -> Result<String, Box<Error>> {
	match env::args().nth(1) {
		None => Err (From::from("Expected 1 argument but got none")),
		Some(file_path) => Ok(file_path),
	}
}