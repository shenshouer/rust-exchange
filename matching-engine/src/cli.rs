use std::env;
use std::io;

mod engine;
use engine::{MatchingEngine, OrderResult, OrderType};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("It is missing the asset code from your command.");
        return;
    }

    let asset: &String = &args[1];

    println!("\n\n*** Starting matching engine for {} ***\n\n", asset);
    let mut engine = MatchingEngine::new(asset.to_string());

    loop {
        println!("Enter your order command in the format: BUY|SELL QTY PRICE\n");

        let mut command: String = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read you order command");
        let args: Vec<&str> = command.split(' ').collect();
        let args_size = args.len();

        if args_size != 3 {
            if args_size > 0 && args[0].trim().to_lowercase() == "q" {
                println!("Bye!");
                break;
            }

            println!("Wrong number of arguments in your command.");
            continue;
        }

        let order_type: String = args[0].trim().to_uppercase();
        let qty: u64 = args[1].trim().parse().expect("Invalid quantity");
        let price: f32 = args[2].trim().parse().expect("Invalid price");
        let mut order_type_enum: OrderType = OrderType::Buy;

        if order_type == "SELL" {
            order_type_enum = OrderType::Sell;
        } else if order_type != "BUY" {
            println!("Invalid order type: {:?}", order_type);
            continue;
        }

        let results: Vec<OrderResult> = engine.add_order(order_type_enum, qty, price);

        for result in results {
            println!(
                "Order {:?} => {:?} x {:?}",
                result.result_type, result.quantity, result.price
            );
        }
        println!();
    }
}
