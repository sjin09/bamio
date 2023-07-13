use clap::{App, Arg};
use std::str;
use std::fs::File;
use std::io::{BufWriter, Write};
use rust_htslib::bam::{Read, Reader};

fn bam2fastq(bam_path: &str, fastq_path: &str){

    let mut alignment = Reader::from_path(bam_path).unwrap();
    let fastq_file: File = File::create(fastq_path).expect("faile to open FASTQ file");
    let mut fastq_writer :BufWriter<File> = BufWriter::new(fastq_file);
    for record in alignment.records() {
        let read = record.unwrap();
        let qname: &str = std::str::from_utf8(read.qname()).unwrap();
        let seq = read.seq().as_bytes();
        let qual = read.qual();
        let seq_string = std::str::from_utf8(&seq).unwrap();
        let qual_string = qual.iter().map(|q| (q + 33) as char).collect::<String>();
        let fastq = format!("@{}\n{}\n+\n{}", qname, seq_string, qual_string);
        writeln!(fastq_writer, "{}", fastq);
    }
}

fn main() {
    let matches = App::new("BAM to FASTQ Converter")
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("input")
                .value_name("bam_file")
                .help("input BAM file to read")
                .required(true),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("fastq_file")
                .help("FASTQ file to return")
                .required(true),
        )
        .get_matches();

    let bam_path = matches.value_of("input").unwrap();
    let fastq_path = matches.value_of("output").unwrap();
    bam2fastq(bam_path, fastq_path)
}
