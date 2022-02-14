use std::error::Error;
use std::env;
use std::process;

use serde::Deserialize;

#[derive(Deserialize)]
struct Row<'a> {
    transaction_type: &'a str,
    client_id: u16,
    transaction_id: u32,
    amount: &'a str
}

#[derive(Debug)]
#[derive(Deserialize)]
struct TransactionList {
    transaction_id: u32,
    amount: f32
}

#[derive(Debug)]
#[derive(Deserialize)]
struct Client {
    client_id: u16,
    current_transactions: TransactionList, 
    available: f32,
    held: f32,
    locked: bool
}

fn read_csv(filename: String) -> Result<(), Box<dyn Error>> {
    // Build the CSV reader and iterate over each record.
    let mut clients : Vec<Client> = Vec::new();
    let mut rdr = csv::Reader::from_path(filename)?;
    for result in rdr.records() {
        let record = result?;
        let row: Row = record.deserialize(None)?;
        process_record(row, &mut clients) 
    }
    println!("{:?}", clients);
    Ok(())
}

fn process_record(row: Row, clients: &mut Vec<Client>) {
        let mut client_found = false;
        let client_id = row.client_id;
        let transaction_id = row.transaction_id;
        let transaction_type = row.transaction_type; 
        let mut amount = 0.0;
        if row.amount != "" {
            amount = row.amount.parse().unwrap();
        }
        for mut client in clients.iter_mut() {
            if client.client_id == client_id {
                client_found = true;
                match transaction_type {
                    "deposit"=> process_deposit(row, &mut client),
                    "withdrawal"=> process_withdrawal(row, &mut client),
                    "dispute"=>process_dispute(row, &mut client),
                    "resolve"=>process_resolve(row, &mut client),
                    "chargeback"=>process_chargeback(row, &mut client),
                    _=>process_invalid(row),
                };
                break;
            }
        }
        if !client_found {
            if transaction_type == "deposit"{
                let trans = TransactionList{
                    transaction_id: transaction_id,
                    amount: amount
                };
                let new_client = Client {
                    client_id: client_id,
                    current_transactions: trans,
                    available: amount,
                    held: 0.0,
                    locked: false
                };
                clients.push(new_client);
            }            
        }
}

fn process_deposit(row: Row, client: &mut Client) {
    
}
fn process_withdrawal(row: Row, client: &mut Client) {

}
fn process_dispute(row: Row, client: &mut Client) {
    
}
fn process_resolve(row: Row, client: &mut Client) {
    
}
fn process_chargeback(row: Row, client: &mut Client) {

}
fn process_invalid(row: Row) {

}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args[1]);
    let filename = &args[1];
    if let Err(err) = read_csv(filename.to_string()) {
        println!("error running readcsv: {}", err);
        process::exit(1);
    }
    
    
}