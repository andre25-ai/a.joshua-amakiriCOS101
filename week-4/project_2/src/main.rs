
use std::io;

fn main() {
	println!("EMPLOYEE DATABASE");

	let mut input1 =String::new();
	let mut input2 =String::new();

	println!("\nEnter your experience level (experienced or inexperienced:)");
	io::stdin().read_line (&mut input1).expect("Failed to read input");
	let experience=input1.trim().to_lowercase();

	println!("\nEnter your age");
	io::stdin().read_line (&mut input2).expect("Failed to read input");
	let age:u32=input2.trim().parse().expect("Invalid data type");

	let  incentive :i32;

	if experience == "experienced"{
		if age >= 40{
			  incentive = 1_560_000*12;
		} else if age >= 30 && age < 40{
			  incentive = 1_480_000 *12;
	        } else if age < 28 {
		 incentive = 1_300_000*12;
	            } else{
	            println!("Age does not have an incentive");
	            return;
	        }
	       }  else if experience == "inexperienced"{
	      incentive = 100_000 *12;
         } else  {
         	println!("Invalid exprience level");
             return;
         }
    println!("\nAnnual incentive is {}",incentive);





   
}
