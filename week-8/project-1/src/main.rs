use std::io;

fn main() {
    // Each entry: (profession, job title, APS level)
    let roles = vec![
        // Admin
        ("admin", "intern", "APS 1–2"),
        ("admin", "administrator", "APS 3–5"),
        ("admin", "senior administrator", "APS 5–8"),
        ("admin", "office manager", "EL 1"),
        ("admin", "director", "EL 2"),
        ("admin", "ceo", "SES"),

        // Academic
        ("academic", "research assistant", "APS 3–5"),
        ("academic", "phd candidate", "APS 5–8"),
        ("academic", "post-doc researcher", "EL 1"),
        ("academic", "senior lecturer", "EL 2"),
        ("academic", "dean", "SES"),

        // Lawyer
        ("lawyer", "paralegal", "APS 1–2"),
        ("lawyer", "junior associate", "APS 3–5"),
        ("lawyer", "associate", "APS 5–8"),
        ("lawyer", "senior associate 1-2", "EL 1"),
        ("lawyer", "senior associate 3-4", "EL 2"),
        ("lawyer", "partner", "SES"),

        // Teacher
        ("teacher", "placement", "APS 1–2"),
        ("teacher", "classroom teacher", "APS 3–5"),
        ("teacher", "senior teacher", "APS 5–8"),
        ("teacher", "leading teacher", "EL 1"),
        ("teacher", "deputy principal", "EL 2"),
        ("teacher", "principal", "SES"),
    ];

    println!("=== APS LEVEL CHECKER ===");

    // Input: profession
    println!("Enter profession (admin / academic / lawyer / teacher):");
    let profession = read().to_lowercase();

    // Input: job title
    println!("Enter job title:");
    let title = read().to_lowercase();

    // Search through vector
    let mut result = "Not found";

    for (p, t, level) in roles {
        if p == profession && t == title {
            result = level;
            break;
        }
    }

    println!("\nAPS Level: {}", result);
}

fn read() -> String {
    let mut text = String::new();
    std::io::stdin().read_line(&mut text).unwrap();
    text.trim().to_string()
}

