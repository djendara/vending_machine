#![allow(dead_code)]
#![allow(unused_variables)]

use std::num::{ParseFloatError, ParseIntError};

pub fn run() {
    // starting the machine
    println!("~~ Welcom to our vending machine !! ~~");

    // product selection
    let cost = select();
    // asking to pay for a random product

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
        let number: Result<f32, ParseFloatError> = line.trim().parse::<f32>();
        let amount: f32 = match number {
            Ok(nb) => nb,
            Err(_) => {
                println!("the amount should be a float number in this format 000.00.");
                continue;
            }
        };

        // the amount should be a float number in this format 000.00.
        if amount <= 999.99 && (amount * 100.0).fract() == 0.0 && amount >= cost {
            let change = amount - cost;
            println!("thanks for paying, you can take your product.");
            if change > 0. {
                println!("and here's your change : {:.1$}", change, 2);
                let list = coins_change(change);
                for c in list {
                    print!("{}x{}€ ", c.1, c.0);
                }
            }
            break;
        } else {
            println!("the amount should be in this format 000.00.");
        }
    }
}

pub fn select() -> f32 {
    // presenting the menu
    let products = vec![
        ("product 0", 5.99),
        ("product 1", 6.69),
        ("product 2", 6.69),
        ("product 3", 6.69),
        ("product 4", 5.99),
        ("product 5", 6.69),
        ("product 6", 6.69),
        ("product 7", 6.69),
        ("product 8", 6.69),
        ("product 9", 6.69),
    ];
    for prod in &products {
        println!("{} : {}€", prod.0, prod.1);
    }

    // choosing a product
    println!("choose a product number");
    let cost: f32;
    loop {
        let mut input = String::new();
        // read input line string and store it into line
        std::io::stdin().read_line(&mut input).unwrap();
        let prod_nb: Result<u32, ParseIntError> = input.trim().parse::<u32>();
        let nbr: u32 = match prod_nb {
            Ok(nb) => nb,
            Err(_) => 200,
        };
        if nbr < 10 {
            cost = products[(nbr as usize)].1;
            break;
        } else {
            println!("the number should be an integer from 0 to 9");
        }
    }
    return cost;
}

pub fn coins_change(number: f32) -> Vec<(f32, i32)> {
    let coins: Vec<f32> = vec![2.00, 1.00, 0.50, 0.20, 0.10, 0.05, 0.02, 0.01];
    let mut list = vec![];
    let mut rest: f32 = number;
    for x in coins {
        let div: f32 = format!("{:.2}", rest / x).trim().parse().unwrap();
        let int_part = div as i32;
        if div.fract() == 0.0 {
            list.push((x, int_part));
            break;
        } else if int_part != 0 {
            list.push((x, int_part));
            rest = rest - x * (int_part as f32);
        }
    }
    return list;
}
