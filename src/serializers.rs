use crate::*;

pub trait Convert {
    fn convert(&self) -> Node;
}

pub trait Serializable: Convert + Sized {
    fn instantiate(node: &Node, tree: &Tree) -> Option<Self>;
    fn initialize(&mut self, node: &Node, tree: &Tree);
}

macro_rules! number {
    ($type: ident) => {
        impl Convert for $type {
            fn convert(&self) -> Node {
                Tree::number(*self as f64)
            }
        }
        impl Serializable for $type {
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

impl Convert for bool {
    fn convert(&self) -> Node {
        Tree::boolean(*self)
    }
}
impl Serializable for bool {
    fn instantiate(node: &Node, _: &Tree) -> Option<Self> {
        if let Node::Boolean(value) = node {
            Some(*value)
        } else {
            None
        }
    }
    fn initialize(&mut self, _: &Node, _: &Tree) {}
}

impl Convert for char {
    fn convert(&self) -> Node {
        Tree::number(*self as u8 as f64)
    }
}
impl Serializable for char {
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

impl Convert for String {
    fn convert(&self) -> Node {
        Tree::string(self)
    }
}
impl Serializable for String {
    fn instantiate(node: &Node, tree: &Tree) -> Option<Self> {
        tree.get_string(node).map(|value| value.to_string())
    }
    fn initialize(&mut self, _: &Node, _: &Tree) {}
}

impl Convert for &str {
    fn convert(&self) -> Node {
        Tree::string(self)
    }
}

impl<T: Convert> Convert for Option<T> {
    fn convert(&self) -> Node {
        if let Some(value) = self {
            value.convert()
        } else {
            Node::Null
        }
    }
}
impl<T: Serializable> Serializable for Option<T> {
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

impl<T: Convert> Convert for Vec<T> {
    fn convert(&self) -> Node {
        Tree::array(self.iter().map(|value| value.convert()).collect())
    }
}
impl<T: Serializable> Serializable for Vec<T> {
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
