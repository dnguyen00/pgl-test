use crate::{lexer::{Lexer}, tokens::Tokens};

#[derive(Debug)]
pub struct Syntax<'a> {
    lexer: Lexer<'a>,
}

impl Syntax<'_> {
    pub fn new(lexer: Lexer) -> Syntax {
        return Syntax { lexer: lexer };
    }

    pub fn check_validity(&mut self) -> bool {
        if self.lexer.peek_token() == None {
            return false;
        }

        let mut statements: Vec<Option<Grammar>> = Vec::new();

        loop {
            if self.lexer.peek_token() == None {
                break;
            }

            statements.push(self.parse_stmt());
            
            if statements.last().unwrap() != &Some(Grammar::STMT) {
                return false;
            }

            println!("{:?}", statements.last().unwrap());
        }

        return true;
    }

    fn parse_stmt_list(&mut self) -> Option<Grammar> {
        if self.lexer.peek_token().clone() == None {
            return Some(Grammar::STMTLIST);
        }

        let mut original_lexer = self.lexer.clone();

        loop {
            let mut is_set = false;
            if self.parse_stmt() == Some(Grammar::STMT) {
                if self.lexer.peek_token().is_some() {
                    if self.lexer.next_token().unwrap().token == Tokens::SEMICOLON {
                        original_lexer = self.lexer.clone();
                        is_set = true;
                    } else {
                        return Some(Grammar::UNKNOWN);
                    }
                }
            } else { self.lexer = original_lexer.clone(); }

            if !is_set {
                break;
            }
        }
        
        return Some(Grammar::STMTLIST);
    }

    fn parse_stmt(&mut self) -> Option<Grammar> {
        let original_lexer = self.lexer.clone();

        if self.parse_if_stmt() == Some(Grammar::IFSTATEMENT) {
            return Some(Grammar::STMT);
        }

        self.lexer = original_lexer.clone();

        if self.parse_while_stmt() == Some(Grammar::WHILESTATEMENT) {
            return Some(Grammar::STMT);
        }

        self.lexer = original_lexer.clone();

        if self.parse_assign() == Some(Grammar::ASSIGN) {
            return Some(Grammar::STMT);
        }

        self.lexer = original_lexer.clone();

        if self.parse_declare() == Some(Grammar::DECLARE) {
            return Some(Grammar::STMT);
        }

        self.lexer = original_lexer.clone();
        
        if self.parse_block() == Some(Grammar::BLOCK) {
            return Some(Grammar::STMT);
        }

        self.lexer = original_lexer.clone();

        if self.parse_expr() == Some(Grammar::EXPRESSION) {
            return Some(Grammar::STMT);
        }

        return Some(Grammar::UNKNOWN);
    }

    fn parse_while_stmt(&mut self) -> Option<Grammar> {
        if self.lexer.peek_token().unwrap().clone().token != Tokens::IDENTIFIER {
            return Some(Grammar::UNKNOWN);
        }

        if self.lexer.peek_token().unwrap().clone().lexeme.as_str() != "while" {
            return Some(Grammar::UNKNOWN);
        }

        self.lexer.next_token();

        if self.lexer.peek_token() == None {
            return Some(Grammar::UNKNOWN);
        }

        if self.lexer.peek_token().clone().unwrap().token != Tokens::LPARENTHESIS {
            return Some(Grammar::UNKNOWN);
        }
        self.lexer.next_token();

        if self.parse_bool_expr() != Some(Grammar::BOOLEXPRESSION) {
            return Some(Grammar::UNKNOWN);
        }

        if self.lexer.peek_token() == None {
            return Some(Grammar::UNKNOWN);
        }

        if self.lexer.peek_token().clone().unwrap().token != Tokens::RPARENTHESIS {
            return Some(Grammar::UNKNOWN);
        }
        self.lexer.next_token();

        let original_lexer = self.lexer.clone();
        if self.parse_stmt() == Some(Grammar::STMT) {
            if self.lexer.peek_token().is_some() {
                if self.lexer.next_token().unwrap().token == Tokens::SEMICOLON {
                    return Some(Grammar::WHILESTATEMENT);
                }
            }
        }

        self.lexer = original_lexer;

        if self.parse_block() == Some(Grammar::BLOCK) {
            return Some(Grammar::WHILESTATEMENT);
        }

        return Some(Grammar::UNKNOWN);
    }

    fn parse_assign(&mut self) -> Option<Grammar> {
        if self.lexer.peek_token().is_some() && self.lexer.peek_token().unwrap().token != Tokens::IDENTIFIER {
            return Some(Grammar::UNKNOWN);
        }

        self.lexer.next_token();

        if self.lexer.peek_token().is_some() && self.lexer.peek_token().unwrap().token != Tokens::IDENTIFIER && self.lexer.peek_token().unwrap().lexeme != "=" {
            return Some(Grammar::UNKNOWN);
        }

        self.lexer.next_token();

        if self.parse_expr() != Some(Grammar::EXPRESSION) {
            return Some(Grammar::UNKNOWN);
        }
        
        return Some(Grammar::ASSIGN);
    }

    fn parse_declare(&mut self) -> Option<Grammar> {
        if self.lexer.peek_token().is_some() && self.lexer.peek_token().unwrap().token != Tokens::IDENTIFIER && self.lexer.peek_token().unwrap().lexeme != "DataType" {
            return Some(Grammar::UNKNOWN);
        }

        self.lexer.next_token();

        if self.lexer.peek_token().is_some() && self.lexer.peek_token().unwrap().token != Tokens::IDENTIFIER {
            return Some(Grammar::UNKNOWN);
        }

        self.lexer.next_token();

        loop {
            let peek_token = self.lexer.peek_token().clone();
            if peek_token == None {
                break;
            }

            if peek_token.clone().unwrap().token == Tokens::IDENTIFIER && peek_token.unwrap().lexeme == "," {
                self.lexer.next_token();

                if self.lexer.peek_token().is_some() && self.lexer.peek_token().unwrap().token == Tokens::IDENTIFIER {
                    self.lexer.next_token();
                } else { return Some(Grammar::UNKNOWN); }
            } else { break; }
        }

        return Some(Grammar::DECLARE);
    }

    fn parse_if_stmt(&mut self) -> Option<Grammar> {
        if self.lexer.peek_token().unwrap().clone().token != Tokens::IDENTIFIER {
            return Some(Grammar::UNKNOWN);
        }

        if self.lexer.peek_token().unwrap().clone().lexeme.as_str() != "if" {
            return Some(Grammar::UNKNOWN);
        }

        self.lexer.next_token();

        if self.lexer.peek_token() == None {
            return Some(Grammar::UNKNOWN);
        }

        if self.lexer.peek_token().clone().unwrap().token != Tokens::LPARENTHESIS {
            return Some(Grammar::UNKNOWN);
        }
        self.lexer.next_token();

        if self.parse_bool_expr() != Some(Grammar::BOOLEXPRESSION) {
            return Some(Grammar::UNKNOWN);
        }

        if self.lexer.peek_token() == None {
            return Some(Grammar::UNKNOWN);
        }

        if self.lexer.peek_token().clone().unwrap().token != Tokens::RPARENTHESIS {
            return Some(Grammar::UNKNOWN);
        }
        self.lexer.next_token();

        let original_lexer = self.lexer.clone();
        if self.parse_stmt() == Some(Grammar::STMT) {
            if self.lexer.peek_token().is_some() {
                if self.lexer.next_token().unwrap().token == Tokens::SEMICOLON {
                    return Some(Grammar::IFSTATEMENT);
                }
            }
        }

        self.lexer = original_lexer;

        if self.parse_block() == Some(Grammar::BLOCK) {
            return Some(Grammar::IFSTATEMENT);
        }

        if self.lexer.peek_token().is_some() {
            if self.lexer.peek_token().unwrap().token == Tokens::IDENTIFIER && self.lexer.peek_token().unwrap().lexeme == "else" {
                self.lexer.next_token();
                let original_lexer = self.lexer.clone();
                if self.parse_stmt() == Some(Grammar::STMT) {
                    if self.lexer.peek_token().is_some() {
                        if self.lexer.next_token().unwrap().token == Tokens::SEMICOLON {
                            return Some(Grammar::IFSTATEMENT);
                        }
                    }
                }
        
                self.lexer = original_lexer;
        
                if self.parse_block() == Some(Grammar::BLOCK) {
                    return Some(Grammar::IFSTATEMENT);
                }
            }
        }

        return Some(Grammar::UNKNOWN);
    }

    fn parse_block(&mut self) -> Option<Grammar> {
        if self.lexer.peek_token().clone() == None {
            return Some(Grammar::UNKNOWN);
        }

        if self.lexer.next_token().clone().unwrap().token != Tokens::LBRACKET {
            return Some(Grammar::UNKNOWN);
        }

        if self.parse_stmt_list() != Some(Grammar::STMTLIST) {
            return Some(Grammar::UNKNOWN);
        }

        if self.lexer.peek_token().clone() == None {
            return Some(Grammar::UNKNOWN);
        }

        if self.lexer.next_token().clone().unwrap().token != Tokens::RBRACKET {
            return Some(Grammar::UNKNOWN);
        }

        return Some(Grammar::BLOCK);
    }

    fn parse_bool_expr(&mut self) -> Option<Grammar> {
        if self.parse_bterm() != Some(Grammar::BTERM) {
            return Some(Grammar::UNKNOWN);
        }

        if self.lexer.peek_token().clone() == None {
            return Some(Grammar::BOOLEXPRESSION);
        }

        loop {
            let peek_token = self.lexer.peek_token().clone();
            if peek_token == None {
                break;
            }

            let peek_token = self.lexer.peek_token().clone().unwrap();
            
            let match_terminal_token = [Tokens::GREAT, Tokens::LESS, Tokens::GREATEQ, Tokens::LESSEQ];
            let mut is_set = false;
            for token in match_terminal_token {
                if peek_token.token == token {
                    self.lexer.next_token();
                    
                    if self.parse_bterm() != Some(Grammar::BTERM) {
                        return Some(Grammar::UNKNOWN);
                    }
    
                    is_set = true;
                    break;
                }
            }

            if !is_set {
                break;
            }
        }
        
        return Some(Grammar::BOOLEXPRESSION);
    }

    fn parse_bterm(&mut self) -> Option<Grammar> {
        if self.parse_band() != Some(Grammar::BAND) {
            return Some(Grammar::UNKNOWN);
        }

        if self.lexer.peek_token().clone() == None {
            return Some(Grammar::BTERM);
        }

        loop {
            let peek_token = self.lexer.peek_token().clone();
            if peek_token == None {
                break;
            }

            let peek_token = self.lexer.peek_token().clone().unwrap();

            let match_terminal_token = [Tokens::EQUALITY, Tokens::INEQUALITY];
            let mut is_set = false;
            for token in match_terminal_token {
                if peek_token.token == token {
                    self.lexer.next_token();
                    
                    if self.parse_band() != Some(Grammar::BAND) {
                        return Some(Grammar::UNKNOWN);
                    }
    
                    is_set = true;
                    break;
                }
            }

            if !is_set {
                break;
            }
        }
        
        return Some(Grammar::BTERM);
    }

    fn parse_band(&mut self) -> Option<Grammar> {
        if self.parse_bor() != Some(Grammar::BOR) {
            return Some(Grammar::UNKNOWN);
        }

        if self.lexer.peek_token().clone() == None {
            return Some(Grammar::BAND);
        }

        loop {
            let peek_token = self.lexer.peek_token().clone();
            if peek_token == None {
                break;
            }

            let peek_token = self.lexer.peek_token().clone().unwrap();

            let match_terminal_token = [Tokens::AND];
            let mut is_set = false;
            for token in match_terminal_token {
                if peek_token.token == token {
                    self.lexer.next_token();
                    
                    if self.parse_bor() != Some(Grammar::BOR) {
                        return Some(Grammar::UNKNOWN);
                    }
    
                    is_set = true;
                    break;
                }
            }

            if !is_set {
                break;
            }
        }
        
        return Some(Grammar::BAND);
    }

    fn parse_bor(&mut self) -> Option<Grammar> {
        if self.parse_expr() != Some(Grammar::EXPRESSION) {
            return Some(Grammar::UNKNOWN);
        }

        if self.lexer.peek_token().clone() == None {
            return Some(Grammar::BOR);
        }

        loop {
            let peek_token = self.lexer.peek_token().clone();
            if peek_token == None {
                break;
            }

            let peek_token = self.lexer.peek_token().clone().unwrap();
            
            let match_terminal_token = [Tokens::OR];
            let mut is_set = false;
            for token in match_terminal_token {
                if peek_token.token == token {
                    self.lexer.next_token();
                    
                    if self.parse_expr() != Some(Grammar::EXPRESSION) {
                        return Some(Grammar::UNKNOWN);
                    }
    
                    is_set = true;
                    break;
                }
            }

            if !is_set {
                break;
            }
        }
        
        return Some(Grammar::BOR);
    }

    fn parse_expr(&mut self) -> Option<Grammar> {
        if self.parse_term() != Some(Grammar::TERM) {
            return Some(Grammar::UNKNOWN);
        }

        if self.lexer.peek_token().clone() == None {
            return Some(Grammar::EXPRESSION);
        }

        loop {
            let peek_token = self.lexer.peek_token().clone();
            if peek_token == None {
                break;
            }

            let peek_token = self.lexer.peek_token().clone().unwrap();
            
            let match_terminal_token = [Tokens::ADD, Tokens::SUB];
            let mut is_set = false;
            for token in match_terminal_token {
                if peek_token.token == token {
                    self.lexer.next_token();
                    
                    if self.parse_term() != Some(Grammar::TERM) {
                        return Some(Grammar::UNKNOWN);
                    }
    
                    is_set = true;
                    break;
                }
            }

            if !is_set {
                break;
            }
        }

        return Some(Grammar::EXPRESSION);
    }

    fn parse_term(&mut self) -> Option<Grammar> {
        if self.parse_fact() != Some(Grammar::FACT) {
            return Some(Grammar::UNKNOWN);
        }

        if self.lexer.peek_token().clone() == None {
            return Some(Grammar::TERM);
        }

        loop {
            let peek_token = self.lexer.peek_token().clone();
            if peek_token == None {
                break;
            }

            let peek_token = self.lexer.peek_token().clone().unwrap();

            let match_terminal_token = [Tokens::MUL, Tokens::DIV, Tokens::MOD];
            let mut is_set = false;
            for token in match_terminal_token {
                if peek_token.token == token {
                    self.lexer.next_token();
    
                    if self.parse_fact() != Some(Grammar::FACT) {
                        return Some(Grammar::UNKNOWN);
                    }
    
                    is_set = true;
                    break;
                }
            }

            if !is_set {
                break;
            }
        }

        return Some(Grammar::TERM);
    }

    fn parse_fact(&mut self) -> Option<Grammar> {
        if self.lexer.peek_token() == None {
            return Some(Grammar::UNKNOWN);
        }

        let mut next_lexeme = self.lexer.next_token().clone().unwrap();

        let match_terminal_token = [Tokens::IDENTIFIER, Tokens::INTEGERS, Tokens::FLOATS];
        for token in match_terminal_token {
            if next_lexeme.token == token {
                return Some(Grammar::FACT);
            }
        }

        if next_lexeme.token != Tokens::LPARENTHESIS {
            return Some(Grammar::UNKNOWN);
        }

        if self.parse_expr() != Some(Grammar::EXPRESSION) {
            return Some(Grammar::UNKNOWN);
        }

        if self.lexer.peek_token().is_some() {
            next_lexeme = self.lexer.next_token().clone().unwrap();

            if next_lexeme.token == Tokens::RPARENTHESIS {
                return Some(Grammar::FACT);
            }
        }

        return Some(Grammar::UNKNOWN);
    }
}

#[derive(Debug, PartialEq)]
enum Grammar {
    STMTLIST,
    STMT,
    WHILESTATEMENT,
    IFSTATEMENT,
    BLOCK,
    ASSIGN,
    DECLARE,
    EXPRESSION,
    TERM,
    FACT,
    BOOLEXPRESSION,
    BTERM,
    BAND,
    BOR,
    UNKNOWN,
}