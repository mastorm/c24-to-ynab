use std::fs;

mod c24;
mod mapper;
mod ynab;

const CSV_EXPORT: &str = "./transactions.csv";

fn main() {
    let f = fs::File::open(CSV_EXPORT).unwrap();
    let transactions = c24::parse_csv(f).map(|t| ynab::Record {
        date: t.booking_date.format("%Y-%m-%d").to_string(),
        payee: t.recipient,
        memo: t.reason_for_payment,
        amount: t.amount.to_string(),
    });

    let out = fs::File::create("out.csv").unwrap();
    ynab::write_csv(out, transactions).unwrap();
}
