use crate::*;

pub fn convert<T: Serializable>(value: &T) -> Tree {
    let mut tree = Tree {
        root: Node::Null,
        nodes: Vec::with_capacity(32),
        text: String::with_capacity(32),
    };
    let root = value.convert(&mut tree);
    tree.root = root;
    tree
}

impl<T: Serializable> From<&T> for Tree {
    fn from(value: &T) -> Self {
        convert(value)
    }
}
