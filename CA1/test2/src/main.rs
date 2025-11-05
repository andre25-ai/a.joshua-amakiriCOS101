use std::io;

fn main() {
loop{
	let mut input1=String::new();
	let mut input2=String::new();
	let mut input3=String::new();
	let mut input4=String::new();

	println!("STUDENT LOAN REPAYMENT ESTIMATOR");


	println!("\nEnter amount borrowed by student(in naira):");
	io::stdin().read_line(&mut input1).expect("Failed to read input");
	let p :f32 =input1.trim().parse().expect("Invalid data type");

	println!("\nNumber of years:");
	io::stdin().read_line(&mut input2).expect("Failed to read input");
	let t :f32 =input2.trim().parse().expect("Invalid data type");

	println!("Annual rate:");
	io::stdin().read_line(&mut input3).expect("Failed to read input");
	let r:f32 =input3.trim().parse().expect("Invalid data type");

	let a =p *((1.0+r)/100.0).powf(t);
	let i = a-p;
	println!("Local interest is {}",i);
	println!("Amount for loan repayment is {}",a);

println!("DO YOU WANT TO CALCULATE FOR ANOTHER BORROWER:Continue if y or End if n ");
	 io::stdin().read_line(&mut input4).expect("Failed to read input");
	let h =input4.trim().to_lowercase();
	if h == "y"{
		continue;
	} else if h =="n"{
		break;
	} else {
		println!("Invalid input");
		break;
	}
}
}






