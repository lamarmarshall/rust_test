
fn main(){
	let words : String = String::from("i am the man baby");
	let wc = |sen : String |{
		let w : Vec<&str> = sen.split(" ").collect();
		w.len()
	}; 
	let length = wc(words);
	println!("{:?}",length);
}
