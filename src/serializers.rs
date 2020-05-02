use crate::*;

pub trait Serializable: Sized {
    fn convert(&self, tree: &mut Tree) -> Node;
    fn instantiate(node: &Node, tree: &Tree) -> Option<Self>;
    fn initialize(&mut self, node: &Node, tree: &Tree);
}

macro_rules! number {
    ($type: ident) => {
        impl Serializable for $type {
            fn convert(&self, _: &mut Tree) -> Node {
                Node::Number(*self as f64)
            }
            fn instantiate(node: &Node, _: &Tree) -> Option<Self> {
                if let Node::Number(value) = node {
                    Some(*value as $type)
                } else {
                    None
                }
            }
            fn initialize(&mut self, _: &Node, _: &Tree) {}
        }
    };
}

number!(isize);
number!(usize);
number!(i8);
number!(i16);
number!(i32);
number!(i64);
number!(i128);
number!(u8);
number!(u16);
number!(u32);
number!(u64);
number!(u128);
number!(f32);
number!(f64);

impl Serializable for bool {
    fn convert(&self, _: &mut Tree) -> Node {
        Node::Boolean(*self)
    }
    fn instantiate(node: &Node, _: &Tree) -> Option<Self> {
        if let Node::Boolean(value) = node {
            Some(*value)
        } else {
            None
        }
    }
    fn initialize(&mut self, _: &Node, _: &Tree) {}
}

impl Serializable for char {
    fn convert(&self, _: &mut Tree) -> Node {
        Node::Number(*self as u8 as f64)
    }
    fn instantiate(node: &Node, tree: &Tree) -> Option<Self> {
        match node {
            Node::Number(value) => Some(*value as u8 as char),
            Node::String(range) => tree.text[range.0..range.1].chars().nth(0),
            _ => None,
        }
    }
    fn initialize(&mut self, _: &Node, _: &Tree) {}
}

impl Serializable for String {
    fn convert(&self, tree: &mut Tree) -> Node {
        let start = tree.text.len();
        tree.text.push_str(&self);
        Node::String((start, tree.text.len()))
    }
    fn instantiate(node: &Node, tree: &Tree) -> Option<Self> {
        if let Node::String(range) = node {
            Some(tree.text[range.0..range.1].to_string())
        } else {
            None
        }
    }
    fn initialize(&mut self, _: &Node, _: &Tree) {}
}

impl<T: Serializable> Serializable for Option<T> {
    fn convert(&self, tree: &mut Tree) -> Node {
        if let Some(value) = self {
            value.convert(tree)
        } else {
            Node::Null
        }
    }
    fn instantiate(node: &Node, tree: &Tree) -> Option<Self> {
        if let Node::Null = node {
            Some(None)
        } else {
            Some(Serializable::instantiate(node, tree))
        }
    }
    fn initialize(&mut self, node: &Node, tree: &Tree) {
        if let Some(value) = self {
            value.initialize(node, tree)
        }
    }
}

impl<T: Serializable> Serializable for Vec<T> {
    fn convert(&self, tree: &mut Tree) -> Node {
        let start = tree.nodes.len();
        for value in self {
            let node = value.convert(tree);
            tree.nodes.push(node);
        }
        Node::Array((start, tree.nodes.len()))
    }
    fn instantiate(node: &Node, tree: &Tree) -> Option<Self> {
        if let Some(items) = tree.items(node) {
            let mut values = Vec::with_capacity(items.len());
            for item in items {
                if let Some(value) = Serializable::instantiate(item, tree) {
                    values.push(value);
                } else {
                    return None;
                }
            }
            Some(values)
        } else {
            None
        }
    }
    fn initialize(&mut self, node: &Node, tree: &Tree) {
        if let Some(items) = tree.items(node) {
            for i in 0..self.len() {
                self[i].initialize(&items[i], tree);
            }
        }
    }
}
