use std::io::Write;
use std::fs::OpenOptions;
fn main() {
	let mut file=OpenOptions::new().append(true).open("data.txt").expect("Failed to append");
	file.write_all("\nhello class".as_bytes()).expect("Failed to write");
	file.write_all("\nThis is appendage to document".as_bytes()).expect("Failed to append");
	println!("File append success");

    
}
