use crate::*;

#[derive(Debug, Clone)]
pub struct Tree {
    pub(crate) root: Node,
    pub(crate) nodes: Vec<Node>,
    pub(crate) text: String,
}

pub enum Path<'a> {
    Member(&'a str),
    Item(usize),
}

impl Tree {
    #[inline]
    pub const fn new(root: Node) -> Tree {
        Tree {
            root,
            nodes: Vec::new(),
            text: String::new(),
        }
    }

    pub fn process(&mut self) {
        if self.nodes.len() > 0 || self.text.len() > 0 {
            Self::process_descent(&mut self.root, &self.nodes, &self.text);
            self.nodes.clear();
            self.text.clear();
        } else {
            Self::process_node(&mut self.root, &self.nodes, &self.text);
        }
    }

    #[inline]
    pub fn get<'a>(&'a mut self, path: &[Path]) -> Option<&'a mut Node> {
        let root = unsafe { &mut *(&mut self.root as *mut Node) };
        self.get_from(root, path)
    }

    pub fn get_from<'a>(&'a mut self, node: &'a mut Node, path: &[Path]) -> Option<&'a mut Node> {
        let mut node = node as *mut Node;
        for segment in path {
            match *segment {
                Path::Member(key) => match unsafe { &mut *node } {
                    Node::Object(members) => match Self::find_member(key, members, &self.text) {
                        Some(value) => node = value,
                        None => return None,
                    },
                    Node::RawObject(range) => {
                        match Self::find_member(key, &mut self.nodes[range.0..range.1], &self.text)
                        {
                            Some(value) => node = value,
                            None => return None,
                        }
                    }
                    _ => return None,
                },
                Path::Item(index) => match unsafe { &mut *node } {
                    Node::Array(items) => match items.get_mut(index) {
                        Some(item) => node = item,
                        None => return None,
                    },
                    Node::RawArray(range) => match self.nodes[range.0..range.1].get_mut(index) {
                        Some(item) => node = item,
                        None => return None,
                    },
                    _ => return None,
                },
            }
        }

        let node = unsafe { &mut *node };
        Self::process_node(node, &self.nodes, &self.text);
        Some(node)
    }

    #[inline]
    pub fn get_items<'a>(&'a self, node: &'a Node) -> Option<&[Node]> {
        match node {
            Node::RawArray(range) => Some(&self.nodes[range.0..range.1]),
            Node::Array(items) => Some(&items),
            _ => None,
        }
    }

    #[inline]
    pub fn add_item(&mut self, node: &mut Node, item: Node) -> bool {
        Self::process_node(node, &self.nodes, &self.text);
        if let Node::Array(items) = node {
            items.push(item);
            true
        } else {
            false
        }
    }

    #[inline]
    pub fn insert_item(&mut self, node: &mut Node, index: usize, item: Node) -> bool {
        Self::process_node(node, &self.nodes, &self.text);
        if let Node::Array(items) = node {
            items.insert(index, item);
            true
        } else {
            false
        }
    }

    #[inline]
    pub fn remove_item(&mut self, node: &mut Node, index: usize) -> Option<Node> {
        Self::process_node(node, &self.nodes, &self.text);
        if let Node::Array(items) = node {
            Some(items.remove(index))
        } else {
            None
        }
    }

    #[inline]
    pub fn get_members<'a>(&'a self, node: &'a Node) -> Option<&[Node]> {
        match node {
            Node::RawObject(range) => Some(&self.nodes[range.0..range.1]),
            Node::Object(members) => Some(&members),
            _ => None,
        }
    }

    pub fn add_member(&mut self, node: &mut Node, key: &str, value: Node) -> bool {
        Self::process_node(node, &self.nodes, &self.text);
        if let Node::Object(members) = node {
            for i in (0..members.len()).step_by(2) {
                if let Some(member) = self.get_string(&members[i]) {
                    if key == member {
                        members[i + 1] = value;
                        return true;
                    }
                }
            }
            members.push(Node::String(key.into()));
            members.push(value);
            true
        } else {
            false
        }
    }

    pub fn remove_member(&mut self, node: &mut Node, key: &str) -> Option<(Node, Node)> {
        Self::process_node(node, &self.nodes, &self.text);
        if let Node::Object(members) = node {
            for i in (0..members.len()).step_by(2) {
                if let Some(member) = self.get_string(&members[i]) {
                    if key == member {
                        return Some((members.remove(i), members.remove(i + 1)));
                    }
                }
            }
        }
        None
    }

    #[inline]
    pub fn get_string<'a>(&'a self, node: &'a Node) -> Option<&str> {
        match node {
            Node::RawString(range) => Some(&self.text[range.0..range.1]),
            Node::String(value) => Some(&value),
            _ => None,
        }
    }

    #[inline]
    fn find_member<'a>(key: &str, members: &'a mut [Node], text: &String) -> Option<&'a mut Node> {
        for i in (0..members.len()).step_by(2) {
            if let Some(member) = Self::as_string(&members[i], text) {
                if key == member {
                    return Some(&mut members[i + 1]);
                }
            }
        }
        None
    }

    #[inline]
    fn as_string<'a>(node: &'a Node, text: &'a String) -> Option<&'a str> {
        match node {
            Node::RawString(range) => Some(&text[range.0..range.1]),
            Node::String(value) => Some(&value),
            _ => None,
        }
    }

    fn process_descent(node: &mut Node, nodes: &Vec<Node>, text: &String) {
        Self::process_node(node, nodes, text);
        match node {
            Node::Array(items) => {
                for i in 0..items.len() {
                    Self::process_descent(&mut items[i], nodes, text);
                }
            }
            Node::Object(members) => {
                for i in 0..members.len() {
                    Self::process_descent(&mut members[i], nodes, text);
                }
            }
            _ => {}
        }
    }

    fn process_node(node: &mut Node, nodes: &Vec<Node>, text: &String) {
        match node {
            Node::RawArray(range) => {
                *node = Node::Array(Vec::from(&nodes[range.0..range.1]));
            }
            Node::RawObject(range) => {
                *node = Node::Object(Vec::from(&nodes[range.0..range.1]));
            }
            Node::RawString(range) => {
                *node = Node::String(String::from(&text[range.0..range.1]));
            }
            _ => {}
        }
    }
}
