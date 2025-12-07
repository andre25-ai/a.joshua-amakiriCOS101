struct Laptop {
    name: String,
    price: u32,
}

impl Laptop {
    fn cost_for_3(&self) -> u32 {
        self.price * 3
    }
}

fn main() {
    let hp = Laptop {
        name: String::from("HP"),
        price: 650_000,
    };

    let ibm = Laptop {
        name: String::from("IBM"),
        price: 755_000,
    };

    let toshiba = Laptop {
        name: String::from("Toshiba"),
        price: 550_000,
    };

    let dell = Laptop {
        name: String::from("Dell"),
        price: 850_000,
    };

    let total =
        hp.cost_for_3() +
        ibm.cost_for_3() +
        toshiba.cost_for_3() +
        dell.cost_for_3();

    println!("Total cost for buying 3 of each brand is: {}", total);
}
