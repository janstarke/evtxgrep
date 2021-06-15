use evtx::err::SerializationResult;
use evtx::EvtxStructureVisitor;
use libxml::tree::document::{Document, SaveOptions};
use libxml::tree::node::Node;

pub struct XmlVisitor {
  doc: Document,
  stack: Vec<Node>,
}

impl XmlVisitor {
  pub fn new() -> Self {
    let mut doc = Document::new().unwrap();
    let mut stack = Vec::new();

    let root = Node::new("Event", None, &doc).unwrap();
    doc.set_root_element(&root);
    stack.push(root);

    Self { doc, stack }
  }
}

impl ToString for XmlVisitor {
  fn to_string(&self) -> String {
    let mut options = SaveOptions::default();
    options.as_xml = true;
    options.no_declaration = true;
    options.format = true;
    self.doc.to_string_with_options(options)
  }
}

impl EvtxStructureVisitor for XmlVisitor {
  type VisitorResult = Option<String>;

  fn get_result(
    &self,
    _event_record_id: u64,
    _timestamp: chrono::DateTime<chrono::Utc>,
  ) -> Self::VisitorResult {
    Some(self.to_string())
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

  fn visit_simple_element<'a, 'b>(
    &'a mut self,
    name: &'b str,
    attributes: Box<dyn Iterator<Item = (&'b str, &'b str)> + 'b>,
    content: &'b str,
  ) -> SerializationResult<()>
  where
    'a: 'b,
  {
    let mut node = self
      .stack
      .last_mut()
      .unwrap()
      .add_text_child(None, name, content)
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
    let _ = self.stack.last_mut().unwrap().add_child(&mut node);
    self.stack.push(node);
    Ok(())
  }

  fn visit_end_element(&mut self, _name: &str) -> SerializationResult<()> {
    self.stack.pop();
    Ok(())
  }
}
