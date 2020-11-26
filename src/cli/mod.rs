use clap::{crate_authors, crate_name, crate_version, App, Arg, ArgMatches, SubCommand};

pub fn cli() -> ArgMatches<'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .about("A BIOES IOB tagger")
        .arg(
            Arg::with_name("dictionary")
                .short("d")
                .long("dictionary")
                .alias("dictionaries")
                .value_name("FILE")
                .help("Sets dictionaries (TSV format)")
                .multiple(true)
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("input")
                .long("input")
                .short("i")
                .help("Sets the input files to use")
                .required(true)
                .multiple(true)
                .value_delimiter(",")
                .min_values(1),
        )
        .arg(
            Arg::with_name("format")
                .long("format")
                .short("f")
                .help("Sets the output format")
                .default_value("iob")
                .possible_values(&["iob", "IOB", "bioes", "BIOES"]),
        )
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .arg(
            Arg::with_name("silent")
                .short("s")
                .long("silent")
                .help("Hides log information"),
        )
        .subcommand(
            SubCommand::with_name("tagger")
                .about("controls tagger features")
                .arg(
                    Arg::with_name("case_sensitive")
                        .long("case_sensitive")
                        .short("c")
                        .takes_value(true)
                        .possible_values(&["true", "false"])
                        .value_name("BOOL")
                        .help("Enables/Disables the case sensitivity of the tagger"),
                )
                .arg(
                    Arg::with_name("word_matching")
                        .short("w")
                        .long("word_matching")
                        .takes_value(true)
                        .possible_values(&["true", "false"])
                        .value_name("BOOL")
                        .default_value("true")
                        .help("Enables/Disables word matching"),
                )
                .arg(
                    Arg::with_name("match_kind")
                        .short("m")
                        .long("match_kind")
                        .takes_value(true)
                        .possible_values(&["standard", "leftmostfirst", "leftmostlongest"])
                        .default_value("leftmostlongest")
                        .value_name("MATCH KIND")
                        .help("Sets tagging matchkind"),
                ),
        )
        .get_matches()
}
