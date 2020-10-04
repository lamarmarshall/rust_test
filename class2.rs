struct Location {
	lat: i32,
	long: i32
}

impl Location {
	fn new(lat: i32, long: i32) -> Location {
		Location {
			lat: lat,
			long: long
		}
	}

	fn get_coords(&self){
		println!("Latitude: {} Longitude: {}", self.lat, self.long);
	}
}

fn main(){
	let laredo : Location = Location::new(
		33456,
		-13495
	);

	laredo.get_coords();
}
