struct Level(u32, u32, u32);

struct Person {
	name: String,
	age: u8
}

fn get_person(person: Person){
	println!("{}  {}", person.name, person.age)
}

fn main(){
	let l1 = Level(0, 0, 1);
	println!("{} {} {}", l1.0, l1.1, l1.2);

	let p : Person = Person{ name: "bob".to_string(), age: 71};
	get_person(p);
	
}
