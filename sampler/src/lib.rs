extern crate csv;

use std::error::Error;
use csv::WriterBuilder;

pub fn write_data(file_path: String, grid: Vec<Vec<Vec<f32>>>) -> Result<(), Box<Error>> {
	let mut writer = WriterBuilder::new().from_path(file_path)?;
	
	for plane in grid.iter() {
		for row in plane.iter() {
			writer.write_record(row.iter().map(|val| val.to_string()).collect::<Vec<String>>())?;	
			writer.flush()?;
		}
	}
	Ok(())
}