// Module for printing TODOs in various formats

use serde;

[#derive(Serialize)]
struct PrintTodo<'a> {
    file: &'a str,
    kind: &'a str,
    line: usize,
    text: &'a str,
    ref: Vec<&str>,
}