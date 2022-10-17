fn main() {
    let example = Node {
        doc: Some("Example!".to_string()),
        value: Value::AttrSet(vec![
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
                    value: Value::Function(Function {
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
                    value: Value::String("x86_64-linux".to_string()),
                },
            },
            Attr {
                ident: "buildCores".to_string(),
                node: Node {
                    doc: Some("The amount of cores".to_string()),
                    value: Value::Number(5),
                },
            },
            Attr {
                ident: "utils".to_string(),
                node: Node {
                    doc: Some("Utilities".to_string()),
                    value: Value::AttrSet(vec![
                        Attr {
                            ident: "parse".to_string(),
                            node: Node {
                                doc: Some(
                                    r#"Convert a JSON string to a Nix value. For example,

                                    ```
                                    builtins.fromJSON ''{"x": [1, 2, 3], "y": null}''
                                    ```

                                    returns the value { x = [ 1 2 3 ]; y = null; }."#
                                        .to_string(),
                                ),
                                value: Value::Function(Function {
                                    param: "e".to_string(),
                                    function: None,
                                }),
                            },
                        },
                        Attr {
                            ident: "isGreaterThan3".to_string(),
                            node: Node {
                                doc: Some(
                                    "Returns true if a provided number is greater than 3."
                                        .to_string(),
                                ),
                                value: Value::Function(Function {
                                    param: "n".to_string(),
                                    function: None,
                                }),
                            },
                        },
                    ]),
                },
            },
        ]),
    };

    let output = generate(example);
    std::fs::write("examples/output.html", &output).unwrap();
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
    String(String),
    Function(Function),
    Boolean,
}

struct Function {
    param: String,
    function: Option<Box<Function>>,
}

