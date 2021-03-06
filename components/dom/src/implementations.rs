use super::dom_ref::DOMObject;
use std::any::Any;

use super::character_data::CharacterData;
use super::comment::Comment;
use super::document::{Document, DocumentType};
use super::element::Element;
use super::node::Node;
use super::text::Text;

impl DOMObject for Node {
    fn as_node(&self) -> &Node {
        self
    }

    fn as_node_mut(&mut self) -> &mut Node {
        self
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn as_element(&self) -> Option<&Element> {
        None
    }

    fn as_element_mut(&mut self) -> Option<&mut Element> {
        None
    }
}

impl DOMObject for Document {
    fn as_node(&self) -> &Node {
        &self.node
    }

    fn as_node_mut(&mut self) -> &mut Node {
        &mut self.node
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn as_element(&self) -> Option<&Element> {
        None
    }

    fn as_element_mut(&mut self) -> Option<&mut Element> {
        None
    }
}

impl DOMObject for DocumentType {
    fn as_node(&self) -> &Node {
        &self.node
    }

    fn as_node_mut(&mut self) -> &mut Node {
        &mut self.node
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn as_element(&self) -> Option<&Element> {
        None
    }

    fn as_element_mut(&mut self) -> Option<&mut Element> {
        None
    }
}

impl DOMObject for Element {
    fn as_node(&self) -> &Node {
        &self.node
    }

    fn as_node_mut(&mut self) -> &mut Node {
        &mut self.node
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn as_element(&self) -> Option<&Element> {
        Some(self)
    }

    fn as_element_mut(&mut self) -> Option<&mut Element> {
        Some(self)
    }
}

impl DOMObject for Comment {
    fn as_node(&self) -> &Node {
        &self.character_data.node
    }

    fn as_node_mut(&mut self) -> &mut Node {
        &mut self.character_data.node
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn as_element(&self) -> Option<&Element> {
        None
    }

    fn as_element_mut(&mut self) -> Option<&mut Element> {
        None
    }
}

impl DOMObject for Text {
    fn as_node(&self) -> &Node {
        &self.character_data.node
    }

    fn as_node_mut(&mut self) -> &mut Node {
        &mut self.character_data.node
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn as_element(&self) -> Option<&Element> {
        None
    }

    fn as_element_mut(&mut self) -> Option<&mut Element> {
        None
    }
}

impl DOMObject for CharacterData {
    fn as_node(&self) -> &Node {
        &self.node
    }

    fn as_node_mut(&mut self) -> &mut Node {
        &mut self.node
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn as_element(&self) -> Option<&Element> {
        None
    }

    fn as_element_mut(&mut self) -> Option<&mut Element> {
        None
    }
}
