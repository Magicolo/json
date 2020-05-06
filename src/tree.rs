use crate::*;

#[derive(Debug, Clone)]
pub struct Tree {
    pub root: Node,
    pub(crate) nodes: Vec<Node>,
    pub(crate) text: String,
}

impl Tree {
    #[inline]
    pub fn null() -> Node {
        Node::Null
    }

    #[inline]
    pub fn boolean(value: bool) -> Node {
        Node::Boolean(value)
    }

    #[inline]
    pub fn number(value: f64) -> Node {
        Node::Number(value)
    }

    #[inline]
    pub fn string(value: &str) -> Node {
        Node::String(String::from(value))
    }

    #[inline]
    pub fn object(members: Vec<Node>) -> Node {
        Node::Object(members)
    }

    #[inline]
    pub fn array(items: Vec<Node>) -> Node {
        Node::Array(items)
    }

    pub fn process<'a>(&'a mut self) {
        let mut root = self.root.clone();
        if self.nodes.len() > 0 || self.text.len() > 0 {
            self.process_descent(&mut root);
            self.nodes.truncate(0);
            self.text.truncate(0);
        } else {
            self.process_node(&mut root);
        }
        self.root = root;
    }

    #[inline]
    pub fn get_children<'a>(&'a self, node: &'a Node) -> Option<&'a [Node]> {
        match node {
            Node::RawArray(range) => Some(&self.nodes[range.0..range.1]),
            Node::Array(items) => Some(items),
            Node::RawObject(range) => Some(&self.nodes[range.0..range.1]),
            Node::Object(members) => Some(members),
            _ => None,
        }
    }

    #[inline]
    pub fn get_items<'a>(&'a self, node: &'a Node) -> Option<&'a [Node]> {
        match node {
            Node::RawArray(range) => Some(&self.nodes[range.0..range.1]),
            Node::Array(items) => Some(&items),
            _ => None,
        }
    }

    #[inline]
    pub fn get_item<'a>(&'a self, node: &'a Node, index: usize) -> Option<&'a Node> {
        self.get_items(node).and_then(|items| items.get(index))
    }

    #[inline]
    pub fn add_item<'a>(&'a mut self, node: &'a mut Node, item: Node) -> Option<usize> {
        self.process_node(node);
        if let Node::Array(items) = node {
            items.push(item);
            Some(items.len())
        } else {
            None
        }
    }

    #[inline]
    pub fn insert_item<'a>(
        &'a mut self,
        node: &'a mut Node,
        index: usize,
        item: Node,
    ) -> Option<usize> {
        self.process_node(node);
        if let Node::Array(items) = node {
            items.insert(index, item);
            Some(items.len())
        } else {
            None
        }
    }

    #[inline]
    pub fn remove_item<'a>(&'a mut self, node: &'a mut Node, index: usize) -> Option<Node> {
        self.process_node(node);
        if let Node::Array(items) = node {
            Some(items.remove(index))
        } else {
            None
        }
    }

    #[inline]
    pub fn get_members<'a>(&'a self, node: &'a Node) -> Option<&'a [Node]> {
        match node {
            Node::RawObject(range) => Some(&self.nodes[range.0..range.1]),
            Node::Object(members) => Some(&members),
            _ => None,
        }
    }

    pub fn get_member<'a>(&'a self, node: &'a Node, key: &str) -> Option<&'a Node> {
        if let Some(members) = self.get_members(node) {
            for i in (0..members.len()).step_by(2) {
                if let Some(member) = self.get_string(&self.nodes[i]) {
                    if key == member {
                        return Some(&self.nodes[i + 1]);
                    }
                }
            }
        }
        None
    }

    #[inline]
    pub fn get_string<'a>(&'a self, node: &'a Node) -> Option<&'a str> {
        match node {
            Node::RawString(range) => Some(&self.text[range.0..range.1]),
            Node::String(value) => Some(&value),
            _ => None,
        }
    }

    fn process_descent<'a>(&'a self, node: &'a mut Node) {
        self.process_node(node);
        match node {
            Node::Array(items) => {
                for i in 0..items.len() {
                    self.process_descent(&mut items[i]);
                }
            }
            Node::Object(members) => {
                for i in 0..members.len() {
                    self.process_descent(&mut members[i]);
                }
            }
            _ => {}
        }
    }

    fn process_node<'a>(&'a self, node: &'a mut Node) {
        match node {
            Node::RawArray(range) => {
                *node = Node::Array(Vec::from(&self.nodes[range.0..range.1]));
            }
            Node::RawObject(range) => {
                *node = Node::Object(Vec::from(&self.nodes[range.0..range.1]));
            }
            Node::RawString(range) => {
                *node = Node::String(String::from(&self.text[range.0..range.1]));
            }
            _ => {}
        }
    }
}
