use crate::filter::XPathFilter;
use crate::record_info::RecordInfo;
use evtx::err::SerializationResult;
use evtx::EvtxStructureVisitor;
use libxml::tree::document::{Document, SaveOptions};
use libxml::tree::node::Node;

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
  type VisitorResult = Option<RecordInfo>;

  fn get_result(
    &self,
    event_record_id: u64,
    timestamp: chrono::DateTime<chrono::Utc>,
  ) -> Self::VisitorResult {
    if let Some(filter) = self.filter {
      if filter.matches(&self.doc) {
        Some(RecordInfo::new(self.to_string(), event_record_id, timestamp))
      } else {
        None
      }
    } else {
      Some(RecordInfo::new(self.to_string(), event_record_id, timestamp))
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

  fn visit_start_element<'a, 'b, I>(
    &'a mut self,
    name: &'b str,
    attributes: I,
  ) -> SerializationResult<()>
  where
    'a: 'b,
    I: Iterator<Item = (&'b str, &'b str)> + 'b,
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
