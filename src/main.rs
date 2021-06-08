use argparse::{ArgumentParser, Store};
use simple_logger::SimpleLogger;
use regex::Regex;
use anyhow::{Error, Result};
use evtx::*;
use std::path::PathBuf;
use std::string::String;
use crate::visitor::*;
use crate::xml_visitor::*;
use crate::xml_output_visitor::XmlOutputVisitor;

mod visitor;
mod xml_visitor;
mod xml_output_visitor;

#[allow(unused)]
struct GrepFilters {
    data: Regex,
    id: Regex
}

trait FilterMethod {
    fn matches(&self, record: &EvtxStructure) -> bool;
}

#[allow(unused)]
struct XmlFilter {
    filters: GrepFilters,
}

#[allow(unused)]
struct JsonFilter {
    filters: GrepFilters,
}

#[allow(unused)]
impl FilterMethod for XmlFilter {

    fn matches(&self, _record: &EvtxStructure) -> bool {
        true
    }
}

fn main() -> Result<()> {
    SimpleLogger::new().with_level(log::LevelFilter::Warn).init().unwrap();

    let mut evtxfile = String::new();
    let mut data = String::new();
    let mut id = String::new();
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("regular expression based search in Windows Event Log files");
        ap.refer(&mut evtxfile).add_argument("evtxfile", Store, "name of the evtx file").required();
        ap.refer(&mut data).add_option(&["-D", "--data"], Store, "pattern to search for in the data section");
        ap.refer(&mut id).add_option(&["-I", "--id"], Store, "pattern used to filter event ids");
        ap.parse_args_or_exit();
    }

    let filters = GrepFilters {
        data: Regex::new(&data)?,
        id: Regex::new(&id)?
    };

    let fp = PathBuf::from(&evtxfile);
    if ! (fp.exists() && fp.is_file()) {
        return Err(Error::msg(format!("File {} does not exist", &evtxfile)));
    }
    
    let settings = ParserSettings::default().num_threads(0);
    let parser = EvtxParser::from_path(fp)?;
    let mut parser = parser.with_configuration(settings);

    let records = parser.records_struct();
    let filter_method = XmlFilter {filters};

    let mut records: Vec<EvtxStructure> = records.filter_map(|r| match r {
        Ok(s) => if filter_method.matches(&s) {
            Some (s)
        } else {
            None
        },
        Err(e) => {log::warn!("parser error: {}", e); None}
    }).collect();
    records.sort_unstable();
    let line_printer = LinePrinter{};
    let mut visitor = XmlOutputVisitor::new(&line_printer);

    for record in records {
        let mut xml_visitor = XmlVisitor::new();
        record.visit_structure(&mut xml_visitor);
        println!("{}", xml_visitor.to_string());

        //record.visit_structure(&mut visitor);
    }
    Ok(())
}

struct LinePrinter {}

impl LineOutput for LinePrinter {
    fn println(&self, line: &str) {
        println!("{}", line);
    }
}