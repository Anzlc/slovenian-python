use std::{ collections::HashMap, fs::read_to_string, fs::write };
mod transpiler;
mod file;
use file::write_and_empty;
use transpiler::{ tokenize, transpile };
use std::process::Command;
fn main() {
    let mut keywords: HashMap<&str, &str> = HashMap::new();
    keywords.insert("definiraj", "def");
    keywords.insert("za", "for");
    keywords.insert("v", "in");
    keywords.insert("dokler", "while");
    keywords.insert("vključi", "import");
    keywords.insert("iz", "from");
    keywords.insert("vrni", "return");
    keywords.insert("natisni", "print");
    keywords.insert("obseg", "range");
    keywords.insert("kot", "as");
    keywords.insert("vnos", "input");

    let program = read_to_string("./src/test.spy").unwrap();
    let tokens = tokenize(&program, &keywords);
    println!("{:#?}", tokens);
    let new_program = transpile(&tokens, &keywords);
    write_and_empty("./src/test.py", &new_program).unwrap();

    Command::new("python").arg("./src/test.py").spawn().expect("oh no");

    println!("{}", new_program);
}
