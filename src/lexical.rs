use std::fs;

use crate::language_definition::{Token, TokenType};
use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub grammar); // synthesized by LALRPOP
lalrpop_mod!(pub grammar_tree); // synthesized by LALRPOP

pub struct SymbolTable {
    tokens: Vec<Token>,
}

pub fn parse_into_symbol_table(filename: &str) -> SymbolTable {
    let contents = fs::read_to_string(filename).unwrap();
    let tokentype_vec = parse_code(filename);
    let mut cur_pos: usize = 0;
    let mut result_vec: Vec<Token> = vec![];

    //TODO: Refatorar de forma funcional
    for element in tokentype_vec {
        let new_contents = contents[cur_pos..].to_string();
        let token_value = element.get_value();
        let token_pos = new_contents.find(&token_value).unwrap() + cur_pos;
        let token_size = token_value.len();
        result_vec.push(Token {
            token_type: element.clone(),
            value: token_value,
            position: token_pos,
            size: token_size,
        });

        cur_pos = token_pos + token_size;
    }
    SymbolTable { tokens: result_vec }
}

pub fn print_symbol_table(st: SymbolTable) {
    println!(
        "{0: <10}{1: <10}{2: <70}{3}",
        "POSTION", "SIZE", "VALUE", "TOKEN TYPE"
    );
    for element in st.tokens {
        println!(
            "{0: <10}{1: <10}{2: <70}{3:?}",
            element.position, element.size, element.value, element.token_type
        );
    }
}

pub fn parse_code<'input>(filename: &str) -> Vec<TokenType> {
    let contents = fs::read_to_string(filename).unwrap();
    let result = grammar::fileParser::new().parse(&contents);
    result.unwrap()
}

mod tests {
    use super::*;

    use lalrpop_util::lalrpop_mod;
    lalrpop_mod!(pub grammar); // synthesized by LALRPOP

    fn _test_code_file(filename: &str) {
        let contents = fs::read_to_string(filename).unwrap();
        let result = grammar_tree::fileParser::new().parse(&contents);
        println!("{:?}", result);
        assert!(result.is_ok());
    }

    #[test]
    fn parse_type() {
        assert!(grammar::vartypeParser::new().parse("int").is_ok());
        assert!(grammar::vartypeParser::new().parse("string").is_ok());
        assert!(grammar::vartypeParser::new().parse("float").is_ok());
    }

    #[test]
    fn parse_sample_code_1() {
        _test_code_file("src/examples/code1.lcc")
    }

    #[test]
    fn parse_sample_code_2() {
        _test_code_file("src/examples/code2.lcc")
    }

    #[test]
    fn parse_sample_code_3() {
        _test_code_file("src/examples/code3.lcc")
    }

    #[test]
    fn parse_int_constant() {
        println!("{:?}", grammar::int_constantParser::new().parse("54"));
    }

    #[test]
    fn test_symbol_table() {
        let x = parse_into_symbol_table("src/examples/code3.lcc");
        print_symbol_table(x);
    }
}
