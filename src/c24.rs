use std::io;

#[derive(Debug)]
pub struct Record {
    pub transaction_type: String,
    pub booking_date: chrono::NaiveDate,
    pub amount: f64,
    pub recipient: String,
    pub iban: String,
    pub bic: String,
    pub reason_for_payment: String,
    pub description: String,
    pub category: String,
    pub sub_category: String,
}

pub fn parse_csv(rdr: impl io::Read) -> impl Iterator<Item = Record> {
    let rdr = csv::Reader::from_reader(rdr);

    return rdr.into_records().map(|x| from_string_record(&x.unwrap()));
}

fn from_string_record(rec: &csv::StringRecord) -> Record {
    Record {
        transaction_type: parse_str(rec.get(0)),
        booking_date: parse_date(rec.get(1)).unwrap(),
        amount: parse_float(rec.get(2)),
        recipient: parse_str(rec.get(3)),
        iban: parse_str(rec.get(4)),
        bic: parse_str(rec.get(5)),
        reason_for_payment: parse_str(rec.get(6)),
        description: parse_str(rec.get(7)),
        category: parse_str(rec.get(8)),
        sub_category: parse_str(rec.get(9)),
    }
}

fn parse_date(val: Option<&str>) -> Option<chrono::NaiveDate> {
    match val {
        Some(d) => Some(chrono::NaiveDate::parse_from_str(d, "%d.%m.%Y").unwrap()),
        None => None,
    }
}

fn parse_float(val: Option<&str>) -> f64 {
    match val.unwrap().replace(",", ".").parse::<f64>() {
        Ok(n) => n,
        _Err => panic!("Value \"{:?}\" can not be parsed to float", val),
    }
}

fn parse_str(val: Option<&str>) -> String {
    val.unwrap_or("").to_string()
}
