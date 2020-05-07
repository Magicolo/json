use crate::*;

pub fn convert<T: Convert>(value: &T) -> Tree {
    Tree::new(value.convert())
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
