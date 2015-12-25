extern crate parser_use_rust;

use parser_use_rust::Token;

//!
//! (lex 词法名 匹配模式)
//!
//!
//!

#[test]
fn it_works() {
    // (@... "/*" (@* (@! "*/")) "*/")
    assert!(true);
}
#[test]
fn test_and() {
    let lex_str = "(lex ab (@... 'a' 'b'))";
}
