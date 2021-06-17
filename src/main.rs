use crate::xml_visitor::*;
use anyhow::{Error, Result};
use clap::{App, Arg};
use convert_case::{Case, Casing};
use evtx::*;
use simple_logger::SimpleLogger;
use std::path::PathBuf;
use strum::IntoEnumIterator;

mod xml_visitor;

fn main() -> Result<()> {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Warn)
        .init()
        .unwrap();

    let mut app = App::new(env!("CARGO_PKG_NAME"))
    .version(env!("CARGO_PKG_VERSION"))
    .author(env!("CARGO_PKG_AUTHORS"))
    .about(env!("CARGO_PKG_DESCRIPTION"))
    .arg(Arg::with_name("EVTXFILE")
        .help("name of the evtx file")
        .required(true))
    .arg(Arg::with_name("use_or")
        .short("O").long("or")
        .help("combine filters non-inclusively (use OR instead of AND, which is the default) "))
    .arg(Arg::with_name("ignore_case")
        .short("i").long("ignore-case")
        .help("ignore case"))
    .arg(Arg::with_name("DATA:FILTER")
        .short("D")
        .long("data")
        .multiple(true)
        .takes_value(true)
        .number_of_values(1)
        .help("key-value pair, separated by colon, to filter based on entries in the data section"))
    ;
    let mut option_strings = Vec::new();
    for filter_type in SystemFilterType::iter() {
        let cli_option = filter_type
            .to_string()
            .to_case(Case::Snake)
            .replace("_", "-");
        let message = format!("filter based on {}", filter_type.to_string());
        option_strings.push((filter_type.to_string(), cli_option, message));
    }

    for os in option_strings.iter() {
        app = app.arg(
            Arg::with_name(&os.0)
                .long(&os.1)
                .help(&os.2)
                .takes_value(true),
        );
    }
    let matches = app.get_matches();
    let evtxfile = matches.value_of("EVTXFILE").unwrap();
    let use_or = matches.is_present("use_or");
    let ignore_case = matches.is_present("ignore_case");

    let mut filters: Vec<RecordFilterSection> = Vec::new();
    for filter_type in SystemFilterType::iter() {
        if let Some(value) = matches.value_of(filter_type.to_string()) {
            filters.push(RecordFilterSection::System(SystemFilter::new(filter_type, value.to_string(), ignore_case)))
        }
    }

    if let Some(values) = matches.values_of("DATA:FILTER") {
        for v in values {
            let pair: Vec<&str> = v.splitn(2, ":").collect();
            if pair.len() != 2 {
                eprintln!("illegal data filter: '{}'", v);
                std::process::exit(-1);
            }
            filters.push(RecordFilterSection::EventData(
                DataFilter::new(pair[0].to_owned(), pair[1].to_owned(), ignore_case)));
        }
    }

    let fp = PathBuf::from(evtxfile);
    if !(fp.exists() && fp.is_file()) {
        return Err(Error::msg(format!("File {} does not exist", &evtxfile)));
    }
    let settings = ParserSettings::default().num_threads(0);
    let parser = EvtxParser::from_path(fp)?;
    let mut parser = parser.with_configuration(settings);

    let filter = if filters.is_empty() {
        None
    } else {
        Some(XPathFilter::new(filters, use_or))
    };

    #[cfg(debug_assertions)]
    if let Some(ref filter) = filter {
        println!("match against {}", filter.filter());
    } else {
        panic!("no match");
    }

    let records = parser
        .records_to_visitor(|| XmlVisitor::new(&filter))
        .filter_map(|r| match r {
            Ok(x) => Some(x),
            Err(e) => {
                log::warn!("parser error: {}", e);
                None
            }
        });

    for record in records {
        if let Some(s) = record {
            println!("{}", s);
        }
    }
    Ok(())
}
