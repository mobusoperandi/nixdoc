fn main() {

}

struct Node {
    doc: Option<String>,
    value: Value,
}

struct Attr {
    ident: String,
    node: Node,
}

enum Value {
    AttrSet(Vec<Attr>),
    Number,
    String,
    Function,
    Bool,
}

const EXAMPLE: Node = Node {
    doc: todo!(),
    value: todo!(),
};
