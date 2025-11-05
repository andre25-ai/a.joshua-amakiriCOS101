use std::io;

fn main() {
	
	println!("Code   Item    Price(N))");
	println!("T      Tea      800");
	println!("C      Coffee   1200");
	println!("S      Sandwich 2000");
	println!("J      Juice    1500");
	loop{

	let mut input1=String::new();
	let mut input2=String::new();
	let mut input3=String::new();

	println!("\nENTER ITEM CODE[MUST BE IN CAPITAL LETTER]");
	io::stdin().read_line(&mut input1).expect("Failed to read input");
	let i = input1.trim().to_uppercase();
	    

	println!("Enter quantity");
	io::stdin().read_line(&mut input2).expect("Failed to read input");
	let q :u32 =input2.trim().parse().expect("Invalid data type");

	if i == "T" {
		let p = 800;
		let c = p*q;
	    if c>=5000{
	    	let c= c-((5/100)*c);
	    	println!("5% Discount is applied");
	    }
	    	println!("Total amount is {}",c);
	    
	} else if i == "C" {
		let p = 1200;
		let c = p*q;
	    if c>=5000{
	    	let c= c-((5/100)*c);
	    	println!("5% Discount is applied");
	    }
	    	println!("Total amount is {}",c);
	    
	} else if i == "S" {
		let p = 2000;
		let c = p*q;
	    if c>=5000{
	    	let c= c-((5/100)*c);
	    	println!("5% Discount is applied");
	    }
	    	println!("Total amount is {}",c);
	    
	} else if i == "J" {
		let p = 1500;
		let c = p*q;
	    if c>=5000{
	    	let c= c-((5/100)*c);
	    	println!("5% Discount is applied");
	    }
	    	println!("Total amount is {}",c);
	    
	} else{
		println!("Invalid code");
	}
	println!("If you want to stop[TYPE EXIT]/if you want to continue(press any key)");
	io::stdin().read_line(&mut input3).expect("Failed to read input");
	let z = input3.trim().to_uppercase();
	if z=="EXIT"{break;}
		else{continue;}
}
}



