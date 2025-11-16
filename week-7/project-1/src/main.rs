use std::io;

fn main() {
    println!("MATH 101 AREA / VOLUME CALCULATOR");
    println!("1. Area of Trapezium");
    println!("2. Area of Rhombus");
    println!("3. Area of Parallelogram");
    println!("4. Area of Cube");
    println!("5. Volume of Cylinder");
    println!("Enter your choice:");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    let choice = choice.trim();

    if choice == "1" {
        let h = read_number("Height:");
        let b1 = read_number("Base 1:");
        let b2 = read_number("Base 2:");
        let area = (h / 2.0) * (b1 + b2);
        println!("Area of Trapezium = {}", area);

    } else if choice == "2" {
        let d1 = read_number("Diagonal 1:");
        let d2 = read_number("Diagonal 2:");
        let area = 0.5 * d1 * d2;
        println!("Area of Rhombus = {}", area);

    } else if choice == "3" {
        let base = read_number("Base:");
        let height = read_number("Altitude:");
        let area = base * height;
        println!("Area of Parallelogram = {}",area);

    } else if choice == "4" {
        let l = read_number("Side length:");
        let area = 6.0 * l * l;
        println!("Area of Cube = {}", area);

    } else if choice == "5" {
        let r = read_number("Radius:");
        let h = read_number("Height:");
        let volume = std::f64::consts::PI * r * r * h;
        println!("Volume of Cylinder = {}", volume);

    } else {
        println!("Invalid option.");
    }
}

fn read_number(msg: &str) -> f64 {
    println!("{}", msg);
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

