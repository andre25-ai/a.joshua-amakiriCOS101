fn main() {
    let name=vec!["Chioma","Joshua","Rikky","Eberechi","Clinton","Chudi"];

    let age = vec![16,17,18,19,20];

    for i in 0..age.len(){
    	println!("\nAge allocation");
    	println!("{} is {}years old",name[i],age[i])
    }
}
