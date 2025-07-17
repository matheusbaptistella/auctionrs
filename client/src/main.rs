struct User {
    id: u8,
    name: String,
    products: Vec<Product>,
}

struct Product {
    id: u8,
    name: String,
}

fn main() {
    let user = User {
        id: 0,
        name: String::from("John Doe"),
        products: Vec::new(),
    };

    println!("Welcome back {}!", user.name);

    loop {
        println!(
            "What would you like to do today?\n[1] Add a product\n[2] List available auctions\n[3] Exit\n"
        );

        let mut input = String::new();

        std::io::stdin().read_line(&mut input).unwrap();

        let cmd = input.trim().parse();

        match cmd {
            Ok(opt) => match opt {
                1 => println!("TODO\n"),
                2 => println!("TODO\n"),
                3 => break,
                _ => println!("Please enter a valid option!\n"),
            },
            Err(_) => println!("Please enter a valid option!\n")
        }
    }

    println!("Exiting...");
}
