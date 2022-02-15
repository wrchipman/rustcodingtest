//Application: Simple toy transaction processing system
//Developer: William Chipman
//Last Updated: 14 Feb 2022

use std::env;
use std::process;

mod lib;
use lib::lib::Client;

fn main() {
    //Reads name of CSV file passed on the command line 
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut clients : Vec<Client> = Vec::new();

    //Reads and processes all transaction in the CSV. If any transaction are in an improper format
    //prints an error to stdout and exits
    if let Err(err) = lib::lib::read_csv(filename.to_string(), &mut clients) {
        println!("Error running readcsv: {}", err);
        process::exit(1);
    }
    //Loops through the final client account list printing account details to stdout
    println!("client, available, held, total, locked");
    for client in clients {
        println!("{},{:.4},{:.4},{:.4},{}", client.client_id.to_string(), client.available, client.held, client.held + client.available, client.locked );
    }  
}