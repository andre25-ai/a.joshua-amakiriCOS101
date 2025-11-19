fn value(n:Option<&char>) {
	println!("Element of vector is {:?}",n);
    
}
fn main () {
	let v = vec!['R','U','S','T','A','C','I','A','N'];

	let mut inpu1=String::new();
	println!("Select a code between 0-8");
	std::io::stdin().read_line(&mut inpu1).expect("Failed to read input");
	let h:usize=inpu1.trim().parse().expect("Invalid data type");

	let g:Option<&char>=v.get(h);
	value(g);
}
