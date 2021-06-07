use evtx::{EvtxStructureVisitor};
use chrono::format::ParseError;
use std::collections::hash_map::Iter;
use std::cell::{Ref, RefCell};

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

pub struct XmlVisitor<'a> {
    indent: RefCell<String>,
    output: &'a (dyn LineOutput + 'a),
}

impl<'a> XmlVisitor<'a> {
    pub fn new(output: &'a impl LineOutput) -> Self {
        Self {
            indent: RefCell::new(String::new()),
            output
        }
    }

    fn indent(&self) -> Ref<'_, String> {
        self.indent.borrow()
    }

    fn enter(&self) {
        self.indent.borrow_mut().push_str("  ");
    }

    fn leave(&self) {
        let mut i = self.indent.borrow_mut();
        i.pop();
        i.pop();
    }

    fn format_attributes(&self, attributes: Iter<String, String>) -> String {
        let mut res = String::new();
        for (key, value) in attributes {
            res.push_str(&format!(" {}=\"{}\"", key, value));
        }
        res
    }
}

impl<'a> EvtxStructureVisitor for XmlVisitor<'a> {
    fn visit_empty_element(&self, name: &str, attributes: Iter<String, String>) {
        self.output.println(
            &format!("{}<{}{}/>", self.indent(), name, self.format_attributes(attributes))
        );
    }

    fn visit_simple_element(&self, name: &str, attributes: Iter<String, String>, content: &str) {
        if content.is_empty() {
            self.visit_empty_element(name, attributes);
        } else {
            let mut start_tag = format!("{}<{}{}/>", self.indent(), name, self.format_attributes(attributes));
            let end_tag = format!("</{}>", name);
            if start_tag.len() + content.len() + end_tag.len() > 80 {
                self.output.println(&start_tag);
                self.output.println(&format!("{}{}", self.indent(), content));
                self.output.println(&format!("{}{}", self.indent(), end_tag));
            } else {
                start_tag.push_str(content);
                start_tag.push_str(&end_tag);
                self.output.println(&start_tag);
            }
        }
    }

    fn visit_start_element(&self, name: &str, attributes: Iter<String, String>) {
        self.output.println(
            &format!("{}<{}{}/>", self.indent(), name, self.format_attributes(attributes))
        );
        self.enter();
    }

    fn visit_end_element(&self, name: &str) {
        self.leave();
        self.output.println(
            &format!("{}</{}>", self.indent(), name)
        );
    }
}