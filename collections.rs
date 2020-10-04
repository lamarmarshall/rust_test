
use std::collections::HashMap;

fn main(){
	// arrays
	
	let a : [u8; 5] = [1,2,3, 4, 5];
	let floats = [0.1f64, 0.3, 3.3 ];
	println!("Floats: {1:?}\n Ints: {0:?}\n", a, floats);

	for i in &a {
		print!("{}\n", i);
	}

	// tuples

	let tup : (u8, &str) = (9, "have a nice day");
	println!("tuple = {:?}", tup);

	// vectors

	let mut vec = vec![1];
	vec.push(3);
	vec.push(4);
	vec.push(5);

	for i in &vec {
		println!("vec {}", i);
	}

	// Hashmap

	let mut h1 = HashMap::new();
	h1.insert("one", "the number one");
	h1.insert("two", "you are number 2");

	for (k,v) in &h1 {
		println!("{:?} {:?}", k,v);
	}

	// slices

	let mut nums : [u8; 5] = [1,2,3,4,5];
	let numbers: &mut [u8] = &mut nums[0..5];
	numbers[2] = 7;
	println!("{:?} <= slice", numbers);

	
}
