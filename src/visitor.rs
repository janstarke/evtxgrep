use evtx::EvtxStructure;
use derive_new::*;

pub trait EvtxVisitor {
    fn visit(&self, record: &EvtxStructure) -> String;
}

#[derive(new)]
pub struct CsvVisitor {
}


impl EvtxVisitor for CsvVisitor {
    fn visit(&self, record: &EvtxStructure) -> String {
        format!("{};{}",
            record.event_record_id(),
            record.timestamp())
    }
}