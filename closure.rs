
fn main(){
	let double = |x| x * 2;
	print!("\ndoubled = {}\n", double(15));

	let fth = double(5);
	let big_math = |b, c| {
		let z = b * c;
		z * fth
	};
	let num = big_math(40, 40);
	println!("\nbig math is = {:?}", num);
}
