fn main() {

	//** Variable shadowing **//
	let some_var: &str = "some string";			// decklare variable 
	println!("some_var: {:?}", some_var);

	// some_var = some_var.len();					// Error. NOT mutable variable

	let some_var: usize = some_var.len();		// OK. Variable shadowing. Note differrent type!
	println!("some_var: {:?}", some_var);

	//** Constants **//
	const PI: f64 = 3.14;						// constant
	println!("PI: {:?}", PI);

	// const PI: f64 = 3.1415;						// Error. Constants can't be shadowed

	// Overflow
    // let mut byte: u8 = 255;
    // byte += 1;
    // println!("byte: {:?}", byte);				// Error. thread 'main' panicked at 'attempt to add with overflow'
}
