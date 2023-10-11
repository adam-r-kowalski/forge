use im::{hashmap, HashMap};
use rug::Integer;
use tao;

#[test]
fn evaluate_keyword() {
    let tokens = tao::tokenize(":x");
    let expression = tao::parse(tokens);
    let environment = HashMap::new();
    let (_, actual) = tao::evaluate(environment, expression);
    let expected = tao::Expression::Keyword(":x".to_string());
    assert_eq!(actual, expected);
}

#[test]
fn evaluate_string() {
    let tokens = tao::tokenize(r#""hello""#);
    let expression = tao::parse(tokens);
    let environment = HashMap::new();
    let (_, actual) = tao::evaluate(environment, expression);
    let expected = tao::Expression::String("hello".to_string());
    assert_eq!(actual, expected);
}

#[test]
fn evaluate_integer() {
    let tokens = tao::tokenize("5");
    let expression = tao::parse(tokens);
    let environment = HashMap::new();
    let (_, actual) = tao::evaluate(environment, expression);
    let expected = tao::Expression::Integer(Integer::from(5));
    assert_eq!(actual, expected);
}

#[test]
fn evaluate_float() {
    let tokens = tao::tokenize("3.14");
    let expression = tao::parse(tokens);
    let environment = HashMap::new();
    let (_, actual) = tao::evaluate(environment, expression);
    let expected = tao::Expression::Float(tao::Float::from_str("3.14"));
    assert_eq!(actual, expected);
}

#[test]
fn evaluate_unbound_symbol() {
    let tokens = tao::tokenize("x");
    let expression = tao::parse(tokens);
    let environment = HashMap::new();
    let (_, actual) = tao::evaluate(environment, expression);
    let expected = tao::Expression::Symbol("x".to_string());
    assert_eq!(actual, expected);
}

#[test]
fn evaluate_symbol_bound_to_integer() {
    let tokens = tao::tokenize("x");
    let expression = tao::parse(tokens);
    let environment = hashmap! {
        "x".to_string() => tao::Expression::Integer(Integer::from(5)),
    };
    let (_, actual) = tao::evaluate(environment, expression);
    let expected = tao::Expression::Integer(Integer::from(5));
    assert_eq!(actual, expected);
}

#[test]
fn evaluate_symbol_bound_to_function() {
    let tokens = tao::tokenize("(double 5)");
    let expression = tao::parse(tokens);
    let environment = hashmap! {
        "double".to_string() => tao::Expression::IntrinsicFunction(
          |env, args| {
            let (_, args) = tao::evaluate_arguments(env, args);
            match &args[0] {
              tao::Expression::Integer(i) => tao::Expression::Integer(i * Integer::from(2)),
              _ => panic!("Expected integer argument"),
            }
          }
        ),
    };
    let (_, actual) = tao::evaluate(environment, expression);
    let expected = tao::Expression::Integer(Integer::from(10));
    assert_eq!(actual, expected);
}

#[test]
fn evaluate_add() {
    let tokens = tao::tokenize("(+ 5 3)");
    let expression = tao::parse(tokens);
    let environment = hashmap! {
        "+".to_string() => tao::Expression::IntrinsicFunction(
          |env, args| {
            let (_, args) = tao::evaluate_arguments(env, args);
            match (&args[0], &args[1]) {
              (tao::Expression::Integer(lhs), tao::Expression::Integer(rhs)) => tao::Expression::Integer((lhs + rhs).into()),
              _ => panic!("Expected integer argument"),
            }
          }
        ),
    };
    let (_, actual) = tao::evaluate(environment, expression);
    let expected = tao::Expression::Integer(Integer::from(8));
    assert_eq!(actual, expected);
}

#[test]
fn evaluate_if_then_branch() {
    let tokens = tao::tokenize("(if true 1 2)");
    let expression = tao::parse(tokens);
    let environment = hashmap! {
        "if".to_string() => tao::Expression::IntrinsicFunction(
          |env, args| {
            let (condition, then, otherwise) = (args[0].clone(), args[1].clone(), args[2].clone());
            let (env, condition) = tao::evaluate(env, condition);
            let (_, e) = match condition {
                tao::Expression::Nil => tao::evaluate(env, otherwise),
                tao::Expression::Bool(false) => tao::evaluate(env, otherwise),
                _ => tao::evaluate(env, then),
            };
            e
          }
        ),
    };
    let (_, actual) = tao::evaluate(environment, expression);
    let expected = tao::Expression::Integer(Integer::from(1));
    assert_eq!(actual, expected);
}

#[test]
fn evaluate_if_else_branch() {
    let tokens = tao::tokenize("(if false 1 2)");
    let expression = tao::parse(tokens);
    let environment = hashmap! {
        "if".to_string() => tao::Expression::IntrinsicFunction(
          |env, args| {
            let (condition, then, otherwise) = (args[0].clone(), args[1].clone(), args[2].clone());
            let (env, condition) = tao::evaluate(env, condition);
            let (_, e) = match condition {
                tao::Expression::Nil => tao::evaluate(env, otherwise),
                tao::Expression::Bool(false) => tao::evaluate(env, otherwise),
                _ => tao::evaluate(env, then),
            };
            e
          }
        ),
    };
    let (_, actual) = tao::evaluate(environment, expression);
    let expected = tao::Expression::Integer(Integer::from(2));
    assert_eq!(actual, expected);
}
