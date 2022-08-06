use std::num::{ParseFloatError, ParseIntError};

pub fn run() {
    // starting the machine
    println!("~~ Welcom to our vending machine !! ~~");
    let mut cash_box:Vec<u32> = vec![20; 8];
    let coins: Vec<f32> = vec![2.00, 1.00, 0.50, 0.20, 0.10, 0.05, 0.02, 0.01];

    // product selection
    let cost = select();
    // paying for the product
    pay(cost, coins, cash_box);
}

pub fn pay(cost: f32, coins: Vec<f32>, cash_box: Vec<u32>) {
    // asking to pay for the product
    println!(
        "This product will cost you: {} €. Or you can press q to quit",
        cost
    );

    // user input
    println!("enter the count number that you have for each coin");
    let mut amount: f32 = 0.0;
    let mut update_cash:Vec<u32> = vec![0; 8];
    for i in 0..8 {
        println!("how much of this coin do you have {}€", coins[i]);
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
                break;
            }
        }

        // check if user input is valid ?
        if (count.trim().parse::<u32>().unwrap() + cash_box[i]) <= 50 {
            update_cash[i] = update_cash[i] + count.trim().parse::<u32>().unwrap();
            amount = amount + coins[i] * count.trim().parse::<f32>().unwrap();
        }else {
            // true false
        }
    }

    // the amount should be a float number in this format 000.00.
    if amount <= 999.99 && amount >= cost {
        let change = amount - cost;
        // println!("thanks for paying, you can take your product.");
        if change > 0. {
            println!("and here's your change : {:.1$}", change, 2);
            let list = changecoins(change, coins, cash_box);
            println!("{:?}", list);
        }
        // break;
    } else {
        println!("the amount should be in this format 000.00.");
    }
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
            cost = products[(nbr as usize)-1].1;
            break;
        } else {
            println!("the amount should be an integer number for the product number")
        }
    }
    return cost;
}

pub fn changecoins(number: f32, coins: Vec<f32>, cash_box: Vec<u32>) -> Vec<(f32, u32)> {
    let mut list = vec![];
    let mut rest: f32 = number;
    for i in 0..8 {
        let div: f32 = format!("{:.2}", rest / coins[i]).trim().parse().unwrap();
        let mut int_part = div as u32;
        if cash_box[i] == 0 || int_part == 0{
            continue;
        }
        if int_part > cash_box[i]{
            int_part = cash_box[i];
        }
        
        list.push((coins[i], int_part));
        rest = rest - coins[i] * (int_part as f32);
       
        if rest == 0.0 {
            break;
        } 
    }

    // if rest != 0.0 {list=[].to_vec();}
    
    return  list;
}
