use std::fs;

use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub grammar); // synthesized by LALRPOP

fn parse_word(word: &str) {
    grammar::vartypeParser::new().parse(word);
}

pub fn parse_code<'input>(filename: &str) {
    let contents = fs::read_to_string(filename).unwrap();
    let bla = grammar::fileParser::new().parse(&contents);
    println!("{:?}", grammar::fileParser::new().parse(&contents));
}

// // Desisti dessa porcaria, por agora
// fn parse_code<'input>(
//     filename: &str,
// ) -> Result<(), lalrpop_util::ParseError<usize, lalrpop_util::lexer::Token<'input>, &'static str>> {
//     let contents = fs::read_to_string(filename).unwrap();
//     let bla = grammar::fileParser::new()
//         .parse(&contents)
//         .map_err(|e| e.map_token(|tt| tt.clone()));

//     bla
// }

// fn parse_code<'input>(filename: &str) -> Result<(), TrabalhoError> {
//     let contents = fs::read_to_string(filename).unwrap();
//     grammar::fileParser::new()
//         .parse(&contents)
//         .map_err(|e| ParseErrorContents::from_lalrpop_parse_error_to_error(e))
// }

mod tests {
    use crate::language_definition;

    use super::*;

    use lalrpop_util::lalrpop_mod;
    lalrpop_mod!(pub grammar); // synthesized by LALRPOP

    fn test_code_file(filename: &str) {
        let contents = fs::read_to_string(filename).unwrap();
        let result = grammar::fileParser::new().parse(&contents);
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
        test_code_file("src/examples/code1.lcc")
    }

    #[test]
    fn parse_sample_code_2() {
        test_code_file("src/examples/code2.lcc")
    }

    #[test]
    fn parse_sample_code_3() {
        test_code_file("src/examples/code3.lcc")
    }

    #[test]
    fn parse_int_constant() {
        println!("{:?}", grammar::int_constantParser::new().parse("54"));
    }
}
