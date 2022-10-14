fn main() {}

struct Node {
    doc: Option<&'static str>,
    value: Value,
}

struct Attr {
    ident: &'static str,
    node: Node,
}

enum Value {
    AttrSet(Vec<Attr>),
    Number,
    string(&'static str),
    Function,
    Bool,
}

const EXAMPLE: Node = Node {
    doc: Some(r#""#),
    value: todo!(),
};
