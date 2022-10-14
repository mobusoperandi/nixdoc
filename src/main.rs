fn main() {
    let example = Node {
        doc: Some("Example!".to_string()),
        value: Value::AttrSet(vec![Attr {
            ident: "hasAttrByPath".to_string(),
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
                    ```"#
                        .to_string(),
                ),
                value: Value::Function(Function),
            },
        }]),
    };
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
    Str(String),
    Function(Function),
    Bool,
}

struct Function {
    param: 
}
