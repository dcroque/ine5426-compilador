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

fn main() {
    parse_constanttype("int");
    parse_constanttype("string");
    parse_constanttype("float");

    parse_constanttype("tipodoido");

    parse_identifier("normal");
    parse_identifier("CamelCase");
    parse_identifier("numero9ok");
    
    parse_identifier("snake_case");
    parse_identifier("7invalido");
}

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