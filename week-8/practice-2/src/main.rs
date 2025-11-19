fn main() {
    let v = vec!['C','O','M','P','U','T','E','R'];

    println!("Enter an index btw (0-7)");
    let mut input3=String::new();
    std::io::stdin().read_line(&mut input3).expect("Failed to read input");
    let grape:usize=input3.trim().parse().expect("INVALID DATA TYPE");
let trace:char=v[grape];
println!("{} is the character from index [{}]",trace,grape);

}
