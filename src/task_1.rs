use std::num::ParseFloatError;
use rand::Rng;

pub fn run() {
    // starting the machine
    println!("~~ Welcom to our vending machine !! ~~");

    // asking to pay for a random product
    let random: f32 = rand::thread_rng().gen_range(1.00..10.00);
    let cost = format!("{:.2}", random).trim().parse().unwrap();
    println!(
        "This product will cost you: {} €. Or you can press q to quit",
        cost
    );

    // user input
    loop {
        // The user enters the cost of money he/she puts into the vending machine
        let mut line = String::new();
        // read input line string and store it into line
        std::io::stdin().read_line(&mut line).unwrap();

        // check if user wants to continue
        if line.trim().to_lowercase() == "q" {
            // confirmation message
            println!("are you sure you want to quit ? y/n (pleas type y for yes or n for no.)");
            // read input line string and store it into conf
            let mut conf = String::new();
            std::io::stdin().read_line(&mut conf).unwrap();
            if conf.trim().to_lowercase() == "y" {
                break;
            }
        }

        // check if user input is valid
        // convert inpute to integer
        let number: Result<f32, ParseFloatError> = line
            .trim()
            .parse::<f32>();
        let amount: f32 = match number {
            Ok(nb) => nb,
            Err(_) => {println!("the amount should be a float number in this format 000.00."); continue;},
        };

        // the amount should be a float number in this format 000.00.
        if amount <= 999.99 && (amount * 100.0).fract() == 0.0 && amount >= cost {
            let change = amount - cost;
            println!("thanks for paying, you can take your product.");
            if change > 0. {println!("and here's your change : {:.1$}", change, 2);}
            break;
        }else {
            println!("the amount should be in this format 000.00.");
        }
    }
}
