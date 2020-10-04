struct Person {
	name: String,
	age: u8
}

impl Person {

	fn get_name(&self) -> String {
		self.name.to_string()
	}

	fn get_age(&self) -> u8 {
		self.age
	}
}

fn main(){
	let bob : Person = Person{
		name: "Bob dole".to_string(),
		age: 76
	};

	println!("{:?} {:?}", bob.get_name(), bob.get_age());
}
