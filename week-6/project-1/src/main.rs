use std::io;

fn main () {
	println!("           MENU                     PRICE");
	println!("P=Poundo Yam/Edinkaiko Soup         N3200");
	println!("F=Fried Rice & Chicken              N3000");
    println!("A=Amala & Ewedu                     N2500");
    println!("E=Eba & Egusi Soup                  N2000");
    println!("W=White Rice & Stew                 N2000");


    let mut input1=String::new();
    let mut input2=String::new();
    
    println!("FOOD TYPE(USE THE CODE)");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let c =input1.trim().to_uppercase();

    println!("QUANTITY");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let q : u32=input2.trim().parse().expect("Invalid input");

       let  _a: u32;

    if c=="P"{
    	let a = 3200;
    	    let t = q*a;
    if t>10000{
    	let t= t as f32;
    	let t=t*0.95;
    	println!("Discount applied:5%");
    	println!("Total cost is {}",t);
}else {println!("
	Total cost  is {}",t
);}
    } else if c=="F"{
    	let a = 3000;
    	    let t = q*a;
    if t>10000{
    	let t= t as f32;
    	let t=t*0.95;
    	println!("Discount applied:5%");
    	println!("Total cost is {}",t);
}else{ println!("
	Total cost  is {}",t
);}
    } else if c=="A"{
    	let a =2500;
    	    let t = q*a;
    if t>10000{
    	let t= t as f32;
    	let t=t*0.95;
    	println!("Discount applied:5%");
    	println!("Total cost is {}",t);
}else {println!("
	Total cost  is {}",t
);}
    
    }else if c=="E"{
    	let a =2000;
    	    let t = q*a;
    if t>10000{
    	let t= t as f32;
    	let t=t*0.95;
    	println!("Discount applied:5%");
    	println!("Total cost is {}",t);
}else {println!("
	Total cost  is {}",t
);}
    }else if c=="W"{
    	let a =2000;
    	    let t = q*a;
    if t>10000{
    	let t= t as f32;
    	let t=t*0.95;
    	println!("Discount applied:5%");
    	println!("Total cost is {}",t);
}else {println!("
	Total cost  is {}",t
);}
    }else{
    	println!("Invalid code");
    	return;
    }



    
}

    



