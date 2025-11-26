use std::io::Write;

fn main() {
	let announce ="Week 9 - Rust File Input & output\n";
	let dept ="Department of computer science";

	let mut file =std::fs::File::create("data.txt").expect("create failed");
	file.write_all("Welcome to Rust Programming\n".as_bytes()).expect("WRITE FAILED");
	file.write_all(announce.as_bytes()).expect("WRITE FAILED");
    file.write_all(dept.as_bytes()).expect("WRITE FAILED");
	println!("Data written to file");
}
