extern crate csv;

use std::collections::{HashMap, HashSet};
use std::env;
use std::error::Error;
use std::fs::File;
use std::io;

fn split_csv(input_file: &str, num_pieces: usize) -> Result<(), Box<dyn Error>> {
    let file = File::open(input_file)?;
    let mut rdr = csv::Reader::from_reader(file);
    let headers = rdr.headers()?.clone();

    let mut organization_ids: HashSet<String> = HashSet::new();
    let mut organization_records: HashMap<String, Vec<Vec<String>>> = HashMap::new();

    // read all records and group them by organization_id
    for result in rdr.records() {
        let record = result?;
        // first column is organization_id
        let organization_id = record.get(0).unwrap_or_default().to_string();
        organization_ids.insert(organization_id.clone());

        let entry = organization_records
            .entry(organization_id)
            .or_insert_with(Vec::new);
        entry.push(record.iter().map(|s| s.to_string()).collect());
    }

    // determine number of records to assign to each output file
    let total_records = organization_records
        .values()
        .map(|records| records.len())
        .sum::<usize>();
    let records_per_piece = total_records / num_pieces;
    println!(
        "total records={}, records per piece={}",
        total_records, records_per_piece
    );

    // create files
    let mut writers: Vec<csv::Writer<File>> = Vec::new();
    for i in 0..num_pieces {
        let filename = format!("piece_{}.csv", i + 1);
        let file = File::create(&filename)?;
        let mut writer = csv::Writer::from_writer(file);
        writer.write_record(headers.iter())?;
        writers.push(writer);
    }

    let mut piece_index = 0;
    let mut written_records = 0;
    // assign records to output files
    for organization_id in organization_ids {
        let records = organization_records.get(&organization_id).unwrap();
        let mut remaining_records = records.len();

        for record in records {
            let writer = &mut writers[piece_index];
            writer.write_record(record)?;
            remaining_records -= 1;
            written_records += 1;

            if remaining_records == 0 && written_records >= records_per_piece {
                piece_index += 1;
                written_records = 0;
            } else {
                continue;
            }
        }
    }

    // flush writers
    for mut writer in writers {
        writer.flush()?;
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <input_file> <num_pieces>", args[0]);
        std::process::exit(1);
    }

    let input_file = &args[1];
    let num_pieces: usize = args[2]
        .parse()
        .expect("num_pieces must be a positive integer");

    split_csv(input_file, num_pieces).expect("Failed to split csv");
    Ok(())
}
