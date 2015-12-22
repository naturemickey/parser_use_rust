extern crate parser_use_rust;

use parser_use_rust::Token;

#[test]
fn it_works() {
    // (@... "/*" (@* (@! "*/")) "*/")
    assert!(true);
}
#[test]
fn test_and() {
    let s = "(lex ab (@... 'a' 'b'))";
}
