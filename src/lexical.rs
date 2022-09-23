use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub grammar); // synthesized by LALRPOP

fn parse_constanttype(typestr: &str) {
    println!("Parsing constant_type: {}", typestr);
    println!("{:?}", grammar::vartypeParser::new().parse(typestr));
}

fn parse_identifier(typestr: &str) {
    println!("Parsing identifier: {}", typestr);
    println!("{:?}", grammar::identParser::new().parse(typestr));
}

enum Parsers {}

fn parse_word(word: &str) {
    grammar::vartypeParser::new().parse(word);
}

fn parse_text() {}

mod tests {
    use std::fs;

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
}
