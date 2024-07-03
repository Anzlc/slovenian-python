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
    is_keyword: bool,
}

impl Token {
    fn new(value: String, is_keyword: bool) -> Token {
        Token { value, is_keyword }
    }
    fn from_string(value: &String) -> Token {
        let tmp = value.clone();
        Token::new(value.to_string(), KEYWORDS.contains_key(tmp.as_str()))
    }
    fn get_new_value(&self) -> String {
        if !self.is_keyword {
            return self.value.to_string();
        }
        match KEYWORDS.get(self.value.as_str()) {
            Some(x) => {
                return x.to_string();
            }
            None => {
                return self.value.to_string();
            }
        }
    }
}

pub fn tokenize(program: &String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    let mut buf: String = String::new();
    let mut pairs: HashMap<&str, &str> = HashMap::new();
    pairs.insert("\"", "\"");
    pairs.insert("'", "'");

    let single_tokens = ": \n\r.=()[]\t";

    let mut i: usize = 0;
    while i < program.chars().count() {
        let tmp = program.chars().nth(i).unwrap().to_string();
        let ch = tmp.as_str();

        if pairs.contains_key(ch) {
            if buf.len() > 0 {
                tokens.push(Token::from_string(&buf.to_string()));
                buf.clear();
            }
            let pair = pairs.get(ch).unwrap(); // Can safely unwrap because we check it before
            buf.push_str(ch);
            let mut j = i + 1;

            while j < program.len() && &program.chars().nth(j).unwrap().to_string() != pair {
                buf.push_str(&program.chars().nth(j).unwrap().to_string());
                j += 1;
            }
            buf.push_str(pairs.get(ch).unwrap());
            tokens.push(Token::from_string(&buf.to_string()));
            buf.clear();
            i = j + 1;

            continue;
        } else if single_tokens.contains(ch) {
            if buf.len() > 0 {
                tokens.push(Token::from_string(&buf.to_string()));
                buf.clear();
            }
            tokens.push(Token::from_string(&ch.to_string()));
        } else {
            buf.push_str(ch);
        }
        i += 1;
    }
    return tokens;
}
pub fn transpile(tokens: &Vec<Token>) -> String {
    let new_tokens = tokens
        .iter()
        .map(|token| token.get_new_value())
        .collect::<Vec<String>>()
        .join("");

    new_tokens
}
