#[derive(Debug)]
pub struct Tree {
    pub root: Node,
    pub nodes: Vec<Node>,
    pub text: String,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Node {
    Null,
    Boolean(bool),
    Number(f64),
    String((usize, usize)),
    Object((usize, usize)),
    Array((usize, usize)),
}

impl Tree {
    pub fn items<'a>(&'a self, node: &'a Node) -> Option<&'a [Node]> {
        if let Node::Array(range) = node {
            Some(&self.nodes[range.0..range.1])
        } else {
            None
        }
    }

    pub fn item<'a>(&'a self, node: &'a Node, index: usize) -> Option<&'a Node> {
        self.items(node).and_then(|items| items.get(index))
    }

    // pub fn add_item<'a>(&'a self, node: &'a mut Node, index: usize, count: usize) {}
    // pub fn remove_item<'a>(&'a self, node: &'a mut Node, index: usize, count: usize) {}

    pub fn members<'a>(&'a self, node: &'a Node) -> Option<&'a [Node]> {
        if let Node::Object(range) = node {
            Some(&self.nodes[range.0..range.1])
        } else {
            None
        }
    }

    pub fn member<'a>(&'a self, node: &'a Node, member: &str) -> Option<&'a Node> {
        if let Some(members) = self.members(node) {
            for i in (0..members.len()).step_by(2) {
                if let Some(key) = self.string(&members[i]) {
                    if key == member {
                        return Some(&members[i + 1]);
                    }
                }
            }
            None
        } else {
            None
        }
    }

    pub fn string<'a>(&'a self, node: &'a Node) -> Option<&'a str> {
        if let Node::String(range) = node {
            Some(&self.text[range.0..range.1])
        } else {
            None
        }
    }
}
