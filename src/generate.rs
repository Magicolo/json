use crate::*;

pub fn generate(tree: &Tree) -> String {
    let mut buffer = String::with_capacity(256);
    descend(&tree.root, tree, &mut buffer);
    buffer
}

#[inline]
fn hex(value: u16) -> char {
    match value & 0xF {
        0 => '0',
        1 => '1',
        2 => '2',
        3 => '3',
        4 => '4',
        5 => '5',
        6 => '6',
        7 => '7',
        8 => '8',
        9 => '9',
        10 => 'A',
        11 => 'B',
        12 => 'C',
        13 => 'D',
        14 => 'E',
        15 => 'F',
        _ => '\0',
    }
}

fn descend(node: &Node, tree: &Tree, buffer: &mut String) {
    match node {
        Node::Null => buffer.push_str("null"),
        Node::Number(value) => buffer.push_str(&value.to_string()),
        Node::Boolean(value) => buffer.push_str(if *value { "true" } else { "false" }),
        _ => {
            if let Some(value) = tree.get_string(node) {
                buffer.push('"');

                for character in value.chars() {
                    match character {
                        '\n' => buffer.push_str(r"\n"),
                        '\r' => buffer.push_str(r"\r"),
                        '\t' => buffer.push_str(r"\t"),
                        '\u{0008}' => buffer.push_str(r"\f"),
                        '\u{000C}' => buffer.push_str(r"\b"),
                        '\"' => buffer.push_str(r#"\""#),
                        '\\' => buffer.push_str(r"\\"),
                        _ => {
                            if character.is_ascii() {
                                buffer.push(character);
                            } else {
                                buffer.push_str(r"\u");
                                buffer.push(hex(character as u16 >> 12));
                                buffer.push(hex(character as u16 >> 8));
                                buffer.push(hex(character as u16 >> 4));
                                buffer.push(hex(character as u16));
                            }
                        }
                    }
                }
                buffer.push('"');
            } else if let Some(items) = tree.get_items(node) {
                buffer.push('[');
                for i in 0..items.len() {
                    if i > 0 {
                        buffer.push(',')
                    }
                    descend(&items[i], tree, buffer);
                }
                buffer.push(']');
            } else if let Some(members) = tree.get_members(node) {
                buffer.push('{');
                for i in 0..members.len() {
                    if i > 0 {
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
