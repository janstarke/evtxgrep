use argparse::{ArgumentParser, Store, StoreTrue};
use simple_logger::SimpleLogger;
use regex::Regex;
use anyhow::{Error, Result};
use evtx::*;
use evtx::err::EvtxError;
use std::path::PathBuf;
use serde_json::{Value};
use std::string::String;

struct GrepFilters {
    data: Regex,
    id: Regex
}

trait FilterMethod {
    fn matches(&self, record: &SerializedEvtxRecord<String>) -> bool;
}

struct XmlFilter {
    filters: GrepFilters,
}

struct JsonFilter {
    filters: GrepFilters,
}

impl FilterMethod for XmlFilter {
    fn matches(&self, _record: &SerializedEvtxRecord<String>) -> bool {
        false
    }
}

impl FilterMethod for JsonFilter {
    fn matches(&self, record: &SerializedEvtxRecord<std::string::String>) -> bool {
        let v: Value = serde_json::from_str(&record.data).unwrap();
        let event = &v["Event"];
        let event_id = &event["System"]["EventID"];
        if ! self.filters.id.is_match(&event_id.to_string()) {
            return false;
        }
        Self::matches_value(&self.filters.data, &event["EventData"])
    }
}

impl JsonFilter {
    fn matches_value(regex: &Regex, value: &Value) -> bool {
        match value {
            Value::Null         => false,
            Value::Bool(_)      => false,
            Value::Number(n)    => regex.is_match(&n.to_string()),
            Value::String(s)    => regex.is_match(s),
            Value::Array(a)     => a.iter().fold(false, |m, x| m || Self::matches_value(&regex, x)),
            Value::Object(o)    => o.values().fold(false, |m, x| m || Self::matches_value(&regex, x))
        }
    }
}

fn main() -> Result<()> {
    SimpleLogger::new().with_level(log::LevelFilter::Warn).init().unwrap();

    let mut evtxfile = String::new();
    let mut data = String::new();
    let mut id = String::new();
    let mut xml_format = false;
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("regular expression based search in Windows Event Log files");
        ap.refer(&mut evtxfile).add_argument("evtxfile", Store, "name of the evtx file").required();
        ap.refer(&mut xml_format).add_option(&["-X", "--xml"], StoreTrue, "use XML format instead of JSON");
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

    let records: Box<dyn Iterator<Item = Result<SerializedEvtxRecord<String>, EvtxError>>> = if xml_format {
        Box::new(parser.records())
    } else {
        Box::new(parser.records_json())
    };

    let filter_method: Box<dyn FilterMethod> = if xml_format {
        Box::new(XmlFilter {filters})
    } else {
        Box::new(JsonFilter {filters})
    };

    let records = records.filter(|r| match r {
        Ok(_) => filter_method.matches(r.as_ref().unwrap()),
        Err(e) => {log::warn!("parser error: {}", e); false}
    });
    for record in records {
        print_record(&record.unwrap());
    }
    Ok(())
}

fn print_record(
    record: &SerializedEvtxRecord<std::string::String>) {
    println!("{}", &record.data);
}