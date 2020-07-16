//! This contains a trait to be able to render
//! virtual dom into a writable buffer
//!
use crate::{
    html::attributes::AttributeValue,
    mt_dom::AttValue,
    Attribute,
    Element,
    Event,
    Node,
};
use std::fmt;

/// render node, elements to a writable buffer
pub trait Render {
    /// render instance to a writable buffer with indention
    fn render(&self, buffer: &mut dyn fmt::Write, indent: usize)
        -> fmt::Result;
}

impl<MSG> Render for Node<MSG> {
    fn render(
        &self,
        buffer: &mut dyn fmt::Write,
        indent: usize,
    ) -> fmt::Result {
        match self {
            Node::Element(element) => element.render(buffer, indent),
            Node::Text(text) => write!(buffer, "{}", text),
        }
    }
}

impl<MSG> Render for Element<MSG> {
    fn render(
        &self,
        buffer: &mut dyn fmt::Write,
        indent: usize,
    ) -> fmt::Result {
        write!(buffer, "<{}", self.tag())?;

        for (att_name, attr_values) in self.get_attribute_key_values() {
            println!("name = {}", att_name);
            write!(buffer, " {}=\"", att_name)?;
            render_attribute_values(&attr_values, buffer)?;
            write!(buffer, "\"")?;
        }
        write!(buffer, ">")?;

        let children = self.get_children();
        let first_child = children.get(0);
        let is_first_child_text_node =
            first_child.map(|node| node.is_text()).unwrap_or(false);

        let is_lone_child_text_node =
            children.len() == 1 && is_first_child_text_node;

        // do not indent if it is only text child node
        if is_lone_child_text_node {
            first_child.unwrap().render(buffer, indent)?;
        } else {
            // otherwise print all child nodes with each line and indented
            for child in self.get_children() {
                write!(buffer, "\n{}", "    ".repeat(indent + 1))?;
                child.render(buffer, indent + 1)?;
            }
        }
        // do not make a new line it if is only a text child node or it has no child nodes
        if !is_lone_child_text_node && !children.is_empty() {
            write!(buffer, "\n{}", "    ".repeat(indent))?;
        }
        write!(buffer, "</{}>", self.tag())?;
        Ok(())
    }
}

fn render_attribute_values<MSG>(
    attr_values: &[&AttValue<AttributeValue, Event, MSG>],
    buffer: &mut dyn fmt::Write,
) -> fmt::Result {
    let mut first = true;
    for av in attr_values {
        match av {
            AttValue::Plain(attr_value) => {
                if !first && !attr_value.is_style() {
                    write!(buffer, " ")?;
                }
                attr_value.render(buffer, 0)?;
            }
            _ => (),
        }
        first = false;
    }
    Ok(())
}

impl<MSG> Render for Attribute<MSG> {
    fn render(
        &self,
        buffer: &mut dyn fmt::Write,
        indent: usize,
    ) -> fmt::Result {
        match self.value() {
            AttValue::Plain(plain) => {
                write!(buffer, "{}=\"", self.name())?;
                plain.render(buffer, indent)?;
                write!(buffer, "\"")?;
            }
            _ => (),
        }
        Ok(())
    }
}

impl Render for AttributeValue {
    fn render(
        &self,
        buffer: &mut dyn fmt::Write,
        _index: usize,
    ) -> fmt::Result {
        match self {
            AttributeValue::Simple(simple) => {
                write!(buffer, "{}", simple.to_string())?;
            }
            AttributeValue::Style(styles_att) => {
                for s_att in styles_att {
                    write!(buffer, "{};", s_att)?;
                }
            }
            _ => (),
        }
        Ok(())
    }
}