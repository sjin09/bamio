use rust_htslib::bam::{Read, Reader};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::str;

pub fn bam2fasta(bam_path: &str, seq_path: &str) {
    let mut alignment = Reader::from_path(bam_path).unwrap();
    let seq_file: File = File::create(seq_path).expect("faile to open FASTQ file");
    let mut seq_writer: BufWriter<File> = BufWriter::new(seq_file);
    for record in alignment.records() {
        let read = record.unwrap();
        let qname: &str = std::str::from_utf8(read.qname()).unwrap();
        let seq = read.seq().as_bytes();
        let seq_string = std::str::from_utf8(&seq).unwrap();
        let seq = format!(">{}\n{}", qname, seq_string);
        writeln!(seq_writer, "{}", seq);
    }
}

pub fn bam2fastq(bam_path: &str, seq_path: &str) {
    let mut alignment = Reader::from_path(bam_path).unwrap();
    let seq_path: File = File::create(seq_path).expect("faile to open seq file");
    let mut seq_writer: BufWriter<File> = BufWriter::new(seq_path);
    for record in alignment.records() {
        let read = record.unwrap();
        let qname: &str = std::str::from_utf8(read.qname()).unwrap();
        let seq = read.seq().as_bytes();
        let qual = read.qual();
        let seq_string = std::str::from_utf8(&seq).unwrap();
        let qual_string = qual.iter().map(|q| (q + 33) as char).collect::<String>();
        let seq = format!("@{}\n{}\n+\n{}", qname, seq_string, qual_string);
        writeln!(seq_writer, "{}", seq);
    }
}



