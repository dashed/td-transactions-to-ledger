// crates

extern crate chrono;
extern crate clap;
extern crate csv;

// 3rd-party imports

use chrono::NaiveDate;
use clap::{App, Arg};

// types

enum NumOfTransactions {
    All,
    Some(u64),
}

struct Transaction {
    date: NaiveDate,
    description: String,
    amount: String,
}

enum Filter {
    FromDate(NaiveDate),
    None,
}

// app

fn main() {
    // cli args

    let matches = App::new("td-transactions-to-ledger")
        .version("0.2.0")
        .author("Alberto Leal <mailforalberto@gmail.com>")
        .about("Convert TD Bank transactions to ledger compatible transactions")
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input CSV file to use")
                .required(true),
        )
        .arg(
            Arg::with_name("start_date")
                .short("s")
                .long("start_date")
                .help("Show only transactions from start_date")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("account_name")
                .short("a")
                .long("account")
                .help("Sets account for each transaction (credit)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("debit_name")
                .short("d")
                .long("debit")
                .help("Sets debit account for each transaction (debit)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("first_n")
                .short("n")
                .long("num")
                .help("Only show last N transactions.")
                .takes_value(true),
        )
        .get_matches();

    let path_to_file = matches.value_of("INPUT").unwrap();

    let account_name = matches
        .value_of("account_name")
        .unwrap_or("account name here");

    let debit_name = matches.value_of("debit_name").unwrap_or("expenses");

    let maybe_start_date = matches
        .value_of("start_date")
        .map(|start_date| {
            Filter::FromDate(NaiveDate::parse_from_str(&start_date, "%b %d, %Y").unwrap())
        })
        .unwrap_or(Filter::None);

    let num_of_transactions = match matches.value_of("first_n") {
        None => NumOfTransactions::All,
        Some(raw_input) => match raw_input.parse::<u64>() {
            Err(_) => NumOfTransactions::All,
            Ok(n) => {
                if n > 0 {
                    NumOfTransactions::Some(n)
                } else {
                    NumOfTransactions::All
                }
            }
        },
    };

    // csv reader

    let mut reader = match csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(path_to_file)
    {
        Ok(reader) => reader,
        Err(err_reason) => {
            println!("{}", err_reason);
            std::process::exit(1);
        }
    };

    let mut transactions = vec![];

    for result in reader.records() {
        let record = result.unwrap();

        // TD Bank exports transactions as follows
        // CSV format: date, transaction description, positive, negative

        let date = record.get(0).unwrap().trim().to_string();
        let description = record.get(1).unwrap().trim().to_string();
        let debit = record.get(2).unwrap().trim().to_string();
        let credit = record.get(3).unwrap().trim().to_string();

        // formatting

        let date = NaiveDate::parse_from_str(&date, "%m/%d/%Y").unwrap();

        match maybe_start_date {
            Filter::None => {}
            Filter::FromDate(start_date) => {
                if start_date > date {
                    continue;
                }
            }
        }

        let amount = if debit.len() > 0 {
            format!("-{}", debit)
        } else {
            credit
        };

        transactions.push(Transaction {
            date,
            description: description.to_string(),
            amount: amount.to_string(),
        });
    }

    let mut transactions = transactions;

    transactions.sort_by(|a, b| a.date.partial_cmp(&b.date).unwrap());

    let transactions = transactions;

    let range = match num_of_transactions {
        NumOfTransactions::All => 0..,
        NumOfTransactions::Some(n) => (transactions.len() - (n as usize))..,
    };

    for transaction in transactions.get(range).unwrap() {
        // ledger-cli journal format: https://www.ledger-cli.org/3.0/doc/ledger3.html#Journal-Format

        println!(
            r#"
{} * {}
    {:76}{} CAD
    {}
"#,
            transaction.date.format("%Y-%m-%d"),
            transaction.description,
            account_name,
            transaction.amount,
            debit_name
        );
    }
}
