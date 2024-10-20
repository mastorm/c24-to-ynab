use std::fs;

mod c24;

const CSV_EXPORT: &str = "./transactions.csv";
fn main() {
    let f = fs::File::open(CSV_EXPORT).unwrap();
    let transactions = c24::parse_csv(f);
    for t in transactions {
        println!("{t:?}")
    }
}
