# Rust Coding Test

This is a simple rust program that reads transaction data from a csv file. Processes the transactions and outputs the client account details.

## Installation

```bash
git clone git@github.com:wrchipman/rustcodingtest.git
```

## Usage

```bash
cargo run -- transactions.csv > accounts.csv
```

## Test files

Test files included:

# transactions.csv - test file with multiple clients and multiple valid and invalid transactions

# transactions_single_account.csv - test file with a single client and multiple valid and invalid transactions

# transactions_single_account_errors.csv - test file with single client and multiple errors.

## Error Handling and Assumptions

Any invalid transaction data that is in the correct format will be discarded.
Any csv with invalid data formats will cause the entire file to be disregarded. An error will be printed explaining that the data is unreadable.
