use evtx::EvtxStructure;
use derive_new::*;
use chrono::format::ParseError;

#[derive(Debug)]
pub enum EvtxParseError {
    DateTime(ParseError),
}

impl From<ParseError> for EvtxParseError {
    fn from(err: ParseError) -> EvtxParseError {
        EvtxParseError::DateTime(err)
    }
}

pub trait EvtxVisitor {
    fn visit(&self, record: &EvtxStructure) -> std::result::Result<String, EvtxParseError>;
}

#[derive(new)]
pub struct CsvVisitor {
}

impl EvtxVisitor for CsvVisitor {
    fn visit(&self, record: &EvtxStructure) -> std::result::Result<String, EvtxParseError> {
        Ok(format!("{};{};{}",
            record.event_record_id(),
            record.time_created()?,
            record.event_id()))
    }
}