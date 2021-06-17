use evtx::err::SerializationResult;
use evtx::EvtxStructureVisitor;
use libxml::tree::document::{Document, SaveOptions};
use libxml::tree::node::Node;
use libxml::xpath::Context;
use strum_macros::{Display, EnumIter};

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

pub struct XmlVisitor<'f> {
  doc: Document,
  stack: Vec<Node>,
  filter: &'f Option<XPathFilter>,
}

impl<'f> XmlVisitor<'f> {
  pub fn new(filter: &'f Option<XPathFilter>) -> Self {
    let doc = Document::new().unwrap();
    let stack = Vec::new();

    Self { doc, stack, filter }
  }
}

impl<'f> ToString for XmlVisitor<'f> {
  fn to_string(&self) -> String {
    let mut options = SaveOptions::default();
    options.as_xml = true;
    options.no_declaration = true;
    options.format = true;
    self.doc.to_string_with_options(options)
  }
}

impl<'f> EvtxStructureVisitor for XmlVisitor<'f> {
  type VisitorResult = Option<String>;

  fn get_result(
    &self,
    _event_record_id: u64,
    _timestamp: chrono::DateTime<chrono::Utc>,
  ) -> Self::VisitorResult {
    if let Some(filter) = self.filter {
      if filter.matches(&self.doc) {
        Some(self.to_string())
      } else {
        None
      }
    } else {
      Some(self.to_string())
    }
  }

  /// called when a new record starts
  fn start_record(&mut self) -> SerializationResult<()> {
    Ok(())
  }

  /// called when the current records is finished
  fn finalize_record(&mut self) -> SerializationResult<()> {
    Ok(())
  }

  // called upon element content
  fn visit_characters(&mut self, _value: &str) -> SerializationResult<()> {
    let node = self.stack.last_mut().unwrap();
    if node.is_element_node() {
      node.set_content(_value)?;
    } else {
      let mut content = node.get_content();
      content.push_str(_value);
      node.set_content(&content)?;
    }
    Ok(())
  }

  fn visit_empty_element<'a, 'b>(
    &'a mut self,
    name: &'b str,
    attributes: Box<dyn Iterator<Item = (&'b str, &'b str)> + 'b>,
  ) -> SerializationResult<()>
  where
    'a: 'b,
  {
    let mut node = self
      .stack
      .last_mut()
      .unwrap()
      .add_text_child(None, name, "")
      .unwrap();

    for (key, value) in attributes {
      node.set_attribute(key, value).unwrap();
    }
    Ok(())
  }

  fn visit_start_element<'a, 'b>(
    &'a mut self,
    name: &'b str,
    attributes: Box<dyn Iterator<Item = (&'b str, &'b str)> + 'b>,
  ) -> SerializationResult<()>
  where
    'a: 'b,
  {
    let mut node = Node::new(name, None, &self.doc).unwrap();

    for (key, value) in attributes {
      node.set_attribute(key, value)?;
    }
    if self.stack.is_empty() {
      self.doc.set_root_element(&node);
    } else {
      let _ = self.stack.last_mut().unwrap().add_child(&mut node);
    }
    self.stack.push(node);
    Ok(())
  }

  fn visit_end_element(&mut self, _name: &str) -> SerializationResult<()> {
    self.stack.pop();
    Ok(())
  }
}
