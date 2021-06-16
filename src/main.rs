use crate::xml_visitor::*;
use anyhow::{Error, Result};
use convert_case::{Case, Casing};
use evtx::*;
use simple_logger::SimpleLogger;
use std::path::PathBuf;
use std::string::String;
use clap::{Arg, App};

mod xml_visitor;

#[allow(unused_assignments)]
macro_rules! filter_opts {
    ($sf: expr, $f: ident, $use_or: ident, $( $p:ident :: $e:ident($v: ident) ),* ) => {
        let mut args = Vec::new();
        $(
            let opt_name = stringify!($v);
            let cli_option = opt_name.replace("_", "-");
            let evtx_name = opt_name.to_case(Case::UpperCamel).replace("Id", "ID");
            args.push((cli_option, format!("filter based on {}", evtx_name)));
        )*

        let app = App::new(env!("CARGO_PKG_NAME"))
                        .version(env!("CARGO_PKG_VERSION"))
                        .author(env!("CARGO_PKG_AUTHORS"))
                        .about(env!("CARGO_PKG_DESCRIPTION"))
                        .arg(Arg::with_name("EVTXFILE")
                            .help("name of the evtx file")
                            .required(true))
                        .arg(Arg::with_name("use_or")
                            .short("O").long("or")
                            .help("combine filters non-inclusively (use OR instead of AND, which is the default) "))
                        ;

        let mut idx = 0;
        $(
            idx += 1; // we are always one step too far, but this avoids an unused_assigments warning
            let app = app.arg(Arg::with_name(stringify!($v))
                .long(&args[idx-1].0)
                .help(&args[idx-1].1)
                .takes_value(true));
        )*
        let matches = app.get_matches();
        $f = matches.value_of("EVTXFILE").unwrap().to_string();
        $use_or = matches.is_present("use_or");

        $(
            if let Some(value) = matches.value_of(stringify!($v)) {
                $sf.push(RecordFilterSection::System($p::$e(value.to_string())))
            }
        )*
    };
}

fn main() -> Result<()> {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Warn)
        .init()
        .unwrap();

    #[allow(unused_mut)]
    let mut evtxfile: String;

    #[allow(unused_mut)]
    let mut use_or: bool = false;
    
    let mut filters: Vec<RecordFilterSection>= Vec::new();

    filter_opts!(
        filters,
        evtxfile,
        use_or,
        SystemFilter::Provider(provider),
        SystemFilter::EventID(event_id),
        SystemFilter::Level(level),
        SystemFilter::Task(task),
        SystemFilter::Opcode(opcode),
        SystemFilter::Keywords(keywords),
        SystemFilter::TimeCreated(time_created),
        SystemFilter::EventRecordID(event_record_id),
        SystemFilter::ActivityID(activity_id),
        SystemFilter::RelatedActivityID(related_activity_id),
        SystemFilter::ProcessID(process_id),
        SystemFilter::ThreadID(thread_id),
        SystemFilter::Channel(channel),
        SystemFilter::Computer(computer),
        SystemFilter::UserID(user_id)
    );

    let fp = PathBuf::from(&evtxfile);
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
