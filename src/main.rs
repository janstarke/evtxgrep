use argparse::{ArgumentParser, Store};
use simple_logger::SimpleLogger;
use anyhow::{Error, Result};
use evtx::*;
use std::path::PathBuf;
use std::string::String;
use crate::xml_visitor::*;

mod xml_visitor;

fn main() -> Result<()> {
    SimpleLogger::new().with_level(log::LevelFilter::Warn).init().unwrap();

    let mut evtxfile = String::new();
    let mut filter_str = String::new();
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("regular expression based search in Windows Event Log files");
        ap.refer(&mut evtxfile).add_argument("evtxfile", Store, "name of the evtx file").required();
        ap.refer(&mut filter_str).add_option(&["-F", "--filter"], Store, "XPath filter condition against which each record is being matched");
        ap.parse_args_or_exit();
    }

    let fp = PathBuf::from(&evtxfile);
    if ! (fp.exists() && fp.is_file()) {
        return Err(Error::msg(format!("File {} does not exist", &evtxfile)));
    }
    
    let settings = ParserSettings::default().num_threads(0);
    let parser = EvtxParser::from_path(fp)?;
    let mut parser = parser.with_configuration(settings);

    let filter = if filter_str.is_empty() {
        None
    } else {
        Some(XPathFilter::new(format!("//*[{}]", filter_str)))
    };

    let records = parser.records_to_visitor(|| XmlVisitor::new(&filter)).filter_map(|r|
        match r {
            Ok(x) => Some(x),
            Err(e) => {log::warn!("parser error: {}", e); None}
        }
    );

    for record in records {
        if let Some(s) = record {
            println!("{}", s);
        }
    }
    Ok(())
}