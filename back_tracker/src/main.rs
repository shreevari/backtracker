extern crate csv;

use std::env;
use std::process;
use std::error::Error;

use source_simulator::Point;
use source_simulator::Edge;
use source_simulator::Geometric;

// Simulation space constants
const STARTING_POINT: i32 = 0;
const ENDING_POINT: i32 = 100;



fn main() {
	let mut grid = read_data().unwrap();
}

fn read_data() -> Result<(Vec<Vec<Vec<f32>>>), Box<Error>> {
	
	let file_path = get_first_arg()?;

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

fn get_first_arg() -> Result<String, Box<Error>> {
	match env::args().nth(1) {
		None => Err (From::from("Expected 1 argument but got none")),
		Some(file_path) => Ok(file_path),
	}
}

// #[derive(Debug)]
// struct Sphere {
// 	center: Point,
// 	vertices: Vec<Point>,
// 	edges: Vec<Edge>,
// }
// impl Sphere {
// 	fn new() -> Sphere {
// 		Sphere {
// 			center: Point::new(0, 0, 0),
// 			vertices: Vec::new(),
// 			edges: Vec::new(),
// 		}
// 	}
// 	fn add_edges(&mut self, mut edges: Vec<Edge>) {
// 		self.edges.append(&mut edges);
// 	}
// }
// impl Geometric for Sphere {
// 	fn get_vertices(&self) -> &Vec<Point> {
// 		&self.vertices
// 	}
// 	fn get_edges(&self) -> &Vec<Edge> {
// 		&self.edges
// 	}
// 	fn add_vertex(&mut self, vertex: Point) {
// 		self.vertices.push(vertex);
// 	}
// 	fn add_edge(&mut self, edge: Edge) {
// 		self.edges.push(edge);
// 	}
// }

// fn get_sphere(center: Point, radius: u32) -> Sphere {
// 	let mut sphere = Sphere::new();

// 	// Add all the vertices that form the sphere
// 	for x in (0..(radius as i32)).rev() {
// 		for y in (0..(radius as i32)).rev() {
// 			for z in (0..(radius as i32)).rev() {
// 				if (((x*x).abs() + (y*y).abs() + (z*z).abs()) as f32).sqrt().round() as u32 == radius {
					
// 					for &x_sign in [1_i32, -1_i32].iter() {
// 						for &y_sign in [1_i32, -1_i32].iter() {
// 							for &z_sign in [1_i32, -1_i32].iter() {
// 								sphere.add_vertex(Point::new(
// 									center.x + x_sign * x,
// 									center.y + y_sign * y,
// 									center.z + z_sign * z
// 								));
// 							}
// 						}
// 					}
// 					continue;
// 				}

// 			}
// 		}
// 	}


// 	// Connect vertices to form edges of a shell
// 	let vertices = sphere.get_vertices();
// 	let mut edges = Vec::new();
// 	for vertex in sphere.get_vertices().iter() {

// 		let vertex = Point::new(vertex.x, vertex.y, vertex.z);
// 		for &x_offset in [1_i32, 0, -1_i32].iter() {
// 			for &y_offset in [1_i32, 0, -1_i32].iter() {
// 				for &z_offset in [1_i32, 0, -1_i32].iter() {
					
// 					// Avoiding single point edges
// 					if x_offset == 0 && y_offset == 0 && z_offset == 0 {
// 						continue;
// 					}

// 					let clone_vertex = vertex.clone();
// 					let neighbor = Point::new(
// 						x_offset + vertex.x,
// 						y_offset + vertex.y,
// 						z_offset + vertex.z
// 					);

// 					// Create edges with neighboring vertices
// 					if vertices.contains(&neighbor) {
// 						edges.push(Edge::new(clone_vertex, neighbor));
// 					}

// 				}
// 			}
// 		}

// 	}

// 	// Add the edges created to the sphere
// 	sphere.add_edges(edges);
	
// 	sphere
// }