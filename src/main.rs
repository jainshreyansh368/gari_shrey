pub mod data;
pub mod client;

use std::io;
use data::FilterData;
use solana_transaction_status::option_serializer::OptionSerializer;

#[tokio::main]
async fn main() -> () {

    println!("start date");
    let mut start_date = String::new();
    io::stdin()
        .read_line(&mut start_date)
        .expect("Failed to read line");

    println!("end date");
    let mut end_date = String::new();
    io::stdin()
        .read_line(&mut end_date)
        .expect("Failed to read line");

    println!("start date {:?}", (start_date.trim()).to_string());
    println!("end date{:?}", (end_date.trim()).to_string());

    let (startD, endD) = FilterData::parse_data_to_date((start_date.trim()).to_string(), (end_date.trim()).to_string());


    let client = client::get_client();

    let signatures = client::get_signatures(&client).await;

    println!("Sigs");

    let transaction = client::fetch_txn(&client, &signatures).await;
    println!("Txns");

    let result_data = client::sort_data(transaction, startD, endD);

    dbg!(result_data.len());

    for result_vec_data in result_data {

        let meta_data = match &result_vec_data.transaction.meta {
            Some(val) => val,
            _ => todo!(),
        };

        let pre_token_bal = match meta_data.pre_token_balances.clone() {
            OptionSerializer::Some(val) => val,
            _ => todo!(),
        };        
        
        let post_token_bal = match meta_data.post_token_balances.clone() {
            OptionSerializer::Some(val) => val,
            _ => todo!(),
        };

        let ui_amount = post_token_bal[0].clone().ui_token_amount.amount; 

        if pre_token_bal.is_empty() && post_token_bal.len() == 1 {
            let user = match post_token_bal[0].owner.clone() {
                OptionSerializer::Some(val) => val,
                _ => todo!(),
            };

            println!("new user data {:?}", user );
            println!("user balance {:?}", ui_amount);
        }

    }

    ()
}
