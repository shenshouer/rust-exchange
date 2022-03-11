use std::env;
use std::io;

use order_protobuf::order_service_client::OrderServiceClient;
use order_protobuf::{OrderRequest, OrderResponse, OrderType};

pub mod order_protobuf {
    tonic::include_proto!("order_protobuf");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("It is missing the asset code from your command.");
        return Ok(());
    }

    let asset: &String = &args[1];

    println!("\n\n*** Starting matching engine for {} ***\n\n", asset);
    let mut client = OrderServiceClient::connect("http://[::1]:3050").await?;

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
        let mut order_type_enum: i32 = 0;

        if order_type == "SELL" {
            order_type_enum = 1;
        } else if order_type != "BUY" {
            println!("Invalid order type: {:?}", order_type);
            continue;
        }

        let request = tonic::Request::new(OrderRequest {
            asset_code: asset.into(),
            order_type: order_type_enum,
            quantity: qty,
            price,
        });

        let response = client.send_order(request).await?;

        println!("RESPONSE={:?}", response);
    }

    Ok(())
}
