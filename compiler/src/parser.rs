
use std::vec;

use crate::tokenizer::Token;
use crate::tokenizer::TokenType;
use std::collections::HashMap;

pub struct Parser {
    token_vec: TokenVec,
}

impl Parser {
    pub fn new(token_vec: Vec<Token>) -> Parser {
        Parser {
            token_vec: TokenVec { token_vec: token_vec },
        }
    }
    pub fn parse(&mut self) -> Result<ASTnode, String>{
        Ok(self.parse_statements(self.token_vec.clone()))
    }

    pub fn parse_statements(&mut self, mut token_vec: TokenVec) -> ASTnode{
        let mut seq_vec = vec![];
        let mut seq: ASTnode;
        while !token_vec.is_empty() {
            match token_vec.peek().token_type {
                TokenType::Emojis(_) => {
                    (seq, token_vec) = self.parse_assignment(token_vec);
                    seq_vec.push(seq);
                },
                TokenType::IfKeyword => {
                    (seq, token_vec) = self.parse_if(token_vec);
                    seq_vec.push(seq);
                },
                TokenType::PrintKeyword => {
                    (seq, token_vec) = self.parse_print(token_vec);
                    seq_vec.push(seq);
                },
                TokenType::LoopKeyword => {
                    (seq, token_vec) = self.parse_loop(token_vec);
                    seq_vec.push(seq);
                },
                TokenType::BreakKeyword => {
                    (seq, token_vec) = self.parse_break_kw(token_vec);
                    seq_vec.push(seq);
                }
                _ => panic!("arm not complete")
            }
        }
        ASTnode::StatementSeq(seq_vec)
    }

    fn parse_assignment(&mut self, mut token_vec: TokenVec) -> (ASTnode, TokenVec) {

        let mut c: Vec<ASTnode> = vec![];
        c.push(ASTnode::new_empty_node(token_vec.get_next()));
        let mut cur_node = ASTtoken::new(token_vec.get_next());
        
        let mut expr_vec = vec![];
        while token_vec.peek().token_type != TokenType::EOL {
            expr_vec.push(token_vec.get_next())
        }
        token_vec.next(); // remov EOL token
        let ast_node = self.parse_expression(expr_vec);
        c.push(ast_node);
        
        cur_node.children = Some(c);
    
        return (ASTnode::Node(cur_node), token_vec);
    }

    fn parse_expression(&mut self, token_vec: Vec<Token>) -> ASTnode {


        let length = token_vec.len();
        // EXIT RECURSION
        if length == 1 {
            return ASTnode::new_empty_node(token_vec[0].clone())
        
        }
        // HANDLE PARENS
        else if self.redundant_parens_wrapping(token_vec.clone()) {
            return self.parse_expression(token_vec[1..length-1].to_vec());
        }
        // SPLIT AT PLUS/MINUS IF EXISTS
        else if let Some(i) = self.get_idx_of(token_vec.clone(), true) {
            // HANDLE MINUS AT BEGINNING
            if i == 0 {
                let mut this_token = ASTtoken::new(token_vec[0].clone());
                this_token.children = Some(vec![
                    self.parse_expression(
                        token_vec[1..].to_vec()
                    )
                ]);
                return ASTnode::Node(this_token);
            } else {
                let mut this_token = ASTtoken::new(token_vec[i].clone());
                this_token.children = Some(vec![
                    self.parse_expression(
                        token_vec[0..i].to_vec()
                    ),
                    self.parse_expression(
                        token_vec[i+1..].to_vec()
                    )
                ]);
                return ASTnode::Node(this_token);
            }
        } else if let Some(i) = self.get_idx_of(token_vec.clone(), false) {
            let mut this_token = ASTtoken::new(token_vec[i].clone());
                this_token.children = Some(vec![
                    self.parse_expression(
                        token_vec[0..i].to_vec()
                    ),
                    self.parse_expression(
                        token_vec[i+1..].to_vec()
                    )
                ]);
                return ASTnode::Node(this_token);
        } else {
            dbg!(token_vec);
            panic!("not finsished! bajs");
        }
        
    }
    fn get_idx_of(&mut self, token_vec: Vec<Token>, add_sub: bool) -> Option<usize> {
        let mut i = 0;
        let mut paren_depth = 0;
        for t in token_vec {
            match t.token_type {
                TokenType::Add | TokenType::Subtract => {
                    if add_sub && paren_depth == 0{
                        return Some(i);
                    }
                },
                TokenType::Multiply | TokenType::Divide => {
                    if !add_sub && paren_depth == 0 {
                        return Some(i);
                    }
                },
                TokenType::LParen => {
                    paren_depth+=1;
                },
                TokenType::RParen => {
                    paren_depth-=1;
                },
                _ =>()
            }
            i+=1;
        }
        return None;
    }
    fn redundant_parens_wrapping(&self, token_vec: Vec<Token>) -> bool {
        let length = token_vec.len();
        if length == 0 {
            return false
        } else {
            if token_vec[0].token_type == TokenType::LParen && token_vec[length-1].token_type == TokenType::RParen {
                let mut paren_depth = 0;
                for t in token_vec[1..length-1].to_vec() {
                    match t.token_type {
                        TokenType::LParen => paren_depth +=1,
                        TokenType::RParen => paren_depth -=1,
                        _ => (),
                    }
                    if paren_depth < 0 {return false}
                }
                return true;
            } else {
                return false;
            }

        }
    }


    fn parse_if(&mut self, mut token_vec: TokenVec) -> (ASTnode, TokenVec) {
        let mut cur_token = ASTtoken::new(token_vec.get_next());
        let mut c: Vec<ASTnode> = vec![];
        let mut expr_0: Vec<Token> = vec![];
        let mut expr_1: Vec<Token> = vec![];
        loop {
            match token_vec.peek().token_type {
                TokenType::EQ | TokenType::NEQ | TokenType::GT | TokenType::GEQ | TokenType::LT | TokenType::LEQ => {
                    break;
                },
                _ => expr_0.push(token_vec.get_next())
            }
        }
        let cmp_token = token_vec.get_next();
        loop {
            match token_vec.peek().token_type {
                TokenType::LBrace => {
                    break;
                },
                _ => expr_1.push(token_vec.get_next())
            }
        }
        let cmp_node = ASTnode::Node(ASTtoken{
            token: cmp_token,
            children: Some(vec![
                self.parse_expression(expr_0),
                self.parse_expression(expr_1),
            ])
        });
        c.push(cmp_node);
        token_vec.next(); // REMOVE LBrace
        token_vec.next(); // REMOVE EOL
        
        
        let mut statements = TokenVec::new();
        let mut brace_depth = 0;
        loop {
            match token_vec.peek().token_type {
                TokenType::LBrace => brace_depth += 1,
                TokenType::RBrace => {
                    if brace_depth == 0 {
                        // THIS MEANS THAT EOL IS AT THE END OF statements, wich is as it should
                        break;
                    } else {
                        brace_depth -= 1;
                    }
                }
                _ => ()
            }
            statements.push(token_vec.get_next())
        }
        token_vec.next(); // REMOVE RBrace
        token_vec.next(); // REMOVE EOL
        c.push(self.parse_statements(statements));
        cur_token.children = Some(c);
        return (ASTnode::Node(cur_token), token_vec);
    }
    
    fn parse_print(&mut self, mut token_vec: TokenVec) -> (ASTnode, TokenVec) {
        let mut cur_token = ASTtoken::new(token_vec.get_next());
        token_vec.next(); // REMOVE LParen
        let mut expr: Vec<Token> = vec![];
        loop {
            match token_vec.peek().token_type {
                TokenType::RParen => {
                    break;
                },
                _ => expr.push(token_vec.get_next())
            }
        }
        token_vec.next(); // REMOVE RParen
        token_vec.next(); // REMOVE EOL
        cur_token.children = Some(vec![self.parse_expression(expr)]);
        return (ASTnode::Node(cur_token), token_vec);
    }

    fn parse_loop(&mut self, mut token_vec: TokenVec) -> (ASTnode, TokenVec) {
        let mut cur_token = ASTtoken::new(token_vec.get_next());
        let mut c: Vec<ASTnode> = vec![];
        token_vec.next(); // REMOVE LBrace
        token_vec.next(); // REMOVE EOL
        let mut statements = TokenVec::new();
        let mut brace_depth = 0;
        loop {
            match token_vec.peek().token_type {
                TokenType::LBrace => brace_depth += 1,
                TokenType::RBrace => {
                    if brace_depth == 0 {
                        // THIS MEANS THAT EOL IS AT THE END OF statements, wich is as it should
                        break;
                    } else {
                        brace_depth -= 1;
                    }
                }
                _ => ()
            }
            statements.push(token_vec.get_next())
        }
        token_vec.next(); // REMOVE RBrace
        token_vec.next(); // REMOVE EOL
        c.push(self.parse_statements(statements));
        cur_token.children = Some(c);
        return (ASTnode::Node(cur_token), token_vec);
    }


    fn parse_break_kw(&mut self, mut token_vec: TokenVec) -> (ASTnode, TokenVec) {
        let cur_token = ASTtoken::new(token_vec.get_next());
        token_vec.next(); // REMOVE EOL
        return (ASTnode::Node(cur_token), token_vec);
    }

}

#[derive(Debug, Clone)]
pub enum ASTnode {
    StatementSeq(Vec<ASTnode>),
    Node(ASTtoken)
}

impl ASTnode {
    fn new_empty_node(token: Token) -> ASTnode {
        ASTnode::Node(ASTtoken { token: token, children: None })
    }
}

// IDEA:
pub struct StatementSeq {
    seq: Vec<ASTnode>,
    symbol_table: HashMap<String, i32>,
}

#[derive(Debug, Clone)]
pub struct ASTtoken {
    token: Token,
    children: Option<Vec<ASTnode>>
}
impl ASTtoken {
    fn new(token: Token) -> ASTtoken {
        ASTtoken { token: token, children: None }
    }
}

#[derive(Debug, Clone)]
pub struct TokenVec {
    token_vec: Vec<Token>
}
impl TokenVec {
    fn new() -> TokenVec {
        TokenVec { token_vec: vec![] }
    }
    fn push(&mut self, new_token: Token) {
        self.token_vec.push(new_token);
    }

    // fn pop(&mut self) {
    //     if !self.is_empty() {
    //         self.token_vec = self.token_vec[..self.token_vec.len()-2].to_vec();
    //     }
    // }
    fn is_empty(&self) -> bool {
        self.token_vec.len() == 0
    }

    fn peek(&self) -> Token {
        if self.is_empty() {
            panic!("should not be empty")
        } else {
            self.token_vec[0].clone()
        }
    }

    fn next(&mut self) {
        if ! self.is_empty() {
            self.token_vec = self.token_vec[1..].to_vec();
        }
    }

    fn get_next(&mut self) -> Token {
        let tmp = self.peek();
        self.next();
        return tmp;
    }
}

// enum StatementType {
//     Assign,
//     If,
// }

