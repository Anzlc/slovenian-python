use std::{ collections::HashMap, hash::Hash, sync::Arc };

#[derive(Debug)]
pub struct Token {
    value: String,
    is_keyword: bool,
}

impl Token {
    fn new(value: String, is_keyword: bool) -> Token {
        Token { value, is_keyword }
    }

    fn get_new_value(&self, keywords: &HashMap<String, String>) -> String {
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

pub fn tokenize(program: &String, keywords: &HashMap<String, String>) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    // for c in program.chars() {
    //     if " ():\"'\n\r".contains(c) {
    //         tokens.push(
    //             Token::new(
    //                 buf.clone(),
    //                 keywords.keys().any(|x| x.to_string() == buf)
    //             )
    //         );
    //         buf.clear();
    //         tokens.push(Token::new(c.to_string(), false));
    //         continue;
    //     }
    //     buf.push(c);
    // }
    let mut buf: String = String::new();
    let mut pairs: HashMap<String, String> = HashMap::new();
    pairs.insert("(".to_string(), ")".to_string());
    pairs.insert("\"".to_string(), "\"".to_string());
    pairs.insert("'".to_string(), "'".to_string());
    pairs.insert("[".to_string(), "]".to_string());

    let single_tokens = ": \n\r.=";

    let mut i: usize = 0;
    println!();
    while i < program.chars().count() {
        let ch = program.chars().nth(i).unwrap();

        if pairs.contains_key(&ch.to_string()) {
            tokens.push(Token::new(buf.to_string(), keywords.contains_key(&buf)));
            buf.clear();
            let mut j = i;
            while
                j < program.len() &&
                &program.chars().nth(j).unwrap().to_string() != pairs.get(&ch.to_string()).unwrap()
            {
                buf.push(program.chars().nth(j).unwrap());
                j += 1;
            }
            buf.push_str(pairs.get(&ch.to_string()).unwrap().as_str());
            tokens.push(Token::new(buf.to_string(), keywords.contains_key(&buf)));
            buf.clear();
            i = j + 1;

            continue;
        } else if single_tokens.contains(ch) {
            tokens.push(Token::new(buf.to_string(), keywords.contains_key(&buf)));
            buf.clear();
            tokens.push(Token::new(ch.to_string(), keywords.contains_key(&ch.to_string())));
        } else {
            buf.push(ch);
        }
        i += 1;
    }
    return tokens;
}
pub fn transpile(tokens: &Vec<Token>, keywords: &HashMap<String, String>) -> String {
    let mut new_tokens = tokens
        .iter()
        .map(|token| token.get_new_value(&keywords))
        .collect::<Vec<String>>()
        .join("");

    new_tokens
}
