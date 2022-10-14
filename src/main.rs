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
    String(&'static str),
    Function,
    Bool,
}

use Value::*;

const EXAMPLE: Node = Node {
    doc: Some("Example!"),
    value: AttrSet(vec![Attr {
        ident: "hasAttrByPath",
        node: Node {
            doc: Some(
                r#"Return if an attribute from nested attribute set exists.
                Example:

                ```
                let
                  x = {
                    a = {
                      b = 3;
                    };
                  };
                in assert hasAttrByPath ["a" "b"] x; true
                ```"#,
            ),
            value: todo!(),
        },
    }]),
};
