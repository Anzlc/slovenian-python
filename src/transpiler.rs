use std::{ collections::HashMap, hash::Hash };

#[derive(Debug)]
pub struct Token {
    value: String,
    is_keyword: bool,
}

impl Token {
    fn new(value: String, is_keyword: bool) -> Token {
        Token { value, is_keyword }
    }

    fn get_new_value(&self, keywords: &HashMap<&str, &str>) -> String {
        if !self.is_keyword {
            return self.value.to_string();
        }
        match keywords.get(self.value.as_str()) {
            Some(x) => {
                return x.to_string();
            }
            None => {
                return self.value.to_string();
            }
        }
    }
}

pub fn tokenize(program: &String, keywords: &HashMap<&str, &str>) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut buf: String = String::new();
    for c in program.chars() {
        if " ():\"'".contains(c) {
            tokens.push(
                Token::new(
                    buf.clone(),
                    keywords.keys().any(|x| x.to_string() == buf)
                )
            );
            buf.clear();
            tokens.push(Token::new(c.to_string(), false));
            continue;
        }
        buf.push(c);
    }
    return tokens;
}

pub fn transpile(tokens: &Vec<Token>, keywords: &HashMap<&str, &str>) -> String {
    let mut new_tokens = tokens
        .iter()
        .map(|token| token.get_new_value(&keywords))
        .collect::<Vec<String>>()
        .join("");

    new_tokens
}
