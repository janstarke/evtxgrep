use argparse::{ArgumentParser, Store};
use simple_logger::SimpleLogger;
use regex::Regex;
use anyhow::{Context, Result};

fn main() -> Result<()> {
    SimpleLogger::new().with_level(log::LevelFilter::Debug).init().unwrap();

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

    let data = Regex::new(&data)?;
    let id = Regex::new(&id)?;
    Ok(())
}
/*
fn parse_regex(regex_str: &str) -> Result<Regex> {

}
*/