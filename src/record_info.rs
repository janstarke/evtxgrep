use std::cmp::Ordering;

#[derive(Eq)]
pub struct RecordInfo {
  pub xml_data: String,
  pub event_record_id: u64,
  pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl RecordInfo {
  pub fn new(
    xml_data: String,
    event_record_id: u64,
    timestamp: chrono::DateTime<chrono::Utc>,
  ) -> Self {
    Self {
      xml_data,
      event_record_id,
      timestamp,
    }
  }
}

impl Ord for RecordInfo {
  fn cmp(&self, other: &RecordInfo) -> Ordering {
    self.event_record_id.cmp(&other.event_record_id)
  }
}
impl PartialOrd for RecordInfo {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.event_record_id.cmp(&other.event_record_id))
  }
}
impl PartialEq for RecordInfo {
  fn eq(&self, other: &RecordInfo) -> bool {
    self.event_record_id == other.event_record_id
  }
}
