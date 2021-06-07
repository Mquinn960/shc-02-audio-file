mod header;

use std::io::{BufWriter, Write};
use std::fs::OpenOptions;
use std::fs;

use header::riff_header::RiffHeader;
use header::fmt_header::FmtHeader;
use header::data_header::DataHeader;

fn main() {
    println!("Started...");

    let blah = "blah";
    let byte3: Vec<u8> = blah.as_bytes().to_vec();

    // let mut file = OpenOptions::new()
    //     .write(true)
    //     .append(true)
    //     .open("./test.txt")
    //     .unwrap();

    // if let Err(e) = writeln!(file, "Some new data") {
    //     eprintln!("Couldn't write to file: {}", e);
    // }

    let riff_header = RiffHeader::default();
    let fmt_header = FmtHeader::default();
    let mut data_header = DataHeader::default();

    data_header.subchunk_2_id = "BLAH".as_bytes().to_vec();

    // Create/Write File
    let data = "Test file data!\n";
    fs::write("./test.txt", data_header.subchunk_2_id).expect("Unable to write file");

    // Append File Buffered
    let data = "Some new data yo!\n";
    let f = OpenOptions::new()
        .append(true)
        .open("./test.txt")
        .expect("Unable to open file");
    let mut f = BufWriter::new(f);
    
    f.write_all(data.as_bytes()).expect("Unable to write data");

    // Append file unbuffered
    // let data = "Some data!\n";
    // let mut f = OpenOptions::new()
    //     .append(true)
    //     .create(true) // Optionally create the file if it doesn't already exist
    //     .open("/tmp/foo")
    //     .expect("Unable to open file");
    // f.write_all(data.as_bytes()).expect("Unable to write data");

    println!("Finished!");
}
