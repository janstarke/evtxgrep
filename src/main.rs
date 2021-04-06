use argparse::{ArgumentParser, Store};
use simple_logger::SimpleLogger;
use regex::Regex;
use anyhow::{Error, Result};
use evtx::*;
use std::path::PathBuf;

struct GrepFilters {
    data: Regex,
    id: Regex
}

impl GrepFilters {
    fn matches(&self, record: &SerializedEvtxRecord<std::string::String>) -> bool {
        self.id.is_match(&record.event_record_id.to_string()) &&
        self.data.is_match(&record.data)
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
        ap.refer(&mut id).add_option(&["-I", "--id"], Store, "pattern used to filter event ids");
        ap.refer(&mut data).add_option(&["-D", "--data"], Store, "pattern to search for in the data section");
        ap.parse_args_or_exit();
    }

    let filter = GrepFilters {
        data: Regex::new(&data)?,
        id: Regex::new(&id)?
    };

    let fp = PathBuf::from(&evtxfile);
    if ! (fp.exists() && fp.is_file()) {
        return Err(Error::msg(format!("File {} does not exist", &evtxfile)));
    }
    
    let mut parser = EvtxParser::from_path(fp)?;
    for record in parser.records()
            .filter(|r| match r {
                Ok(_) => filter.matches(r.as_ref().unwrap()),
                Err(e) => {log::warn!("parser error: {}", e); false} }) {
                print_record(&record.unwrap());
            }
    Ok(())
}

fn print_record(
    record: &SerializedEvtxRecord<std::string::String>) {
    println!("{:?}", &record);
}