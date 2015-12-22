
extern crate rust_s_exp;

use rust_s_exp::SNode;

pub struct Token {
    _type:i8,
    _name:String,
    _line:usize,
    _column:usize,
}