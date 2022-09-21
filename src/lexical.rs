use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub grammar); // synthesized by LALRPOP

fn parse_constanttype(typestr: &str) {
    println!("Parsing constant_type: {}", typestr);
    println!("{:?}", grammar::constant_typeParser::new().parse(typestr));
}

fn parse_identifier(typestr: &str) {
    println!("Parsing identifier: {}", typestr);
    println!("{:?}", grammar::identifierParser::new().parse(typestr));
}

enum Parsers {

}

fn parse_word(word: &str){
    grammar::constant_typeParser::new().parse(word);
}

fn parse_text(){}

mod tests {
    use lalrpop_util::lalrpop_mod;
    lalrpop_mod!(pub grammar); // synthesized by LALRPOP

    #[test]
    fn parse_type(){
        assert!(grammar::constant_typeParser::new().parse("int").is_ok());
        assert!(grammar::constant_typeParser::new().parse("string").is_ok());
        assert!(grammar::constant_typeParser::new().parse("float").is_ok());
    }
}