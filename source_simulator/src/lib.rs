use std::ops::{Add, Sub};

// Medium constant
const MEDIUM_CONSTANT: f32 = 111.0;

pub enum Dimension {
	OneDimensional,
	TwoDimensional,
	ThreeDimensional,
}

pub trait Geometric {
	fn get_corners(&self) -> &Vec<Point>;
	fn get_surfaces(&self) -> &Vec<Surface>;
	fn add_corner(&mut self, point: Point);
	fn add_surface(&mut self, surface: Surface);
	fn get_dimension(&self) -> Dimension;
}

pub struct Source {
	origin: Point,
	geometry: Box<dyn Geometric>,
}
impl Source {
	pub fn new(origin: Point, geometry: Box<dyn Geometric>) -> Source {
		Source {
			origin,
			geometry,
		}
	}
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Point {
	pub x: i32,
	pub y: i32,
	pub z: i32,
}

impl Point {
	pub fn new(x: i32, y: i32, z: i32) -> Point {
		Point { x, y, z }
	}

	pub fn distance_from(&self, other: &Point) -> f32 {
		let value = ( (self.x - other.x).abs().pow(2)
		+ (self.y - other.y).abs().pow(2)
		+ (self.z - other.z).abs().pow(2) ) as f32;
		value.sqrt()
	}
}

impl Add for Point {
	type Output = Point;

	fn add(self, other: Point) -> Point {
		Point::new (
			self.x + other.x,
			self.y + other.y,
			self.z + other.z
		)
	}
}
impl Sub for Point {
	type Output = Point;

	fn sub(self, other: Point) -> Point {
		Point::new(
			self.x - other.x,
			self.y - other.y,
			self.z - other.z,	
		)
	}
}
#[derive(Debug)]
pub struct Direction {
	theta: i32,
	phi: i32,
}

impl Direction {
	pub fn new(vertices: &Vec<Point>, center: &Point) -> Direction {
		let theta = 0;
		let phi = 0;
		assert!(vertices.len() >= 3);
		for vertex in vertices.iter().take(3) {
			// Calculate equation of the plane
			// Calculate equation of perpendicular
			// Calculate inclination with respect to center point
		}
		Direction {
			theta,
			phi,
		}
	}
}

#[derive(Debug)]
pub struct Surface {
		vertices: Vec<Point>,
		direction: Direction, 
}
impl Surface {
	pub fn new(vertices: Vec<Point>, direction: Direction) -> Surface {
		Surface {
			vertices,
			direction,
		}
	}
}

pub struct Space {
	origin: Point,
	terminus: Point,
}

impl Space {
	pub fn new(origin: Point, terminus: Point) -> Space {
		Space {
			origin,
			terminus,
		}
	}
	// Check for dimension integrity
	fn encloses(&self, geometry: &Box<dyn Geometric>) -> bool {
		let mut does_enclose = true;
		geometry.get_corners().iter().for_each(|corner| does_enclose = does_enclose && corner >= &self.origin && corner <= &self.terminus);
		does_enclose
	} 	
}
#[derive(Debug)]
pub struct Sample {
	position: Point,
	dosage: f32,
}

impl Sample {
	pub fn new(position: Point, dosage: f32) -> Sample {
		Sample {
			position,
			dosage,
		}
	}
	pub fn get_position(&self) -> &Point {
		&self.position
	}
	pub fn get_x(&self) -> &i32 {
		&self.position.x
	}
	pub fn get_y(&self) -> &i32 {
		&self.position.y
	}
	pub fn get_z(&self) -> &i32 {
		&self.position.z
	}
	pub fn get_dosage(&self) -> &f32 {
		&self.dosage
	}

}

pub struct Simulator {
	space: Space,
	unit: i32,
	sources: Vec<Source>,
}

impl Simulator {
	
	pub fn new(space: Space, unit: i32) -> Simulator {
		Simulator {
			space,
			unit,
			sources: vec![],
		}
	}

	pub fn add_source(&mut self, source: Source) {
		assert!(self.space.encloses(&source.geometry));
		self.sources.push(source);
	}

	//Check for dimensional radiation
	pub fn get_dosage_at(&self, position: &Point) -> f32 {
		let mut dosage = 0.0f32;
		for source in self.sources.iter() {
			
			match source.geometry.get_dimension() {
				Dimension::OneDimensional => {
					dosage += MEDIUM_CONSTANT / position.distance_from(&source.origin).powi(2);
				},
				Dimension::TwoDimensional => {
					dosage += calc_2d_dosage(&source.geometry.get_corners(), position); 
				},
				Dimension::ThreeDimensional => {
					dosage += calc_3d_dosage(&source.geometry.get_corners(), position);
				},

			}

		}
		dosage
	}

	pub fn get_sample_at(&self, position: Point) -> Sample {
		let dosage = self.get_dosage_at(&position);
		Sample {
			position,
			dosage,
		}
	}
}

fn calc_2d_dosage(corners: &Vec<Point>, position: &Point) -> f32 {
	let m = get_perpendicular_distance(&corners[0],&corners[1], position);

	let mut l1 = 0.0f32;
	let mut l2 = 0.0f32; 
	if corners[0].y == corners[1].y && corners[0].z == corners[1].z {
		l1 = position.x as f32;
		l2 =  1.0f32 - position.x as f32;
	} else if corners[0].x == corners[1].x && corners[0].z == corners[1].z {
		l1 = position.y as f32;
		l2 =  1.0f32 - position.y as f32;
	} else if corners[0].x == corners[1].x && corners[0].y== corners[1].y {
		l1 = position.z as f32;
		l2 =  1.0f32 - position.z as f32;
	};
	MEDIUM_CONSTANT * (l1 / m).atan() + (l2 / m).atan() / m
}

// fn get_equation_of_line(point_1: &Point, point_2: &Point) -> (f32,f32) {
// 	( (point_2.y as f32 - point_1.y as f32) / (point_2.x as f32 - point_1.x as f32), 0.0f32 )
// }

fn get_perpendicular_distance(source: &Point, terminus: &Point, point: &Point) -> f32 {
	let theta = (terminus.y - source.y) / (terminus.x - source.x); 
	let phi = 0.0f32;
	let mut distance = 0.0f32;
	if source.y == terminus.y && source.z == terminus.z {
		distance = ((source.y - point.y).pow(2) as f32 + (source.z - point.z).pow(2) as f32).sqrt();
	} else if source.x == terminus.x && source.z == terminus.z {
		distance = ((source.x - point.x).pow(2) as f32 + (source.z - point.z).pow(2) as f32).sqrt();
	} else if source.x == terminus.x && source.y== terminus.y {
		distance = ((source.x - point.x).pow(2) as f32 + (source.y - point.y).pow(2) as f32).sqrt();
	} else {
		distance = 0.0f32;
	}
	distance
}
fn calc_3d_dosage(corners: &Vec<Point>, position: &Point) -> f32 {
	0.0f32
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}