use std::fs::File;
use std::io::{Write, Result};

// Simple program for storing student info

fn main() -> Result<()> {
    // Using separate vectors (beginner-friendly)
    let names = vec![
        "Oluchi Mordi",
        "Adams Aliyu",
        "Shania Bolade",
        "Adekunle Gold",
        "Blanca Edemoh",
    ];

    let matric_numbers = vec![
        "ACC10211111",
        "ECO10110101",
        "CSC10328828",
        "EEE11020202",
        "MEE10202001",
    ];

    let departments = vec![
        "Accounting",
        "Economics",
        "Computer",
        "Electrical",
        "Mechanical",
    ];

    let scores = vec![300, 310, 280, 200, 270];

    // Print to screen
    println!("PAU SMIS - Student Records");
    println!("-----------------------------------------------");

    for i in 0..names.len() {
        println!(
            "{}  |  {}  |  {}  |  {}",
            names[i], matric_numbers[i], departments[i], scores[i]
        );
    }

    // Save to file
    let mut file = File::create("student_records.txt")?;
    writeln!(file, "PAU SMIS - Student Records")?;
    writeln!(file, "-----------------------------------------------")?;

    for i in 0..names.len() {
        writeln!(
            file,
            "{}  |  {}  |  {}  |  {}",
            names[i], matric_numbers[i], departments[i], scores[i]
        )?;
    }

    println!("Saved to student_records.txt");

    Ok(())
}
