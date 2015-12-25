
extern crate rust_s_exp;

use rust_s_exp::SNode;
use std::collections::HashMap;

pub struct Token {
    _type:String,
    _name:String,
    _line:usize,
    _column:usize,
}

pub enum STreeNode {
    Leaf(Token),
    Children(Vec<STreeNode>)
}

pub fn parse_lex(lex:&str, s:&str) -> Token {
    _parse_lex(&rust_s_exp::parse(lex), s)
}

fn _parse_lex(lex:&Vec<SNode>, s:&str) -> Token {
    // lex_name --> lex_define
    let lexMap = HashMap::new();
    for snode in lex {
        if snode.is_leaf() {
            panic!("lex define error");
        } else {
            
        }
    }
    Token{_type:"".to_string(), _name:"".to_string(), _line:0, _column:0}
}

fn get_one_lex_define(lex:&SNode) -> (String, SNode) {
    if lex.is_leaf() || lex.children.len() != 3 {
        panic!("lex define error");
    } else { unsafe {
        let n0 = lex.children.get_unchecked(0);
        let n1 = lex.children.get_unchecked(1);
        let n2 = lex.children.get_unchecked(2);
        if n0.is_leaf() && n0._type == 0 && n0.token == "lex" {
            if n1.is_leaf() && n1._type == 1 {
                let lex_name = n1.token;
                if n2.is_leaf() {
                    panic!("lex define error");
                }
                (lex_name, n2)
            } else {
                panic!("lex define error");
            }
        } else {
            panic!("lex define error");
        }
    }}
}

pub fn parse_tree(grammar:&str, s:&str) -> STreeNode {
    _parse_tree(&rust_s_exp::parse(grammar), s)
}

fn _parse_tree(grammar:&Vec<SNode>, s:&str) -> STreeNode {
    STreeNode::Children(Vec::new())
}