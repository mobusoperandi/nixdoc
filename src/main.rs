fn main() {

}

struct Node {
    doc: String,
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

const v: Vec<()> = Vec::new();

