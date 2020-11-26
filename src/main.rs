use colored::*;
use std::error::Error;
use std::path::PathBuf;

#[macro_use]
mod tagger;
mod cli;
mod dict;
mod errors;
mod format;
mod traits;
mod types;
use cli::cli;
use dict::DictionaryBuilder;
use format::{BIOES, IOB};
use tagger::TaggerBuilder;
use traits::PrettyDisplay;

use std::fs::File;
use std::io::Read;

#[macro_use]
extern crate log;
use aho_corasick::MatchKind;
use env_logger::Builder;
use glob::glob;
use log::LevelFilter;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = cli();

    if !matches.is_present("silent") {
        let mut log_builder = Builder::new();
        log_builder.filter(None, LevelFilter::Info).init();
    }

    let dictionary = match matches.values_of("dictionary") {
        Some(dicts) => {
            let dictionaries = dicts
                .collect::<Vec<&str>>()
                .iter()
                .flat_map(|path| glob(path).expect("Failed to read glob pattern"))
                .map(|p| p.expect("Failed to read glob pattern"))
                .collect::<Vec<PathBuf>>();

            DictionaryBuilder::from_files(&dictionaries)?.build()
        }

        None => panic!("A dictionary is required"),
    };

    let mut tagger_builder = TaggerBuilder::default().dictionary(&dictionary);

    if let Some(matches) = matches.subcommand_matches("tagger") {
        if let Some(case) = matches.value_of("case_sensitive") {
            tagger_builder = tagger_builder.case_sensitive(case.parse()?);
        }

        if let Some(word_matching) = matches.value_of("word_matching") {
            tagger_builder = tagger_builder.word_matching(word_matching.parse()?);
        }

        if let Some(match_kind) = matches.value_of("match_kind") {
            match match_kind {
                "standard" => {
                    tagger_builder = tagger_builder.match_kind(MatchKind::Standard);
                }
                "leftmostfirst" => {
                    tagger_builder = tagger_builder.match_kind(MatchKind::LeftmostFirst);
                }
                "leftmostlongest" | _ => {
                    tagger_builder = tagger_builder.match_kind(MatchKind::LeftmostLongest);
                }
            }
        }
    }

    let tagger = tagger_builder.build()?;

    if let Some(mut input_files) = matches.values_of("input") {
        while let Some(input) = input_files.next() {
            for entry in glob(input).expect("Failed to read glob pattern") {
                info!("Tagging {}", input.bold());
                match entry {
                    Ok(path) => {
                        let mut file = File::open(path)?;
                        let mut file_content = String::new();

                        file.read_to_string(&mut file_content)?;

                        for line in file_content.lines() {
                            let tagged_values = tagger.tag(&line);

                            match matches.value_of("format") {
                                Some("bioes") | Some("BIOES") => {
                                    println!("{}", BIOES::from(tagged_values).pretty_display());
                                }
                                Some("iob") | Some("IOB") | _ => {
                                    println!("{}", IOB::from(tagged_values).pretty_display());
                                }
                            }
                        }

                        println!("");
                    }
                    Err(e) => println!("{:?}", e),
                }
            }
        }
    }

    info!("Done");

    Ok(())
}
