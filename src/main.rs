// crates

extern crate chrono;
extern crate clap;
extern crate csv;

// 3rd-party imports

use clap::{App, Arg};
use chrono::NaiveDate;

fn main() {
    // cli args

    let matches = App::new("td-transactions-to-ledger")
        .version("0.1.0")
        .author("Alberto Leal <mailforalberto@gmail.com>")
        .about("Convert TD Bank transactions to ledger compatible transactions")
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input CSV file to use")
                .required(true),
        )
        .arg(
            Arg::with_name("account_name")
                .short("a")
                .long("account")
                .help("Sets account for each transaction")
                .takes_value(true),
        )
        .get_matches();

    let path_to_file = matches.value_of("INPUT").unwrap();
    let account_name = matches.value_of("account_name").unwrap_or("account name here");

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

        let amount = if debit.len() > 0 {
            format!("-{}", debit)
        } else {
            credit
        };

        // ledger-cli journal format: https://www.ledger-cli.org/3.0/doc/ledger3.html#Journal-Format

        println!(
            r#"
{} * {}
    {:76}{} CAD
    ???
"#,
            date.format("%Y-%m-%d"),
            description,
            account_name,
            amount
        );

    }
}
