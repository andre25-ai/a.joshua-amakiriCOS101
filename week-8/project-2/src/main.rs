use std::io;

struct Developer {
    name: String,
    years: u32,
}

fn main() {
    // Sample interview data stored in a vector of structs
    let applicants = vec![
        Developer { name: "John".to_string(), years: 3 },
        Developer { name: "Amaka".to_string(), years: 5 },
        Developer { name: "Lukman".to_string(), years: 2 },
        Developer { name: "Chidi".to_string(), years: 7 },
        Developer { name: "Fatima".to_string(), years: 4 },
    ];

    // Assume the first person has the highest until proven otherwise
    let mut top = &applicants[0];

    for dev in &applicants {
        if dev.years > top.years {
            top = dev;
        }
    }

    println!("Most experienced developer:");
    println!("Name: {}", top.name);
    println!("Years of experience: {}", top.years);
}

