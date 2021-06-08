mod header;

use std::io::{BufWriter, Write};
use std::fs::{OpenOptions, File};
use std::convert::TryFrom;

use byteorder::{LittleEndian, WriteBytesExt};
use dasp_signal::{self as signal, Signal};

use header::riff_header::RiffHeader;
use header::fmt_header::FmtHeader;
use header::data_header::DataHeader;

fn main() {
    println!("Creating WAV file...");

    let mut riff_header = RiffHeader::default();
    let fmt_header = FmtHeader::default();
    let mut data_header = DataHeader::default();

    let file = File::create("./test.wav");

    let f = OpenOptions::new()
        .append(true)
        .open("./test.wav")
        .expect("Unable to open file");
    let mut f = BufWriter::new(f);

    let duration = 20;
    let freq = 50.0;

    // Actual data
    let mut data = Vec::new();
    let take_samples = (fmt_header.sample_rate * duration) as usize;

    // Create sinewave generator and sample
    let signal = signal::rate(fmt_header.sample_rate as f64).const_hz(freq).sine();
    for n in signal.scale_amp(32767.0).take(take_samples) {
        data.write_i16::<LittleEndian>(n as i16).unwrap();
    }
    let data_len = data.len();
    let data_size = u32::try_from(data_len).unwrap();

    // RIFF Header
    f.write_all(&riff_header.chunk_id).expect("Unable to write data");

    let mut wtr = Vec::new();
    riff_header.chunk_size = data_size - 8;
    wtr.write_u32::<LittleEndian>(riff_header.chunk_size).unwrap();
    f.write_all(&wtr).expect("Unable to write data");

    f.write_all(&riff_header.format).expect("Unable to write data");

    // fmt Header
    f.write_all(&fmt_header.subchunk_1_id).expect("Unable to write data");

    let mut wtr = Vec::new();
    wtr.write_u32::<LittleEndian>(fmt_header.subchunk_1_size).unwrap();
    wtr.write_u16::<LittleEndian>(fmt_header.audio_format).unwrap();
    wtr.write_u16::<LittleEndian>(fmt_header.num_channels).unwrap();
    wtr.write_u32::<LittleEndian>(fmt_header.sample_rate).unwrap();
    wtr.write_u32::<LittleEndian>(fmt_header.byte_rate).unwrap();
    wtr.write_u16::<LittleEndian>(fmt_header.block_align).unwrap();
    wtr.write_u16::<LittleEndian>(fmt_header.bits_per_sample).unwrap();
    f.write_all(&wtr).expect("Unable to write data");

    // data Header
    f.write_all(&data_header.subchunk_2_id).expect("Unable to write data");

    let mut wtr = Vec::new();
    data_header.subchunk_2_size = data_size - 44;
    wtr.write_u32::<LittleEndian>(data_header.subchunk_2_size).unwrap();
    f.write_all(&wtr).expect("Unable to write data");

    f.write_all(&data).expect("Unable to write data");

    println!("WAV File created!");
}
