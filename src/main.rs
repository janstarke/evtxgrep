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

    let fp = PathBuf::from(&evtxfile);
    if ! (fp.exists() && fp.is_file()) {
        return Err(Error::msg(format!("File {} does not exist", &evtxfile)));
    }
    
    let settings = ParserSettings::default().num_threads(0);
    let parser = EvtxParser::from_path(fp)?;
    let mut parser = parser.with_configuration(settings);

    let records = parser.records_to_visitor(|| XmlVisitor::new()).filter_map(|r|
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