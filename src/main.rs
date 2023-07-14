use clap::{App, Arg, SubCommand};
use rust_htslib::bam::{Read, Reader};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::str;

fn bam2fasta(bam_path: &str, seq_path: &str) {
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

fn bam2fastq(bam_path: &str, seq_path: &str) {
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

fn main() {
    // Define the top-level application and global options
    let app = App::new("bamio")
        .version("0.0.1")
        .author("Sangjin Lee")
        .about("accept BAM file as input and return a desired file as output");

        // Define the subcommands
    let subcommands = vec![
        SubCommand::with_name("bam2fastq")
            .about("extract FASTQ files from BAM files")
            .arg(
                Arg::with_name("input")
                    .short("i")
                    .long("input")
                    .value_name("bam_file")
                    .help("BAM file to read")
                    .required(true),
            )
            .arg(
                Arg::with_name("output")
                    .short("o")
                    .long("output")
                    .value_name("fastq_file")
                    .help("FASTQ File to return")
                    .required(true),
            ),
        SubCommand::with_name("bam2fasta")
            .about("extract FASTA files from BAM files")
            .arg(
                Arg::with_name("input")
                    .short("i")
                    .long("input")
                    .value_name("bam_file")
                    .help("BAM file to read")
                    .required(true),
            )
            .arg(
                Arg::with_name("output")
                    .short("o")
                    .long("output")
                    .value_name("fasta_file")
                    .help("FASTA File to return")
                    .required(true),
            ),
    ];

    // Add the subcommands to the top-level application
    let mut app = app.subcommands(subcommands);

    // Parse the command-line arguments
    let matches = app.clone().get_matches();

    // Handle the verbose option
    if matches.is_present("verbose") {
        println!("Verbose mode enabled");
    }

    // Match the subcommand
    match matches.subcommand() {
        ("bam2fasta", Some(bam2fasta_matches)) => {
            let bam_path = bam2fasta_matches.value_of("input").unwrap();
            let seq_path= bam2fasta_matches.value_of("output").unwrap();
            println!("running bam2fasta -i {} and -o {}", bam_path, seq_path);
            bam2fasta(bam_path, seq_path)
        }
        ("bam2fastq", Some(bam2fastq_matches)) => {
            let bam_path = bam2fastq_matches.value_of("input").unwrap();
            let seq_path = bam2fastq_matches.value_of("output").unwrap();
            println!("running bam2fastq -i {} and -o {}", bam_path, seq_path);
            bam2fastq(bam_path, seq_path)
        }
        _ => {
            app.print_help().unwrap();
            println!();
        }
    }
}
