use evtx::err::SerializationResult;
use evtx::EvtxStructureVisitor;
use libxml::tree::document::{Document, SaveOptions};
use libxml::tree::node::Node;
use libxml::xpath::Context;

pub struct XPathFilter {
  filter: String,
}

impl XPathFilter {
  pub fn new(filter: String) -> Self {
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
