use std::io;

fn main() {
	let mut input1 = String::new();
	let mut input2 = String::new();
	println!("Units consumed(kwh)   Rates per unit(N) ");
	println!("0-100                  20");
	println!("101-300                35");
	println!("301-above              50");

println!("\nENTER CUSTONER NAME");
io::stdin().read_line(&mut input1).expect("Failed to read input");

println!("ENTER UNIT CONSUMED(in kwh)");
io::stdin().read_line(&mut input2).expect("Failed to read input");
let u:u32=input2.trim().parse().expect("Invalid data type");
	println!("\nCustomers name is {}",input1);
	println!("Unit(in kwh) is {}",u);

if u > 0 && u <= 100{
	let r = 20;
	let total_bill = r * u;
	println!("Rate per unit is {}",r);
	println!("Total bill is {}",total_bill);

} else if u>100 && u <= 300{
	let r = 35;
	let total_bill = r * u;
	println!("Rate per unit is {}",r);
	println!("Total bill is {}",total_bill);

} else if u >300{
	let r = 50;
	let total_bill = r * u;
	if u>500{
		let total_bill= total_bill + 5000;
	}
		
	println!("Rate per unit is {}",r);
	println!("Total bill is {}",total_bill)

}
}
