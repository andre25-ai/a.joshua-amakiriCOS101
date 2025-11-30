use std::fs::File;
use std::io::{Write, Result};

fn main() -> Result<()> {
    // Separate datasets
    let commissioners = vec![
        "Aigboqun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Caliistus Ogbona",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etieve",
    ];

    let ministries = vec![
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum",
    ];

    let zones = vec![
        "South West",
        "North East",
        "South South",
        "South West",
        "South East",
    ];

    println!("Merged EFCC Dataset\n");
    println!("S/N | Commissioner Name              | Ministry           | Zone");
    println!("---------------------------------------------------------------");

    for i in 0..commissioners.len() {
        println!(
            "{}   | {} | {} | {}",
            i + 1,
            commissioners[i],
            ministries[i],
            zones[i]
        );
    }

    // Save to file
    let mut file = File::create("efcc_merged_records.txt")?;

    writeln!(file, "Merged EFCC Dataset")?;
    writeln!(file, "S/N | Commissioner Name              | Ministry           | Zone")?;
    writeln!(file, "---------------------------------------------------------------")?;

    for i in 0..commissioners.len() {
        writeln!(
            file,
            "{}   | {} | {} | {}",
            i + 1,
            commissioners[i],
            ministries[i],
            zones[i]
        )?;
    }

    println!("\nData saved to efcc_merged_records.txt");

    Ok(())
}

