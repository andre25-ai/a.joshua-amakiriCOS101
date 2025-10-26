use std::io;

fn main () {
	let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("\n ENTER VALUE FOR a");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let a : f32 = input1.trim().parse().expect("Invalid data type");

    println!("\n ENTER VALUE FOR b");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let b : f32 = input2.trim().parse().expect("Invalid data type");

    println!("\n ENTER VALUE FOR c");
    io::stdin().read_line(&mut input3).expect("Failed to read input");
    let c : f32 = input3.trim().parse().expect("Invalid data type");

   let discriminant = (b*b) - ( 4.0 * a * c);

     if discriminant > 0.0{
     	let root_1 = (-b+discriminant.sqrt())/(2.0*a);
     	let root_2 = (-b-discriminant.sqrt())/(2.0 *a);

     	println!("\nThere are two distinct roots");
     	println!("root 1 = {}",root_1);
     	println!("root 2 = {}",root_2);
}
     if discriminant ==0.0 {
     	let real_root=-b/(2.0*a);
     	println!("\nThere is one real root");
     	println!("\nreal root = {}",real_root);
    }

    if discriminant < 0.0 {
    	println!("There is no real root");
    }

 }






