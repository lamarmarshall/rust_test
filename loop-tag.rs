fn main(){
	let mut x : u32 = 10;
	let mut sec = false;
	loop {
		print!("good");
		x -= 1;
		if x < 0 {
			if sec {
				break;
			}
			break 'l2;
		}
			loop {
		print!("bad");
		x += 1;
		if x > 10 {
			sec = true;
			break 'l1;
		}
	}
	}


}
