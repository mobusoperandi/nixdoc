use Value::*;
fn main() {
    let example = Node {
        doc: Some("Example!".to_string()),
        value: AttrSet(vec![
            Attr {
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
                    value: Func(Function {
                        param: "attrPath".to_string(),
                        function: Some(Box::new(Function {
                            param: "e".to_string(),
                            function: None,
                        })),
                    }),
                },
            },
            Attr {
                ident: "system".to_string(),
                node: Node {
                    doc: Some("The system".to_string()),
                    value: Str("x86_64-linux".to_string()),
                },
            },
            Attr {
                ident: "buildCores".to_string(),
                node: Node {
                    doc: Some("The amount of cores".to_string()),
                    value: Number(5),
                },
            },
            Attr {
                ident: "utils".to_string(),
                node: AttrSet(vec![Attr {
                    ident: "parse".to_string(),
                    node: Func(Function {
                        param: (),
                        function: (),
                    }),
                }]),
            },
        ]),
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
    Number(usize),
    Str(String),
    Func(Function),
    Bool,
}

struct Function {
    param: String,
    function: Option<Box<Function>>,
}
