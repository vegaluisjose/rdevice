use serde::Deserialize;
use std::fs::File;

#[derive(Deserialize)]
pub struct Record {
    pub column: u64,
    pub row: u64,
    pub tilex: u64,
    pub tiley: u64,
    pub tile: String,
    pub sitex: u64,
    pub sitey: u64,
    pub site: String,
}

fn main() -> Result<(), csv::Error> {
    let file = File::open("examples/xczu3eg-sbva484.csv").unwrap();
    let mut reader = csv::Reader::from_reader(file);
    for record in reader.deserialize() {
        let record: Record = record?;
        println!(
            "site:{} x:{} y:{}",
            record.site,
            record.sitex,
            record.sitey,
        );
    }
    Ok(())
}
