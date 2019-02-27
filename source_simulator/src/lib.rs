use std::ops::{Add, Sub};

// Medium constant
const MEDIUM_CONSTANT: f32 = 100.0;

pub trait Geometric {
	fn get_vertices(&self) -> &Vec<Point>;
	fn get_edges(&self) -> &Vec<Edge>;
	fn add_vertex(&mut self, point: Point);
	fn add_edge(&mut self, edge: Edge);
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
pub struct Edge {
	origin: Point,
	terminus: Point,
}
impl Edge {
	pub fn new(origin: Point, terminus: Point) -> Edge {
		Edge {
			origin,
			terminus,
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

	fn encloses(&self, geometry: &Box<dyn Geometric>) -> bool {
		let mut does_enclose = true;
		geometry.get_vertices().iter().for_each(|vertex| does_enclose = does_enclose && vertex >= &self.origin && vertex <= &self.terminus);
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

	pub fn get_dosage_at(&self, position: &Point) -> f32 {
		let mut dosage = 0.0f32;
		for source in self.sources.iter() {
			dosage += MEDIUM_CONSTANT / position.distance_from(&source.origin).powi(2);
		}
		dosage
	}
	pub fn get_sample_at(&self, position: Point) -> Sample {
		let mut dosage = 0.0f32;
		for source in self.sources.iter() {
			dosage += MEDIUM_CONSTANT / position.distance_from(&source.origin).powi(2);
		}
		Sample {
			position,
			dosage,
		}
	}
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}