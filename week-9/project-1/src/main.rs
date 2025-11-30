use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    // Drink categories from the table
    let lager = vec![
        "33 Export",
        "Desperados",
        "Goldberg",
        "Gulder",
        "Heineken",
        "Star",
    ];

    let stout = vec![
        "Legend",
        "Turbo King",
        "Williams",
    ];

    let non_alcoholic = vec![
        "Maltina",
        "Amstel Malta",
        "Malta Gold",
        "Fayrouz",
    ];

    // Create the output file
    let mut file = File::create("drink_categories.txt")?;

    // Write categories in a clean, readable format
    writeln!(file, "Lager Drinks:")?;
    for item in &lager {
        writeln!(file, " - {}", item)?;
    }

    writeln!(file, "
Stout Drinks:")?;
    for item in &stout {
        writeln!(file, " - {}", item)?;
    }

    writeln!(file, "
Non‑Alcoholic Drinks:")?;
    for item in &non_alcoholic {
        writeln!(file, " - {}", item)?;
    }

    println!("drink_categories.txt created successfully.");
    Ok(())
}
