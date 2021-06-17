use serde::Deserialize;
use std::fs::File;
use std::collections::{HashSet, HashMap};

#[derive(Clone, Debug, Deserialize)]
pub struct Record {
    pub col: u64,
    pub row: u64,
    pub tilex: u64,
    pub tiley: u64,
    pub tile: String,
    pub sitex: u64,
    pub sitey: u64,
    pub site: String,
}

// #[derive(Clone, Debug)]
// pub struct Cell {
//     pub prim: String,
//     pub col: u64,
//     pub row: u64,
//     pub x: u64,
//     pub y: u64,
// }

fn main() -> Result<(), csv::Error> {
    let file = File::open("examples/xczu3eg-sbva484.csv").unwrap();
    let mut reader = csv::Reader::from_reader(file);
    let mut max_slicel = 0;
    let mut max_slicem = 0;
    let mut max_dsp = 0;
    let mut max_ram = 0;
    let mut col: HashSet<u64> = HashSet::new();
    let mut items: Vec<Record> = Vec::new();
    for record in reader.deserialize() {
        let record: Record = record?;
        match record.site.as_ref() {
            "SLICEL" => {
                if record.sitey > max_slicel {
                    max_slicel = record.sitey;
                }
                col.insert(record.col);
                items.push(record.clone());
            }
            "SLICEM" => {
                if record.sitey > max_slicem {
                    max_slicem = record.sitey;
                }
                col.insert(record.col);
                items.push(record.clone());
            }
            "DSP48E2" => {
                if record.sitey > max_dsp {
                    max_dsp = record.sitey;
                }
                col.insert(record.col);
                items.push(record.clone());
            }
            "RAMB181" => {
                if record.sitey > max_ram {
                    max_ram = record.sitey;
                }
                col.insert(record.col);
                items.push(record.clone());
            }
            _ => (),
        };
    }
    let mut cv: Vec<u64> = col.into_iter().collect();
    cv.sort_unstable();
    let mut map: HashMap<u64, u64> = HashMap::new();
    for (i, val) in cv.iter().enumerate() {
        map.insert(*val, i as u64);
    }
    items.sort_by_key(|r| r.col);
    for record in items {
        if let Some(x) = map.get(&record.col) {
            let max = match record.site.as_ref() {
                "SLICEL" => max_slicel,
                "SLICEM" => max_slicem,
                "DSP48E2" => max_dsp,
                _ => max_ram,
            };
            let y = max - record.sitey;
            println!("{},{},{},{},{}", x, y, record.site, record.sitex, record.sitey);
        }
    }
    Ok(())
}
