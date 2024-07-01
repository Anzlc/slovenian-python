use std::{ collections::HashMap, fs::read_to_string };
mod transpiler;
use transpiler::{ tokenize, transpile };
fn main() {
    let mut keywords: HashMap<&str, &str> = HashMap::new();
    keywords.insert("definiraj", "def");
    keywords.insert("za", "for");
    keywords.insert("v", "in");
    keywords.insert("dokler", "while");
    keywords.insert("vključi", "import");
    keywords.insert("iz", "from");

    let program = read_to_string("./src/test.py").unwrap();
    let tokens = tokenize(&program, &keywords);
    println!("{:#?}", tokens);
    let new_program = transpile(&tokens, &keywords);
    println!("{}", new_program);
}
