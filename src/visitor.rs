use evtx::{EvtxStructureVisitor};
use chrono::format::ParseError;

pub trait LineOutput {
    fn println(&self, line: &str);
}

#[derive(Debug)]
pub enum EvtxParseError {
    DateTime(ParseError),
}

impl From<ParseError> for EvtxParseError {
    fn from(err: ParseError) -> EvtxParseError {
        EvtxParseError::DateTime(err)
    }
}
