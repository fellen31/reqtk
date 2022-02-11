extern crate seq_io;

use clap::{AppSettings, Parser, Subcommand};

use seq_io::fasta::{Record, Reader};

use std::io::{self, Write};
use std::str;

#[derive(Parser)]
#[clap(name = "reqtk")]
#[clap(version = "0.0.1")]
#[clap(
    about = "Heng Li Appreciation Program",
    long_about = "Does odd things noone usually needs."
)]
#[clap(global_setting(AppSettings::PropagateVersion))]
#[clap(global_setting(AppSettings::UseLongFormatForHelpSubcommand))]
#[clap(global_setting(AppSettings::ArgRequiredElseHelp))]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// uncommon transformation of FASTA
    Seq {
        #[clap(required = true, name = "in.fa")]
        input: Option<String>,
        #[clap(short)]
        /// get frequency of masked positions (from seqtk seq -M)
        masked_frequency: bool,
        #[clap(short)]
        /// actually reverse (not reverse complement)
        reverse: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Seq {
            input,
            masked_frequency,
            reverse,
        } => {
            if *masked_frequency {
                masked_positions(input);
            }
            if *reverse {
                reverse_records(input);
            }
        }
    }
}

fn masked_positions(input: &Option<String>) {
    // Read two times
    let mut reader = Reader::from_path(input.as_deref().unwrap()).expect("Error reading file");

    let mut total_bases = vec![0,10^6];
    let mut masked_bases = vec![0,10^6];
    while let Some(record) = reader.next() {
        let record = record.expect("Error reading record");
        let mut i = 0;
        for line in record.seq_lines() {
            for base in line {
                if i >= total_bases.len() {
                    total_bases.push(0);
                    masked_bases.push(0);
                }

                if (*base as char).is_lowercase() {
                    masked_bases[i] += 1;
                    total_bases[i] += 1;
                }
                total_bases[i] += 1;
                i += 1;
            }
        }
    }


    // Create a writer
    let stdout = io::stdout();
    let  handle = stdout.lock();
    let mut writer = io::BufWriter::new(handle);

    for i in 0..masked_bases.len() {
        writeln!(writer, "{} {} {} {}", i, masked_bases[i], total_bases[i], masked_bases[i] as f32 / total_bases[i] as f32).unwrap();
    }
}

fn reverse_records(input: &Option<String>) {

      let mut reader = Reader::from_path(input.as_deref().unwrap()).unwrap();

    while let Some(record) = reader.next() {
        let record = record.expect("Error reading record");
        let mut output = vec![];
        record.write(&mut output).unwrap();
        
        let s = String::from_utf8(output);
        let reversed: String = s.unwrap().chars().rev().collect();
        // Create a writer
        let stdout = io::stdout();
        let handle = stdout.lock();
        let mut writer = io::BufWriter::new(handle);
        writeln!(writer, ">{}\n{}", record.id().unwrap(), reversed).unwrap();
    }
}
