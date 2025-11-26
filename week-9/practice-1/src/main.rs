use std::io::Write;

fn main() {
	let announce ="\nWeek 9 - Rust File Input & output";
	let dept ="\nDepartment of computer science";

	let mut file =std::fs::File::create("data.txt").expect("create failed");
	file.write_all("Welcome to Rust Programming".as_bytes()).expect("WRITE FAILED");
	file.write_all(announce.as_bytes()).expect("WRITE FAILED");
    file.write_all(dept.as_bytes()).expect("WRITE FAILED");
	println!("Data written to file");
}
