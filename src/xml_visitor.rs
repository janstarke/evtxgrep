use evtx::err::SerializationResult;
use evtx::EvtxStructureVisitor;
use libxml::tree::document::{Document, SaveOptions};
use libxml::tree::node::Node;
use libxml::xpath::Context;

pub enum SystemFilter {
  Provider(String),
  EventID(String),
  Level(String),
  Task(String),
  Opcode(String),
  Keywords(String),
  TimeCreated(String),
  EventRecordID(String),
  ActivityID(String),
  RelatedActivityID(String),
  ProcessID(String),
  ThreadID(String),
  Channel(String),
  Computer(String),
  UserID(String)
}

impl ToString for SystemFilter {
  fn to_string(&self) -> String {
    match self {
      Self::Provider(v) => format!("Provider/@Name='{}'", v),
      Self::EventID(v) => format!("EventID/text()='{}'", v),
      Self::Level(v) => format!("Level/text()='{}'", v),
      Self::Task(v) => format!("Task/text()='{}'", v),
      Self::Opcode(v) => format!("Opcode/text()='{}'", v),
      Self::Keywords(v) => format!("Keywords/text()='{}'", v),
      Self::TimeCreated(v) => format!("TimeCreated/@SystemTime='{}'", v),
      Self::EventRecordID(v) => format!("EventRecordID/text()='{}'", v),
      Self::ActivityID(v) => format!("Correlation/@ActivityID='{}'", v),
      Self::RelatedActivityID(v) => format!("Correlation/@RelatedActivityID='{}'", v),
      Self::ProcessID(v) => format!("Execution/@ProcessID='{}'", v),
      Self::ThreadID(v) => format!("Execution/@ThreadID='{}'", v),
      Self::Channel(v) => format!("Channel/text()='{}'", v),
      Self::Computer(v) => format!("Computer/text()='{}'", v),
      Self::UserID(v) => format!("Security/@UserID='{}'", v),
    }
  }
}

pub enum RecordFilterSection {
  System(SystemFilter),
  EventData(String, String)
}

impl ToString for RecordFilterSection {
  fn to_string(&self) -> String{
    match self {
      Self::System(s) => format!("System/{}", s.to_string()),
      Self::EventData(k,v) => format!("EventData/Data[@Name='{}']='{}'", k , v)
    }
  }
}

pub struct XPathFilter {
  filter: String,
}

impl XPathFilter {
  pub fn new(system_filters: Vec<RecordFilterSection>) -> Self {
    let filter = system_filters.iter().map(|f| f.to_string()).collect::<Vec<String>>().join(" and ");
    Self { filter }
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
    let node = self
      .stack
      .last_mut()
      .unwrap();
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
