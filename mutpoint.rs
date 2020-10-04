fn update_score(mut val: u32, how_much: u32){
	val += how_much;
	print!("result = {}\n", val);
}

fn main(){

let score : u32 = 50;
update_score(score, 45);

}
