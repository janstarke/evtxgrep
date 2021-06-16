use crate::xml_visitor::*;
use anyhow::{Error, Result};
use argparse::{ArgumentParser, Store};
use convert_case::{Case, Casing};
use evtx::*;
use simple_logger::SimpleLogger;
use std::path::PathBuf;
use std::string::String;

mod xml_visitor;

#[allow(unused_assignments)]
macro_rules! filter_opts {
    ($sf: expr, $f: ident, $( $p:ident :: $e:ident($v: ident) ),* ) => {
        let mut args = Vec::new();
        $(
            let mut $v = String::new();
            let opt_name = stringify!($v);
            let cli_option = format!("--{}", opt_name.replace("_", "-"));
            let evtx_name = opt_name.to_case(Case::UpperCamel).replace("Id", "ID");
            args.push((cli_option, format!("filter based on {}", evtx_name)));
        )*
        {
            let mut ap = ArgumentParser::new();
            ap.set_description("regular expression based search in Windows Event Log files");
            ap.refer(&mut $f)
                .add_argument("evtxfile", Store, "name of the evtx file")
                .required();

            let mut idx = 0;
            $(
                idx += 1; // we are always one step too far, but this avoids an unused_assigments warning
                ap.refer(&mut $v).add_option(&[&args[idx-1].0], Store, &args[idx-1].1);
            )*
            ap.parse_args_or_exit();
        }

        let mut system_filters = Vec::new();
        $(
            if !$v.is_empty() {
                system_filters.push(RecordFilterSection::System($p::$e($v)))
            }
        )*
    };
}

fn main() -> Result<()> {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Warn)
        .init()
        .unwrap();

    let mut evtxfile = String::new();
    let mut filter_str = String::new();
    let filters: Vec<RecordFilterSection>= Vec::new();

    filter_opts!(
        filters,
        evtxfile,
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

    let filter = if filter_str.is_empty() && filters.is_empty() {
        None
    } else {
        Some(XPathFilter::new(filters))
    };

    #[cfg(debug_assertions)]
    if let Some(ref filter) = filter {
        println!("match against {}", filter.filter());
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
