#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Object(Vec<Node>),
    Array(Vec<Node>),
    RawString((usize, usize)),
    RawObject((usize, usize)),
    RawArray((usize, usize)),
}

impl Node {
    #[inline]
    pub fn is_null(&self) -> bool {
        match self {
            Node::Null => true,
            _ => false,
        }
    }
    #[inline]
    pub fn is_boolean(&self) -> bool {
        match self {
            Node::Boolean(_) => true,
            _ => false,
        }
    }
    #[inline]
    pub fn is_number(&self) -> bool {
        match self {
            Node::Number(_) => true,
            _ => false,
        }
    }
    #[inline]
    pub fn is_object(&self) -> bool {
        match self {
            Node::Object(_) | Node::RawObject(_) => true,
            _ => false,
        }
    }
    #[inline]
    pub fn is_array(&self) -> bool {
        match self {
            Node::Array(_) | Node::RawArray(_) => true,
            _ => false,
        }
    }
    #[inline]
    pub fn is_string(&self) -> bool {
        match self {
            Node::String(_) | Node::RawString(_) => true,
            _ => false,
        }
    }
    #[inline]
    pub fn is_branch(&self) -> bool {
        self.is_array() || self.is_object()
    }
    #[inline]
    pub fn is_leaf(&self) -> bool {
        self.is_null() || self.is_boolean() || self.is_number() || self.is_string()
    }
}
