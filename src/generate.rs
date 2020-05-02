use crate::*;

pub fn generate(tree: &Tree) -> String {
    let mut buffer = String::with_capacity(256);
    descend(&tree.root, tree, &mut buffer);
    buffer
}

fn descend(node: &Node, tree: &Tree, buffer: &mut String) {
    match node {
        Node::Null => buffer.push_str("null"),
        Node::Number(value) => buffer.push_str(&value.to_string()),
        Node::Boolean(value) => buffer.push_str(if *value { "true" } else { "false" }),
        Node::String(range) => buffer.push_str(&tree.text[range.0..range.1]),
        Node::Array(range) => {
            let items = &tree.nodes[range.0..range.1];
            if items.len() == 0 {
                buffer.push_str("[]");
            } else {
                buffer.push('[');
                let mut first = false;
                for item in items {
                    if first {
                        first = false
                    } else {
                        buffer.push(',')
                    }
                    descend(item, tree, buffer);
                }
                buffer.push(']');
            }
        }
        Node::Object(range) => {
            let members = &tree.nodes[range.0..range.1];
            if members.len() == 0 {
                buffer.push_str("{}");
            } else {
                buffer.push('{');
                let mut first = false;
                for i in 0..members.len() {
                    if first {
                        first = false
                    } else {
                        buffer.push(',')
                    }
                    descend(&members[i], tree, buffer);
                    buffer.push(':');
                    descend(&members[i + 1], tree, buffer);
                }
                buffer.push('}');
            }
        }
    }
}
