pub mod lib {
    use std::error::Error;
    use serde::Deserialize;

    //Describes the valid structure of a row of data in the CSV
    #[derive(Deserialize)]
    pub struct Row<'a> {
        pub transaction_type: &'a str,
        pub client_id: u16,
        pub transaction_id: u32,
        pub amount: &'a str
    }
 
    //Holds a valid deposit transaction. Only deposits can be disputed so all other 
    //transactions are processed but not stored
    #[derive(Debug)]
    #[derive(Deserialize)]
    pub struct ApprovedTransaction {
        pub transaction_id: u32,
        pub amount: f32,
        pub in_dispute: bool
    }

    //Describes a client account with valid transactions.
    #[derive(Debug)]
    #[derive(Deserialize)]
    pub struct Client {
        pub client_id: u16,
        pub current_transactions: Vec<ApprovedTransaction>, 
        pub available: f32,
        pub held: f32,
        pub locked: bool
    }

    pub fn read_csv(filename: String, mut clients: &mut Vec<Client>) -> Result<(), Box<dyn Error>> {
        let mut rdr = csv::Reader::from_path(filename)?;
        for result in rdr.records() {
            let record = result?;
            let row: Row = record.deserialize(None)?;
            process_record(row, &mut clients);  
        }   
        Ok(())
    }
    
    fn process_record(row: Row, clients: &mut Vec<Client>) {
        //looks for a current client matching the client id in the row. 
        //If found, then calls appropriate function the handle the transaction
        //If no current client found, then creates a new client and processes the transaction
        //New clients can only be created with an initial deposit transaction
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
        //Adds deposit to client account if the account is not locked due to a chargeback
        //Deposit transactions and ammounts are stored for reference in case of a future dispute
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
        //Withdraws amount from client account if the account is unlocked and has enough available funds
        if !client.locked {
            let amount = row.amount.parse().unwrap();
            if client.available >= amount {
                client.available -= amount;
            }
        }
    }
    
    fn process_dispute(row: Row, client: &mut Client) {
        //Marks a transaction as in dispute and moves the funds from available to held pending resolution
        //If transaction is not found then dispute is discarded
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
        //If transaction is in dispute, marks ar not in dispute and moves ammount from held to available
        //If transaction is not found then resolve is discarded
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
        //If transaction is in dispute then amount is removed from held funds and account is locked 
        //Locked accounts can not have any future transactions processed
        if !client.locked {
            for trans in client.current_transactions.iter_mut() {
                if trans.transaction_id == row.transaction_id && trans.in_dispute {
                    client.locked = true;
                    client.held -= trans.amount;
                }
            }
        }
    }

}