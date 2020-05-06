use crate::*;

pub fn convert<T: Serializable>(value: &T) -> Tree {
    Tree {
        root: value.convert(),
        nodes: Vec::new(),
        text: String::new(),
    }
}

impl<T: Serializable> From<&T> for Tree {
    fn from(value: &T) -> Self {
        convert(value)
    }
}

impl<T: Serializable> From<&T> for Node {
    fn from(value: &T) -> Self {
        value.convert()
    }
}
