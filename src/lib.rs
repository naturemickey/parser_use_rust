
extern crate rust_s_exp;

use rust_s_exp::SNode;

pub struct Token {
    _type:i8,
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
    Token{_type:0, _name:"".to_string(), _line:0, _column:0}
}

pub fn parse_tree(grammar:&str, s:&str) -> STreeNode {
    _parse_tree(&rust_s_exp::parse(grammar), s)
}

fn _parse_tree(grammar:&Vec<SNode>, s:&str) -> STreeNode {
    STreeNode::Children(Vec::new())
}