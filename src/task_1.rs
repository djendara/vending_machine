use rand::Rng;

pub fn run() {
    // starting the machine
    println!("~~ Welcom to our vending machine !! ~~");

    // asking to pay for a random product
    let random: f32 = rand::thread_rng().gen_range(1.00..10.00);
    let amount = format!("{:.2}", random).trim().parse().unwrap();
    println!(
        "This product will cost you: {} €. Or you can press q to quit",
        amount
    );

    // user input
    loop {
        let mut line = String::new();
        // read input line string and store it into line
        std::io::stdin().read_line(&mut line).unwrap();
        // check if user wants to continue
        if line.trim().to_lowercase() == "q" {
            // confirmation message
            break;
        }

        // convert line to integer
        let number: f32 = line
            .trim()
            .parse()
            .expect("the amount should be in this format 000.00.");
        
        
        if number <= 999.99 && (number * 100.0).fract() == 0.0 && number >= amount {
            println!("it worked {}", number);
            let change = number - amount;
            println!("the change = {:.1$}", change, 2);
        }
    }
}
