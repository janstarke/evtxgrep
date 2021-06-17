use libxml::xpath::Context;
use strum_macros::{Display, EnumIter};
use libxml::tree::document::Document;

#[derive(Display, EnumIter)]
pub enum SystemFilterType {
  Provider,
  EventID,
  Level,
  Task,
  Opcode,
  Keywords,
  TimeCreated,
  EventRecordID,
  ActivityID,
  RelatedActivityID,
  ProcessID,
  ThreadID,
  Channel,
  Computer,
  UserID,
}

impl SystemFilterType {
  pub fn xpath_attribute(&self) -> &str {
    match self {
      Self::Provider => "Provider/@Name",
      Self::EventID => "EventID/text()",
      Self::Level => "Level/text()",
      Self::Task => "Task/text()",
      Self::Opcode => "Opcode/text()",
      Self::Keywords => "Keywords/text()",
      Self::TimeCreated => "TimeCreated/@SystemTime",
      Self::EventRecordID => "EventRecordID/text()",
      Self::ActivityID => "Correlation/@ActivityID",
      Self::RelatedActivityID => "Correlation/@RelatedActivityID",
      Self::ProcessID => "Execution/@ProcessID",
      Self::ThreadID => "Execution/@ThreadID",
      Self::Channel => "Channel/text()",
      Self::Computer => "Computer/text()",
      Self::UserID => "Security/@UserID",
    }
  }
}

pub struct SystemFilter {
  filter_type: SystemFilterType,
  value: String,
  ignore_case: bool,
}

impl ToString for SystemFilter {
  fn to_string(&self) -> String {
    if self.ignore_case {
      format!(
        "translate(System/{},'abcdefghijklmnopqrstuvwxyz','ABCDEFGHIJKLMNOPQRSTUVWXYZ')='{}'",
        self.filter_type.xpath_attribute(),
        self.value.to_uppercase()
      )
    } else {
      format!("System/{}='{}'", self.filter_type.xpath_attribute(), self.value)
    }
  }
}

impl SystemFilter {
  pub fn new(filter_type: SystemFilterType, value: String, ignore_case: bool) -> Self {
    Self {
      filter_type,
      value,
      ignore_case,
    }
  }
}

pub struct DataFilter {
  field_name: String,
  field_value: String,
  ignore_case: bool,
}

impl DataFilter {
  pub fn new(field_name: String, field_value: String, ignore_case: bool) -> Self {
    Self {
      field_name,
      field_value,
      ignore_case,
    }
  }
}

impl ToString for DataFilter {
  fn to_string(&self) -> String {
    if self.ignore_case {
      format!("translate(EventData/Data[translate(@Name,'abcdefghijklmnopqrstuvwxyz','ABCDEFGHIJKLMNOPQRSTUVWXYZ')='{}'],'abcdefghijklmnopqrstuvwxyz','ABCDEFGHIJKLMNOPQRSTUVWXYZ')='{}'", self.field_name.to_uppercase() , self.field_value.to_uppercase())
    } else {
      format!(
        "EventData/Data[@Name='{}']='{}'",
        self.field_name, self.field_value
      )
    }
  }
}

pub enum RecordFilterSection {
  System(SystemFilter),
  EventData(DataFilter),
}

impl ToString for RecordFilterSection {
  fn to_string(&self) -> String {
    match self {
      Self::System(s) => s.to_string(),
      Self::EventData(d) => d.to_string(),
    }
  }
}

pub struct XPathFilter {
  filter: String,
}

impl XPathFilter {
  pub fn new(system_filters: Vec<RecordFilterSection>, use_or: bool) -> Self {
    let combination = if use_or { " or " } else { " and " };
    let filter = system_filters
      .iter()
      .map(|f| f.to_string())
      .collect::<Vec<String>>()
      .join(combination);
    Self {
      filter: format!("//Event[{}]", filter),
    }
  }

  pub fn matches(&self, doc: &Document) -> bool {
    match Context::new(doc) {
      Ok(ctx) => match ctx.evaluate(&self.filter) {
        Ok(obj) => obj.get_number_of_nodes() > 0,
        Err(_) => panic!("unable to use XPath expression"),
      },
      Err(_) => panic!("unable to generate XPath context for document"),
    }
  }

  pub fn filter(&self) -> &str {
    &self.filter
  }
}