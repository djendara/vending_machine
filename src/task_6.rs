#![allow(dead_code, unused_imports)]

use std::io::Read;
use std::io::Write;

use std::num::ParseIntError;

static mut CASH_BOX: [u32; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
static COINS: [f32; 8] = [2.00, 1.00, 0.50, 0.20, 0.10, 0.05, 0.02, 0.01];
static mut IT_WORKED: bool = true;

pub fn run() {
    // starting the machine
    println!("~~ Welcom to our vending machine !! ~~");
    // loading cash box from file
    load_cash_box();
    // update_cash_box();

    loop {
        println!("choose your option: 0_ show prosucts. 1_ service.");
        let mut opt = String::new();
        std::io::stdin().read_line(&mut opt).unwrap();
        if opt.trim().to_lowercase() == "1" {
            // service
            service();
            continue;
        }

        // product selection
        let cost = select();

        // paying for the product
        let payed = pay(cost);
        if payed {
            break;
        }
    }
}

pub fn update_cash_box() {
    let mut file = std::fs::File::create("CASH_BOX.txt").expect("create failed");
    for cash in unsafe { CASH_BOX } {
        let val = format!("{}\n", cash);
        file.write_all(val.as_bytes()).expect("write failed");
    }
    println!("data written to file");
}

pub fn load_cash_box() {
    let mut file = std::fs::File::open("CASH_BOX.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut index = 0;
    for lin in contents.split('\n') {
        if lin == "" {
            break;
        }
        unsafe {
            CASH_BOX[index] = lin.trim().parse::<u32>().unwrap();
        }
        index = index + 1;
    }
}

pub fn pay(cost: f32) -> bool {
    // asking to pay for the product
    println!(
        "This product will cost you: {} €. Or you can press q to quit",
        cost
    );

    // user input
    println!("enter the count number that you have for each coin");
    let mut amount: f32 = 0.0;
    let mut user_cash: Vec<u32> = vec![0; 8];
    for i in 0..8 {
        println!("how much of this coin do you have {}€", COINS[i]);
        // read the count of this coin
        let mut count = String::new();
        std::io::stdin().read_line(&mut count).unwrap();

        // check if user wants to continue or not
        if count.trim().to_lowercase() == "q" {
            // confirmation message
            println!("are you sure you want to quit ? y/n (pleas type y for yes or n for no.)");
            // read input line string and store it into conf
            let mut conf = String::new();
            std::io::stdin().read_line(&mut conf).unwrap();
            if conf.trim().to_lowercase() == "y" {
                return true;
            }
        }

        // check if user input is valid ?
        if (count.trim().parse::<u32>().unwrap() + unsafe { CASH_BOX[i] }) <= 50 {
            user_cash[i] = user_cash[i] + count.trim().parse::<u32>().unwrap();
            amount = amount + COINS[i] * count.trim().parse::<f32>().unwrap();
        } else {
            println!("no space left");
            return false;
        }
    }

    // the amount should be a float number in this format 000.00.
    if amount <= 999.99 && amount >= cost {
        let change = amount - cost;
        // println!("thanks for paying, you can take your product.");
        if change > 0. {
            println!("Thanks and here's your change : {:.1$}€", change, 2);
            let list = coins_change(change);
            if unsafe { IT_WORKED } {
                for i in 0..8 {
                    unsafe {
                        CASH_BOX[i] = CASH_BOX[i] + user_cash[i];
                    }
                }
                for cash in &list {
                    unsafe {
                        let index_element = COINS.iter().position(|&x| x == cash.0).unwrap();
                        CASH_BOX[index_element] = CASH_BOX[index_element] - cash.1;
                    }
                }
                for c in list {
                    print!("{}x{}€ ", c.1, c.0);
                }
                println!("");
                update_cash_box();
            } else {
                println!("unabale to give change");
                return false;
            }
        }
    } else {
        println!("the amount should be in this format 000.00.");
        return false;
    }
    return true;
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

pub fn coins_change(number: f32) -> Vec<(f32, u32)> {
    let mut list = vec![];
    let mut rest: f32 = number;

    for i in 0..8 {
        // the coin is empty in the cash box so we move to the nex one, mybe we can do the change with the rest
        if unsafe { CASH_BOX[i] } == 0 {
            continue;
        }
        let div: f32 = format!("{:.2}", rest / COINS[i]).trim().parse().unwrap();
        let mut int_part = div as u32;
        if int_part == 0 {
            continue;
        }
        if int_part > unsafe { CASH_BOX[i] } {
            int_part = unsafe { CASH_BOX[i] };
        }

        list.push((COINS[i], int_part));
        rest = rest - COINS[i] * (int_part as f32);

        // this means we can give user the change with no problem
        if rest == 0.0 {
            break;
        }
    }

    // this means we can't give chnage with curent cash box, we need more coins
    if rest != 0.0 {
        unsafe {
            IT_WORKED = false;
        }
    };

    return list;
}

pub fn service() {
    println!("choose a service you want: 0_ Chech cash box. 1_ add to the cash box. 2_ remove from the cash box.");
    let mut input;
    loop {
        // let mut input = String::new();
        // read input line string and store it into line
        input = String::new();

        std::io::stdin().read_line(&mut input).unwrap();
        let serv_nb: Result<u32, ParseIntError> = input.trim().parse::<u32>();
        let nbr: u32 = match serv_nb {
            Ok(nb) => nb,
            Err(_) => 200,
        };
        if nbr < 3 {
            if nbr == 0 {
                println!("{:?}", unsafe { CASH_BOX });
            } else if nbr == 1 {
                input = String::new();
                println!("give a coin index that you wanna add to it {:?}", COINS);
                std::io::stdin().read_line(&mut input).unwrap();
                let index = input.trim().parse::<u32>().unwrap();

                println!("number of coins you are adding :");
                input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                if input.trim().parse::<u32>().unwrap() + unsafe { CASH_BOX[index as usize] } <= 50
                {
                    unsafe {
                        CASH_BOX[index as usize] =
                            CASH_BOX[index as usize] + input.trim().parse::<u32>().unwrap();
                    }
                    update_cash_box();
                    println!("done.");
                } else {
                    println!("not much space left...");
                }
            } else {
                println!("give a coin index to remove from {:?}", COINS);
                input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                let index = input.trim().parse::<u32>().unwrap();

                println!("how many coins you taking :");
                input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                if unsafe { CASH_BOX[index as usize] } >= input.trim().parse::<u32>().unwrap() {
                    unsafe {
                        CASH_BOX[index as usize] =
                            CASH_BOX[index as usize] - input.trim().parse::<u32>().unwrap();
                    }
                    update_cash_box();
                    println!("done.");
                } else {
                    println!("you are requesting more coins then the box have");
                }
            }
        } else {
            println!("the service number should be an integer like 0, 1 or 2.");
            continue;
        }
        break;
    }
}
