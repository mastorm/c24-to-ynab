use std::{error::Error, io, string};

#[derive(Debug)]
pub struct Record {
    pub date: String,
    pub payee: String,
    pub memo: String,
    pub amount: String,
}

pub fn write_csv(
    records: impl Iterator<Item = Record>,
) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut out_buffer = vec![];
    let mut writer = csv::Writer::from_writer(&mut out_buffer);

    write_csv_header(&mut writer)?;

    for r in records {
        writer.write_field(r.date)?;
        writer.write_field(r.payee)?;
        writer.write_field(r.memo)?;
        writer.write_field(r.amount)?;
        writer.write_record(None::<&[u8]>)?;
    }

    writer.flush()?;

    drop(writer);
    Ok(out_buffer)
}

fn write_csv_header(writer: &mut csv::Writer<impl std::io::Write>) -> Result<(), Box<dyn Error>> {
    writer.write_record(&["Date", "Payee", "Memo", "Amount"])?;
    Ok(())
}
