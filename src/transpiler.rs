use std::collections::HashMap;
use phf::phf_map;

static KEYWORDS: phf::Map<
    &'static str,
    &'static str
> = phf_map! {
    "definiraj" => "def",
    "za" => "for",
    "v" => "in",
    "dokler" => "while",
    "vključi" => "import",
    "iz" => "from",
    "vrni" => "return",
    "natisni" => "print",
    "obseg" => "range",
    "kot" => "as",
    "vnos" => "input",
    "če" => "if",
    "drugače" => "else",
    "drugačeče" => "elif",
    "in" => "and",
    "ali" => "or",
    "razred" => "class",
    "preveri" => "assert",
    "zlomi" => "break",
    "nadaljuj" => "continue",
    "izbriši" => "del",
    "razen" => "except",
    "Ne" => "False",
    "Da" => "True",
    "končno" => "finally",
    "globalno" => "global",
    "je" => "is",
    "Nič" => "None",
    "nelokalen" => "nonlocal",
    "ne" => "not",
    "brez" => "pass",
    "dvigni" => "raise",
    "poskusi" => "try",
    "z" => "with",
    "dajaj" => "yield",
};

#[derive(Debug)]
pub struct Token {
    value: String,
}

impl Token {
    fn new(value: String) -> Token {
        Token { value }
    }

    fn get_new_value(&self) -> String {
        KEYWORDS.get(self.value.as_str()).unwrap_or(&self.value.as_str()).to_string()
    }
}

pub fn tokenize(program: &String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    let mut buf: String = String::new();
    let mut pairs: HashMap<&str, &str> = HashMap::new();
    pairs.insert("\"", "\"");
    pairs.insert("'", "'");

    let single_tokens = ": \n\r=()[]\t";

    let mut chars = program.chars().peekable();
    while let Some(ch) = chars.next() {
        let tmp = ch.to_string();
        let ch = tmp.as_str();
        if let Some(&pair) = pairs.get(&ch) {
            if buf.len() > 0 {
                tokens.push(Token::new(buf.to_string()));
                buf.clear();
            }
            buf.push_str(ch);

            while let Some(&new_ch) = chars.peek() {
                buf.push(new_ch);
                chars.next();
                if new_ch.to_string().as_str() == pair {
                    break;
                }
            }
            tokens.push(Token::new(buf.to_string()));
            buf.clear();
            continue;
        } else if single_tokens.contains(ch) {
            if buf.len() > 0 {
                tokens.push(Token::new(buf.to_string()));
                buf.clear();
            }
            tokens.push(Token::new(ch.to_string()));
        } else {
            buf.push_str(ch);
        }
    }
    tokens
}
pub fn transpile(tokens: &Vec<Token>) -> String {
    tokens
        .iter()
        .map(|token| token.get_new_value())
        .collect::<Vec<String>>()
        .join("")
}
