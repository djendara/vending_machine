use std::num::ParseIntError;

static mut CASH_BOX: [u32; 8] = [20, 20, 20, 20, 20, 20, 20, 20];
static COINS: [f32; 8] = [2.00, 1.00, 0.50, 0.20, 0.10, 0.05, 0.02, 0.01];

pub fn run() {
    // starting the machine
    println!("~~ Welcom to our vending machine !! ~~");
    // let CASH_BOX: Vec<u32> = vec![20; 8];

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
        if payed{
            break;
        }
        
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
    let mut update_cash: Vec<u32> = vec![0; 8];
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
            update_cash[i] = update_cash[i] + count.trim().parse::<u32>().unwrap();
            amount = amount + COINS[i] * count.trim().parse::<f32>().unwrap();
        } else {
            // true false
            println!("no space left");
            return false;
        }
    }

    // the amount should be a float number in this format 000.00.
    if amount <= 999.99 && amount >= cost {
        let change = amount - cost;
        // println!("thanks for paying, you can take your product.");
        if change > 0. {
            println!("and here's your change : {:.1$}", change, 2);
            let it_worked: bool = true;
            let list = changecoins(change, it_worked);
            if it_worked {
                println!("{:?}", list);    
            }else {
                println!("unabale to give change");
                return false;
            }
            
        }
        // break;
    } else {
        println!("the amount should be in this format 000.00.");
        return false;
    }
    return true;
}

pub fn select() -> f32 {
    // presenting the menu
    let products = vec![
        ("product 1", 5.99),
        ("product 2", 6.69),
        ("product 2", 6.69),
        ("product 2", 6.69),
    ];
    for prod in &products {
        println!("{:?}", prod);
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
            println!("the amount should be an integer number for the product number")
        }
    }
    return cost;
}

pub fn changecoins(number: f32, mut it_worked: bool) -> Vec<(f32, u32)> {
    let mut list = vec![];
    let mut rest: f32 = number;
    
    for i in 0..8 {
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

        if rest == 0.0 {
            it_worked = true;
            break;
        }
    }
    // println!("{:?}",list);
    if rest != 0.0 {it_worked=false;};

    return list;
}

pub fn service() {
    println!("choose a service you want: 0_ Chech cash box. 1_ add to the cash box. 2_ remove from the cash box.");
    let mut input = String::new();
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
