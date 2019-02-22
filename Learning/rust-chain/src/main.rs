//#[macro_use]
//extern crate serde; // 1.0.85

//#[macro_use]
//extern crate serde_json; // 1.0.37

//#[macro_use]
extern crate serde_derive; // 1.0.37

//use serde::{Serialize, Deserialize};

//use serde_json::{Result, Value};
//use crate::block::Block;



//#[derive(Serialize, Deserialize, Debug)]
//struct Point {
//	x: i32,
//	y: i32,
//}

mod test_serde;

fn main() {
	test_serde::run();

//	let point = Point { x: 1, y: 2 };
//
//	// Convert the Point to a JSON string.
//	let serialized = serde_json::to_string(&point).unwrap();
//
//	// Prints serialized = {"x":1,"y":2}
//	println!("serialized = {}", serialized);
//
//	// Convert the JSON string back to a Point.
//	let deserialized: Point = serde_json::from_str(&serialized).unwrap();
//
//	// Prints deserialized = Point { x: 1, y: 2 }
//	println!("deserialized = {:?}", deserialized);
//	dbg!(deserialized);

//	println!("{:?}", "Hello block chain");
//	let block = Block::new(String::from("Mindaugas"));
//
//	block.calculate_hash();
//
//	println!("{:?}", block.time_stamp);
//
//	let x = 5;
//	let y = x;
//
//	println!("x: {:?}, y: {:?}", x, y);
}