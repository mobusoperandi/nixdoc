use maud::{html, PreEscaped, DOCTYPE};

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

    let output = document_module(example);
    std::fs::write("examples/output.html", &output).unwrap();
}

fn document_module(node: Node) -> String {
    html! {
        (DOCTYPE)
        html {
            head { title { "index.nix" } }
            body {
                p { "Evaluated using nix (Nix) 2.11.0 at 2022-10-12 10:47." }
                (document_node(node))
            }
        }
    }
    .0
}

fn document_node(node: Node) -> PreEscaped<String> {
    let (type_, contents): (String, PreEscaped<String>) = match node.value {
        Value::AttrSet(attrs) => ("{}".to_owned(), {
            let attrs = attrs.into_iter().map(document_attr);
            html! { ul { @for attr in attrs { (attr) } } }
        }),
        Value::Number(number) => (number.to_string(), html!()),
        Value::String(string) => (format!("\"{string}\""), html!()),
        Value::Function(function) => (function.param, html!()),
        Value::Boolean(value) => (value.to_string(), html!()),
    };

    html! {
        details open {
            @if let Some(doc) = node.doc {
                pre { code { (doc) } }
            }
            summary { code { (type_) } }
            (contents)
        }
    }
}

fn document_attr(Attr { ident, node }: Attr) -> PreEscaped<String> {
    html! {
        li {
            code { (ident) }
            br;
            (document_node(node))
        }
    }
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
    Boolean(bool),
}

struct Function {
    param: String,
    function: Option<Box<Function>>,
}
