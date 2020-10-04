fn get_reg() -> u32 {
	200
}
fn main(){
let status = get_reg();
match status {
	200 => print!("OK"),
	400 => print!("NOT OK"),
	other => {
		print!("NO RESPONSE")	
	} 
}
}
