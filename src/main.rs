extern crate csv;

use std::env;
use std::error::Error;
use std::fs::File;
use std::io;

fn split_csv(input_file: &str, num_pieces: usize) -> Result<(), Box<dyn Error>> {
    let file = File::open(input_file)?;
    let mut rdr = csv::Reader::from_reader(file);
    let headers = rdr.headers()?.clone();

    let mut writers: Vec<csv::Writer<File>> = Vec::new();
    for i in 0..num_pieces {
        let filename = format!("piece_{}.csv", i + 1);
        let file = File::create(&filename)?;
        let mut writer = csv::Writer::from_writer(file);
        writer.write_record(headers.iter())?;
        writers.push(writer);
    }

    let mut counter = 0;
    for result in rdr.records() {
        let record = result?;
        let writer_index = counter % num_pieces;
        writers[writer_index].write_record(record.iter())?;
        counter += 1;
    }

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
