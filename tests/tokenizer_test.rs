use rug::Integer;
use forge;

#[test]
fn tokenize_symbol() {
    let actual = forge::tokenize("snake_case PascalCase kebab-case camelCase predicate?");
    let expected = vec![
        forge::Token::Symbol("snake_case".to_string()),
        forge::Token::Symbol("PascalCase".to_string()),
        forge::Token::Symbol("kebab-case".to_string()),
        forge::Token::Symbol("camelCase".to_string()),
        forge::Token::Symbol("predicate?".to_string()),
    ];
    assert_eq!(actual, expected);
}

#[test]
fn tokenize_keyword() {
    let actual = forge::tokenize(":snake_case :PascalCase :kebab-case :camelCase :predicate?");
    let expected = vec![
        forge::Token::Keyword(":snake_case".to_string()),
        forge::Token::Keyword(":PascalCase".to_string()),
        forge::Token::Keyword(":kebab-case".to_string()),
        forge::Token::Keyword(":camelCase".to_string()),
        forge::Token::Keyword(":predicate?".to_string()),
    ];
    assert_eq!(actual, expected);
}

#[test]
fn tokenize_string_literal() {
    let actual = forge::tokenize(r#""hello" "world" "123""#);
    let expected = vec![
        forge::Token::String("hello".to_string()),
        forge::Token::String("world".to_string()),
        forge::Token::String("123".to_string()),
    ];
    assert_eq!(actual, expected);
}

#[test]
fn tokenize_integer() {
    let actual = forge::tokenize("123 456 789 1_000 -321 -456");
    let expected = vec![
        forge::Token::Integer(Integer::from(123)),
        forge::Token::Integer(Integer::from(456)),
        forge::Token::Integer(Integer::from(789)),
        forge::Token::Integer(Integer::from(1000)),
        forge::Token::Integer(Integer::from(-321)),
        forge::Token::Integer(Integer::from(-456)),
    ];
    assert_eq!(actual, expected);
}

#[test]
fn tokenize_float() {
    let actual = forge::tokenize("1.23 4.56 7.89 1_000.0 -3.23");
    let expected = vec![
        forge::Token::Float(forge::Float::from_str("1.23")),
        forge::Token::Float(forge::Float::from_str("4.56")),
        forge::Token::Float(forge::Float::from_str("7.89")),
        forge::Token::Float(forge::Float::from_str("1000.0")),
        forge::Token::Float(forge::Float::from_str("-3.23")),
    ];
    assert_eq!(actual, expected);
}

#[test]
fn tokenize_delimiters() {
    let actual = forge::tokenize("( { [ ] } )");
    let expected = vec![
        forge::Token::LeftParen,
        forge::Token::LeftBrace,
        forge::Token::LeftBracket,
        forge::Token::RightBracket,
        forge::Token::RightBrace,
        forge::Token::RightParen,
    ];
    assert_eq!(actual, expected);
}

#[test]
fn tokenize_call_inside_array() {
    let actual = forge::tokenize("[3.14 (+ 2 3)]");
    let expected = vec![
        forge::Token::LeftBracket,
        forge::Token::Float(forge::Float::from_str("3.14")),
        forge::Token::LeftParen,
        forge::Token::Symbol("+".to_string()),
        forge::Token::Integer(Integer::from(2)),
        forge::Token::Integer(Integer::from(3)),
        forge::Token::RightParen,
        forge::Token::RightBracket,
    ];
    assert_eq!(actual, expected);
}

#[test]
fn tokenize_rational() {
    let actual = forge::tokenize("1/2");
    let expected = vec![
        forge::Token::Integer(Integer::from(1)),
        forge::Token::Symbol("/".to_string()),
        forge::Token::Integer(Integer::from(2)),
    ];
    assert_eq!(actual, expected);
}

#[test]
fn tokenize_quote() {
    let actual = forge::tokenize("'(1 2)");
    let expected = vec![
        forge::Token::Quote,
        forge::Token::LeftParen,
        forge::Token::Integer(Integer::from(1)),
        forge::Token::Integer(Integer::from(2)),
        forge::Token::RightParen,
    ];
    assert_eq!(actual, expected);
}
