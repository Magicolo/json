use crate::*;

pub trait Serializable: Sized {
    fn convert(&self) -> Node;
    fn instantiate<'a>(node: &'a Node, tree: &'a Tree) -> Option<Self>;
    fn initialize<'a>(&mut self, node: &'a Node, tree: &'a Tree);
}

macro_rules! number {
    ($type: ident) => {
        impl Serializable for $type {
            fn convert(&self) -> Node {
                Tree::number(*self as f64)
            }
            fn instantiate<'a>(node: &'a Node, _: &'a Tree) -> Option<Self> {
                if let Node::Number(value) = node {
                    Some(*value as $type)
                } else {
                    None
                }
            }
            fn initialize<'a>(&mut self, _: &'a Node, _: &'a Tree) {}
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
    fn convert(&self) -> Node {
        Tree::boolean(*self)
    }
    fn instantiate<'a>(node: &'a Node, _: &'a Tree) -> Option<Self> {
        if let Node::Boolean(value) = node {
            Some(*value)
        } else {
            None
        }
    }
    fn initialize<'a>(&mut self, _: &'a Node, _: &'a Tree) {}
}

impl Serializable for char {
    fn convert(&self) -> Node {
        Tree::number(*self as u8 as f64)
    }
    fn instantiate(node: &Node, tree: &Tree) -> Option<Self> {
        if let Node::Number(value) = node {
            Some(*value as u8 as char)
        } else if let Some(value) = tree.get_string(node) {
            value.chars().nth(0)
        } else {
            None
        }
    }
    fn initialize(&mut self, _: &Node, _: &Tree) {}
}

impl Serializable for String {
    fn convert(&self) -> Node {
        Tree::string(self)
    }
    fn instantiate(node: &Node, tree: &Tree) -> Option<Self> {
        tree.get_string(node).map(|value| value.to_string())
    }
    fn initialize(&mut self, _: &Node, _: &Tree) {}
}

impl<T: Serializable> Serializable for Option<T> {
    fn convert(&self) -> Node {
        if let Some(value) = self {
            value.convert()
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
    fn convert(&self) -> Node {
        Tree::array(self.iter().map(|value| value.convert()).collect())
    }
    fn instantiate(node: &Node, tree: &Tree) -> Option<Self> {
        if let Some(items) = tree.get_items(node) {
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
        if let Some(items) = tree.get_items(node) {
            for i in 0..self.len() {
                self[i].initialize(&items[i], tree);
            }
        }
    }
}
