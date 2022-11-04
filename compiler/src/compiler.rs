
use crate::parser::*;

use crate::tokenizer::*;


use std::collections::HashMap;



use std::fmt::format;
// CREATING FILE
use std::fs::File;
use std::io::prelude::*;
use std::ops::RangeToInclusive;
// !CREATING FILE

pub struct Compiler {
    cw: CWriter
}
impl Compiler {
    pub fn new() -> Compiler {
        Compiler {cw: CWriter::new()}
    }

    pub fn compile(&mut self, root: ASTnode) -> Result<String, String> {
        let mut stmts = match root {
            ASTnode::StatementSeq(v) => v,
            ASTnode::Node(n) => return Err("root is always statement seq".to_string())
        };

        self.cw.write_opening();
        let mut st = SymbolTable::new();
        match self.compile_stmts(stmts, st) {
            Err(e) => return Err(e),
            Ok(st) => (),
        };
        self.cw.write_closing();
        self.cw.compile_to_c();
        return self.ok();
    }

    fn compile_stmts(&mut self, stmts: Vec<ASTnode>, mut st: SymbolTable) -> Result<String, String> {
        for stmt in stmts {
            st = match self.compile_stmt(stmt, st) {
                Err(e) => return Err(e),
                Ok(st) => st,
            }
        }
        return self.ok();
    }

    fn compile_stmt(&mut self, stmt: ASTnode, mut st: SymbolTable) -> Result<SymbolTable, String> {
        let ast_token = match stmt {
            ASTnode::StatementSeq(_) => return Err("only ASTnodes in seq".to_string()),
            ASTnode::Node(t) => t
        };
        let status = match ast_token.token.token_type {
            TokenType::PrintKeyword => {
                self.compile_print(ast_token, st)
            },
            TokenType::Assign => {
                self.compile_assign(ast_token, st)
            },
            TokenType::IfKeyword => {
                self.compile_if(ast_token, st)
            },
            TokenType::LoopKeyword => {
                self.compile_loop(ast_token, st)
            },
            TokenType::BreakKeyword => self.compile_break(st),
            _ => return Err("unexpected tokentype in compile_stmt()".to_string())
        };
        return status
    }

    fn compile_loop(&mut self, cur_token: ASTtoken, mut st: SymbolTable) -> Result<SymbolTable, String> {
        self.cw.write("while(1){\n");
        let stmt_seq = match cur_token.children {
            None => return Err("expected children".to_string()),
            Some(v) => {
                match v[0].clone() {
                    ASTnode::StatementSeq(ssq) => ssq,
                    _ => panic!()
                }
            }
        };
        match self.compile_stmts(stmt_seq, st.clone()) {
            Err(e) => return Err(e),
            Ok(_) => ()
        }
        self.cw.write("}\n");
        Ok(st)
    }

    fn compile_if(&mut self, cur_token: ASTtoken, mut st: SymbolTable) -> Result<SymbolTable, String> {
        self.cw.write("if (");
        let (cmp, stmt_seq) = match cur_token.children {
            None => return Err("expected children".to_string()),
            Some(v) => {
                let cmp = match v[0].clone() {
                    ASTnode::Node(t) => t,
                    _ => panic!()
                };
                let stmt_seq = match v[1].clone() {
                    ASTnode::StatementSeq(ssq) => ssq,
                    _ => panic!()
                };
                (cmp, stmt_seq)
            }
        };
        let (expr0, expr1) = match cmp.children {
            None => return Err("expected children of cmp".to_string()),
            Some(v) => {
                let expr0 = match v[0].clone() {
                    ASTnode::Node(t) => t,
                    _ => panic!()
                };
                let expr1 = match v[1].clone() {
                    ASTnode::Node(t) => t,
                    _ => panic!()
                };
                (expr0, expr1)
            }
        };
        match self.compile_expr(expr0, st.clone()) {
            Err(e) => return Err(e),
            Ok(_) => ()
        }
        let cmp_str = match cmp.token.token_type {
            TokenType::EQ => "=",
            TokenType::NEQ => "!=",
            TokenType::GT => ">",
            TokenType::LT => "<",
            TokenType::GEQ => "=>",
            TokenType::LEQ => "<=",
            _ => return Err("token not cmp".to_string())
        };
        self.cw.write(cmp_str);
        match self.compile_expr(expr1, st.clone()) {
            Err(e) => return Err(e),
            Ok(_) => ()
        }
        self.cw.write(") {\n");
        match self.compile_stmts(stmt_seq, st.clone()) {
            Err(e) => return Err(e),
            Ok(_) => ()
        }
        self.cw.write("}\n");
        return Ok(st);

    }

    fn compile_assign(&mut self, cur_token: ASTtoken, mut st: SymbolTable) -> Result<SymbolTable, String> {
        let (lc, rc) = match cur_token.children {
            None => return Err("expected children".to_string()),
            Some(v) => {
                let t0 = match v[0].clone() {
                    ASTnode::Node(t0) => t0,
                    _ => panic!(),
                };
                let t1 = match v[1].clone() {
                    ASTnode::Node(t1) => t1,
                    _ => panic!(),
                };
                (t0, t1)
            }
        };
        let var = match lc.token.token_type {
            TokenType::Emojis(s) => s,
            _ => panic!()
        };
        // DECLARE NEW VARIABLE ONLY IF NOT ALREADY EXISTS
        if !st.contains(&var) {
            self.cw.write("int ");
        }
        let var_in_c = st.assign(var);
        self.cw.write_String(var_in_c+" = ");
        match self.compile_expr(rc, st.clone()) {
            Err(e) => return Err(e),
            Ok(_) => ()
        }
        self.cw.write(";\n");
        Ok(st)
    }

    fn compile_break(&mut self, st: SymbolTable) -> Result<SymbolTable, String> {
        self.cw.write("break;\n");
        Ok(st)
    }

    fn compile_print(&mut self, cur_token: ASTtoken, st: SymbolTable) -> Result<SymbolTable, String> {
        self.cw.write("printf(\"%d\\n\", ");
        let c = match cur_token.children {
            None => return Err("print should have child".to_string()),
            Some(v) => v
        };
        let expr_token = match c[0].clone() {
            ASTnode::Node(t) => t,
            _ => return Err("child of print must be ASTtoken".to_string())
        };
        match self.compile_expr(expr_token, st.clone()) {
            Err(e) => return Err(e),
            Ok(_) => ()
        }
        self.cw.write(");\n");
        Ok(st)
    }

    fn compile_expr(&mut self, cur_token: ASTtoken, st: SymbolTable) -> Result<String, String> {
        match cur_token.token.token_type {
            TokenType::Int(i) => {
                self.cw.write_int(i);
            },
            TokenType::Emojis(s) => {
                self.cw.write_String(st.get(s))
            },
            TokenType::Subtract => {
                match cur_token.children {
                    None => return Err("no children found".to_string()),
                    Some(v) => {
                        match v.len() {
                            1 => {
                                let child_token = match v[0].clone() {
                                    ASTnode::Node(t) => t,
                                    _ => panic!()
                                };
                                self.cw.write("-(");
                                match self.compile_expr(child_token, st) {
                                    Err(e) => return Err(e),
                                    Ok(_) => ()
                                }
                                self.cw.write(")");
                                return self.ok();
                            },
                            2 => {
                                let child_token_0 = match v[0].clone() {
                                    ASTnode::Node(t) => t,
                                    _ => panic!()
                                };
                                let child_token_1 = match v[1].clone() {
                                    ASTnode::Node(t) => t,
                                    _ => panic!()
                                };
                                self.cw.write("(");
                                match self.compile_expr(child_token_0, st.clone()) {
                                    Ok(_) => (),
                                    Err(e) => return Err(e),
                                }
                                self.cw.write(")-(");
                                match self.compile_expr(child_token_1, st) {
                                    Err(e) => return Err(e),
                                    Ok(_) => ()
                                }
                                self.cw.write(")");
                            },
                            _ => panic!()
                        }
                    }
                }
            },
            TokenType::Add | TokenType::Multiply | TokenType::Divide => {
                let child_vec = match cur_token.children {
                    None => return Err("expected two children".to_string()),
                    Some(v) => v,
                };
                let child_token_0 = match child_vec[0].clone() {
                    ASTnode::Node(t) => t,
                    _ => panic!()
                };
                let child_token_1 = match child_vec[1].clone() {
                    ASTnode::Node(t) => t,
                    _ => panic!()
                };
                self.cw.write("(");
                match self.compile_expr(child_token_0, st.clone()) {
                    Ok(_) => (),
                    Err(e) => return Err(e),
                }
                self.cw.write(")");
                let s = match cur_token.token.token_type {
                    TokenType::Add => "+",
                    TokenType::Multiply => "*",
                    TokenType::Divide => "/",
                    _ => panic!()
                };
                self.cw.write(s);
                self.cw.write("(");
                match self.compile_expr(child_token_1, st) {
                    Err(e) => return Err(e),
                    Ok(_) => ()
                }
                self.cw.write(")");
                return self.ok();
            }
            _ => return Err("unexpected tokentype".to_string())
        }
        Ok("".to_string())
    }

    fn ok(&self) -> Result<String, String> {
        Ok("".to_string())
    }
}


#[derive(Debug, Clone)]
struct SymbolTable {
    table: HashMap<String, String>
}

impl SymbolTable {
    fn new() -> SymbolTable {
        SymbolTable { table: HashMap::new() }
    }

    fn get(&self, k: String) -> String {
        match self.table.get(&k) {
            None => panic!("{}, is undefined", k),
            Some(v) => v.to_string()
        }
    }

    fn assign(&mut self, k: String) -> String {
        match self.table.get(&k) {
            None => {
                let new_name = format!("v{}", self.table.len());
                self.table.insert(k, new_name.clone());
                new_name
            },
            Some(s) => s.to_string()
        }
    }

    fn contains(&self, k: &String) -> bool {
        self.table.contains_key(k)
    }
}


struct CWriter {
    c_code: String
}

impl CWriter {
    fn new() -> CWriter {
        CWriter { c_code: "".to_string() }
    }
    fn nl(&mut self) {
        self.c_code += "\n";
    }
    fn write_int(&mut self, i: i32) {
        self.c_code += &format!("{}", i)[..];
    }
    fn write(&mut self, str: &'static str) {
        self.c_code += str;
    }
    fn write_String(&mut self, Str: String) {
        self.c_code += &Str[..];
    }
    fn write_opening(&mut self) {
        self.c_code += "#include <stdio.h>\nint main() {\n"
    }
    fn write_closing(&mut self) {
        self.c_code += "return 0;\n}"
    }
    fn compile_to_c(&self) {
        let s = self.c_code.as_bytes();
        let mut file = File::create("output/main.c").expect("Error encountered while creating file!");
        file.write_all(s).expect("Error while writing to file");
    }
}