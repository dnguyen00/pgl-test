use std::collections::HashMap;
use crate::tokens::Tokens;

#[derive(Debug, Clone)]
pub struct Lexer<'a> {
    to_lex: &'a str,
    position: i64,
    tokens_chars: HashMap<Tokens, &'a str>
}

impl Lexer<'_> {
    pub fn new(to_lex: &str) -> Lexer {
        let mut tokens: HashMap<Tokens, &str> = HashMap::new();
        tokens.insert(Tokens::ADD, "+");
        tokens.insert(Tokens::SUB, "-");
        tokens.insert(Tokens::MUL, "*");
        tokens.insert(Tokens::DIV, "/");
        tokens.insert(Tokens::MOD, "%");
        tokens.insert(Tokens::ADDASSIGNMENT, "+=");
        tokens.insert(Tokens::SUBASSIGNMENT, "-=");
        tokens.insert(Tokens::MULASSIGNMENT, "*=");
        tokens.insert(Tokens::DIVASSIGNMENT, "/=");
        tokens.insert(Tokens::MODASSIGNMENT, "%=");
        tokens.insert(Tokens::LPARENTHESIS, "(");
        tokens.insert(Tokens::RPARENTHESIS, ")");
        tokens.insert(Tokens::EQUALS, "=");
        tokens.insert(Tokens::LESS, "<");
        tokens.insert(Tokens::LESSEQ, "<=");
        tokens.insert(Tokens::GREAT, ">");
        tokens.insert(Tokens::GREATEQ, ">=");
        tokens.insert(Tokens::AND, "&&");
        tokens.insert(Tokens::OR, "||");
        tokens.insert(Tokens::SEMICOLON, ";");
        tokens.insert(Tokens::LBRACKET, "{");
        tokens.insert(Tokens::RBRACKET, "}");
        tokens.insert(Tokens::EQUALITY, "==");
        tokens.insert(Tokens::INEQUALITY, "!=");
        tokens.insert(Tokens::COMMA, ",");

        return Lexer { to_lex: to_lex, position: 0, tokens_chars: tokens}
    }

    fn peek_next_char(&self) -> Option<char> {
        return self.to_lex.chars().nth(self.position as usize + 1);
    }

    fn get_current_char(&self) -> Option<char> {
        return self.to_lex.chars().nth(self.position as usize);
    }

    fn get_next_char(&mut self) -> Option<char> {
        self.position += 1;
        return self.to_lex.chars().nth(self.position as usize);
    }

    pub fn next_token(&mut self) -> Option<Lexemes> {
        let mut lexeme: String = String::new();
        let mut current_char = self.get_current_char();
        let starting_position: u32 = self.position as u32;

        if current_char == None {
            return None;
        }

        while current_char == Some(' ') || current_char == Some('\n') {
            if self.peek_next_char().is_some() {
                current_char = self.get_next_char();
            }
        }

        lexeme.push(current_char.unwrap());
        for (k, v) in &self.tokens_chars {
            if *v == lexeme.as_str() {
                if !self.peek_next_char().is_some() {
                    self.position += 1;
                    return Some(Lexemes { token: *k, lexeme: lexeme, position: [starting_position, self.position as u32 - 1] });
                }

                let special_tokens = [[Tokens::GREAT, Tokens::GREATEQ], 
                [Tokens::LESS, Tokens::LESSEQ], [Tokens::ADD, Tokens::ADDASSIGNMENT], [Tokens::SUB, Tokens::SUBASSIGNMENT],
                [Tokens::MUL, Tokens::MULASSIGNMENT], [Tokens::DIV, Tokens::DIVASSIGNMENT], [Tokens::MOD, Tokens::MODASSIGNMENT],
                [Tokens::EQUALS, Tokens::EQUALITY]];
                
                for pairs in special_tokens {
                    if pairs.get(0).unwrap() == k {
                        if self.peek_next_char() == self.tokens_chars.get(pairs.get(1).unwrap()).unwrap().chars().last() {
                            current_char = self.get_next_char();
                            lexeme.push(current_char.unwrap());
                            self.position += 1;
                            return Some(Lexemes { token: *pairs.get(1).unwrap(), lexeme: lexeme, position: [starting_position, self.position as u32 - 1] });
                        }

                        self.position += 1;
                        return Some(Lexemes { token: *k, lexeme: lexeme, position: [starting_position, self.position as u32 - 1] })
                    }
                }

                self.position += 1;
                return Some(Lexemes { token: *k, lexeme: lexeme, position: [starting_position, self.position as u32 - 1] })
            }
            
            let literal_tokens = [Tokens::AND, Tokens::OR, Tokens::INEQUALITY];
            for l_tokens in literal_tokens {
                if lexeme.chars().nth(0) == self.tokens_chars.get(&l_tokens).unwrap().chars().nth(0) {
                    if self.peek_next_char() == self.tokens_chars.get(&l_tokens).unwrap().chars().nth(1) {
                        current_char = self.get_next_char();
                        lexeme.push(current_char.unwrap());
                        self.position += 1;
                        return Some(Lexemes { token: l_tokens, lexeme: lexeme, position: [starting_position, self.position as u32 - 1] });
                    }
                }
            }
        }

        if current_char.unwrap().is_numeric() {
            let mut is_float = false;
            while self.peek_next_char().is_some() {
                if self.peek_next_char().unwrap().is_numeric() {
                    current_char = self.get_next_char();
                    lexeme.push(current_char.unwrap());
                } else if self.peek_next_char() == Some('.') {
                    current_char = self.get_next_char();
                    lexeme.push(current_char.unwrap());
                    is_float = true;
                } else { break; }
            }

            self.position += 1;
            if is_float { return Some(Lexemes { token: Tokens::FLOATS, lexeme: lexeme, position: [starting_position, self.position as u32 - 1] }); }
            return Some(Lexemes { token: Tokens::INTEGERS, lexeme: lexeme, position: [starting_position, self.position as u32 - 1] });
        }

        if current_char.unwrap().is_alphabetic() {
            while self.peek_next_char().is_some() {
                if self.peek_next_char().unwrap().is_alphanumeric() {
                    current_char = self.get_next_char();
                    lexeme.push(current_char.unwrap());
                } else { break; };
            }

            self.position += 1;
            return Some(Lexemes { token: Tokens::IDENTIFIER, lexeme: lexeme, position: [starting_position, self.position as u32 - 1] });
        }

        self.position += 1;
        let current_token: Option<Lexemes> = Some(Lexemes { token: Tokens::UNKNOWN, lexeme: lexeme, position: [starting_position, self.position as u32 - 1] });
        return current_token;
    }

    pub fn peek_token(&mut self) -> Option<Lexemes> {
        let next_token = self.next_token();
        if next_token == None {
            return None;
        }

        self.position -= *next_token.clone().unwrap().position.get(1).unwrap() as i64 - *next_token.clone().unwrap().position.get(0).unwrap() as i64 + 1;
        return next_token;
    }

    pub fn parse(&mut self) {
        loop {
            let lexeme = self.next_token();
            if lexeme == None { break; }

            println!("{:?}", lexeme.unwrap());
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Lexemes {
    pub token: Tokens,
    pub lexeme: String,
    pub position: [u32;2]
}