use std::{ collections::HashMap, fs::read_to_string, fs::write };
mod transpiler;
mod file;
use file::write_and_empty;
use transpiler::{ tokenize, transpile };
use std::process::Command;
fn main() {
    let mut keywords: HashMap<String, String> = HashMap::new();
    keywords.insert("definiraj".to_string(), "def".to_string());
    keywords.insert("za".to_string(), "for".to_string());
    keywords.insert("v".to_string(), "in".to_string());
    keywords.insert("dokler".to_string(), "while".to_string());
    keywords.insert("vključi".to_string(), "import".to_string());
    keywords.insert("iz".to_string(), "from".to_string());
    keywords.insert("vrni".to_string(), "return".to_string());
    keywords.insert("natisni".to_string(), "print".to_string());
    keywords.insert("obseg".to_string(), "range".to_string());
    keywords.insert("kot".to_string(), "as".to_string());
    keywords.insert("vnos".to_string(), "input".to_string());

    let program = read_to_string("./src/test.spy").unwrap();
    let tokens = tokenize(&program, &keywords);
    println!("{:#?}", tokens);
    let new_program = transpile(&tokens, &keywords);
    write_and_empty("./src/test.py", &new_program).unwrap();

    println!("{}", new_program);
}
