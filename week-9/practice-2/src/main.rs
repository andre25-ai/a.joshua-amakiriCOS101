use std::io::Read;
fn main() {

let mut more=std::fs::File::open("../practice-1/data.txt").unwrap();
let mut content=String::new();
more.read_to_string(&mut content).unwrap();
println!("{}",content);
}
