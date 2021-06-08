use evtx::{EvtxStructureVisitor};
use std::collections::hash_map::Iter;
use std::cell::{Ref, RefCell};
use crate::visitor::LineOutput;
use libxml::tree::document::{Document, SaveOptions};
use libxml::tree::node::Node;
use libxml::tree::namespace::Namespace;

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

        Self {
            doc,
            stack
        }
    }

    fn set_attributes(node: &mut Node, attributes: Iter<String, String>) {
        for (key, value) in attributes {
            node.set_attribute(key, value).unwrap();
        }
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
    fn visit_empty_element(&mut self, name: &str, attributes: Iter<String, String>) {
        let mut node = self.stack.last_mut().unwrap().add_text_child(
            None,
            name,
            ""
        ).unwrap();
        XmlVisitor::set_attributes(&mut node, attributes);
    }

    fn visit_simple_element(&mut self, name: &str, attributes: Iter<String, String>, content: &str) {
        let mut node = self.stack.last_mut().unwrap().add_text_child(
            None,
            name,
            content
        ).unwrap();
        XmlVisitor::set_attributes(&mut node, attributes);
    }

    fn visit_start_element(&mut self, name: &str, attributes: Iter<String, String>) {
        let mut node = Node::new(name, None, &self.doc).unwrap();
        XmlVisitor::set_attributes(&mut node, attributes);
        self.stack.last_mut().unwrap().add_child(&mut node);
        self.stack.push(node);
    }

    fn visit_end_element(&mut self, name: &str) {
        self.stack.pop();
    }
}