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
struct ApprovedTransaction {
    transaction_id: u32,
    amount: f32,
    in_dispute: bool
}

#[derive(Debug)]
#[derive(Deserialize)]
struct Client {
    client_id: u16,
    current_transactions: Vec<ApprovedTransaction>, 
    available: f32,
    held: f32,
    locked: bool
}

fn read_csv(filename: String, mut clients: &mut Vec<Client>) -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(filename)?;
    for result in rdr.records() {
        let record = result?;
        let row: Row = record.deserialize(None)?;
        process_record(row, &mut clients);  
    }   
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
                    _=>(),
                };
                break;
            }
        }
        if !client_found {
            if transaction_type == "deposit"{
                let approved_trans = ApprovedTransaction{
                    transaction_id: transaction_id,
                    amount: amount,
                    in_dispute: false
                };
                let mut new_client = Client {
                    client_id: client_id,
                    available: amount,
                    held: 0.0,
                    locked: false,
                    current_transactions: Vec::new()
                };
                new_client.current_transactions.push(approved_trans);
                clients.push(new_client);
            }            
        }
}

fn process_deposit(row: Row, client: &mut Client) {
    if !client.locked {
        let approved_trans = ApprovedTransaction{
            transaction_id: row.transaction_id,
            amount: row.amount.parse().unwrap(),
            in_dispute: false
        };
        client.available += approved_trans.amount;
        client.current_transactions.push(approved_trans);
    }
}

fn process_withdrawal(row: Row, client: &mut Client) {
    if !client.locked {
        let amount = row.amount.parse().unwrap();
        if client.available >= amount {
            client.available -= amount;
        }
    }
}

fn process_dispute(row: Row, client: &mut Client) {
    if !client.locked {
        for trans in client.current_transactions.iter_mut() {
            if trans.transaction_id == row.transaction_id {
                trans.in_dispute = true;
                client.available -= trans.amount;
                client.held += trans.amount;   
            }
        }
    }
}

fn process_resolve(row: Row, client: &mut Client) {
    if !client.locked {
        for trans in client.current_transactions.iter_mut() {
            if trans.transaction_id == row.transaction_id && trans.in_dispute {
                trans.in_dispute = false;
                client.available += trans.amount;
                client.held -= trans.amount;
            }
        }
    }
}

fn process_chargeback(row: Row, client: &mut Client) {
    if !client.locked {
        for trans in client.current_transactions.iter_mut() {
            if trans.transaction_id == row.transaction_id && trans.in_dispute {
                client.locked = true;
                client.held -= trans.amount;
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut clients : Vec<Client> = Vec::new();

    if let Err(err) = read_csv(filename.to_string(), &mut clients) {
        println!("Error running readcsv: {}", err);
        process::exit(1);
    }

    println!("client, available, held, total, locked");
    for client in clients {
        println!("{},{:.4},{:.4},{:.4},{}", client.client_id.to_string(), client.available, client.held, client.held + client.available, client.locked );
    }  
}