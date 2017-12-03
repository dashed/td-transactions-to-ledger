// crates

extern crate clap;
extern crate csv;
extern crate chrono;

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
        .get_matches();

    let path_to_file = matches.value_of("INPUT").unwrap();

    // csv reader

    let mut reader = match csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(path_to_file) {
            Ok(reader) => reader,
            Err(err_reason) => {
                println!("{}", err_reason);
                std::process::exit(1);
            }
        };

    for result in reader.records() {

        let record = result.unwrap();

        // CSV format: date, transaction description, positive, negative

        let date = record.get(0).unwrap().trim().to_string();
        let description = record.get(1).unwrap().trim().to_string();
        let debit = record.get(2).unwrap().trim().to_string();
        let credit = record.get(3).unwrap().trim().to_string();

        let date = NaiveDate::parse_from_str(&date, "%m/%d/%Y").unwrap();

        println!("{} {} {} {}", date.format("%Y-%m-%d"), description, debit, credit);


    }
}
