extern crate alloc;

use crate::evaluate_expressions;
use crate::Expression;
use crate::Expression::{Integer, IntrinsicFunction, Ratio};
use crate::RaisedEffect;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use im::{hashmap, vector, HashMap};
use rug;

fn error(message: &str) -> RaisedEffect {
    RaisedEffect {
        environment: environment(),
        effect: "error".to_string(),
        arguments: vector![Expression::String(message.to_string())],
    }
}

fn html(expr: Expression, string: &mut String) -> core::result::Result<(), RaisedEffect> {
    match expr {
        Expression::Array(a) => match &a[0] {
            Expression::Keyword(s) => {
                let s = &s[1..];
                string.push('<');
                string.push_str(s);
                string.push('>');
                if a.len() > 1 {
                    html(a[1].clone(), string)?;
                }
                string.push_str("</");
                string.push_str(s);
                string.push('>');
                Ok(())
            }
            _ => Err(error("Expected keyword")),
        },
        Expression::String(s) => {
            string.push_str(&s);
            Ok(())
        }
        _ => Err(error("Expected keyword")),
    }
}

pub fn environment() -> HashMap<String, Expression> {
    hashmap! {
        "+".to_string() => IntrinsicFunction(
          |env, args| {
            let (env, args) = evaluate_expressions(env, args)?;
            match (&args[0], &args[1]) {
              (Integer(lhs), Integer(rhs)) => Ok((env, Integer((lhs + rhs).into()))),
              _ => Err(error("Expected integer argument")),
            }
          }
        ),
        "-".to_string() => IntrinsicFunction(
          |env, args| {
            let (env, args) = evaluate_expressions(env, args)?;
            match (&args[0], &args[1]) {
              (Integer(lhs), Integer(rhs)) => Ok((env, Integer((lhs - rhs).into()))),
              _ => Err(error("Expected integer argument")),
            }
          }
        ),
        "*".to_string() => IntrinsicFunction(
          |env, args| {
            let (env, args) = evaluate_expressions(env, args)?;
            match (&args[0], &args[1]) {
              (Integer(lhs), Integer(rhs)) => Ok((env, Integer((lhs * rhs).into()))),
              _ => Err(error("Expected integer argument")),
            }
          }
        ),
        "/".to_string() => IntrinsicFunction(
          |env, args| {
            let (env, args) = evaluate_expressions(env, args)?;
            match (&args[0], &args[1]) {
              (Integer(lhs), Integer(rhs)) => {
                let rational = rug::Rational::from((lhs, rhs));
                if rational.is_integer() {
                    Ok((env, Integer(rational.numer().clone())))
                } else {
                    Ok((env, Ratio(rational)))
                }
              },
              _ => Err(error("Expected integer argument")),
            }
          }
        ),
        "if".to_string() => IntrinsicFunction(
          |env, args| {
            let (condition, then, otherwise) = (args[0].clone(), args[1].clone(), args[2].clone());
            let (env, condition) = crate::evaluate(env, condition)?;
            match condition {
                Expression::Nil => crate::evaluate(env, otherwise),
                Expression::Bool(false) => crate::evaluate(env, otherwise),
                _ => crate::evaluate(env, then),
            }
          }
        ),
        "def".to_string() => IntrinsicFunction(
          |env, args| {
            let (name, value) = (args[0].clone(), args[1].clone());
            let (env, value) = crate::evaluate(env, value)?;
            let name = match name {
                Expression::Symbol(s) => s,
                _ => return Err(error("Expected symbol")),
            };
            let mut new_env = env.clone();
            new_env.insert(name, value);
            Ok((new_env, Expression::Nil))
          }
        ),
        "fn".to_string() => IntrinsicFunction(
          |env, args| {
            let (parameters, body) = (args[0].clone(), args[1].clone());
            let parameters = match parameters {
                Expression::Array(a) => a,
                _ => return Err(error("Expected array")),
            };
            for parameter in parameters.iter() {
                match parameter {
                    Expression::Symbol(_) => {},
                    _ => return Err(error("Expected symbol")),
                }
            }
            let body = Box::new(body);
            let function = Expression::Function{parameters, body};
            Ok((env, function))
          }
        ),
        "defn".to_string() => IntrinsicFunction(
          |env, args| {
            let (name, parameters, body) = (args[0].clone(), args[1].clone(), args[2].clone());
            let (env, function) = crate::evaluate(env, Expression::Call{
                function: Box::new(Expression::Symbol("fn".to_string())),
                arguments: vector![parameters, body],
            })?;
            crate::evaluate(env, Expression::Call{
                function: Box::new(Expression::Symbol("def".to_string())),
                arguments: vector![name, function],
            })
          }
        ),
        "assoc".to_string() => IntrinsicFunction(
          |env, args| {
            let (env, args) = evaluate_expressions(env, args)?;
            let (map, key, value) = (args[0].clone(), args[1].clone(), args[2].clone());
            match map {
                Expression::Map(m) => {
                    let mut new_map = m.clone();
                    new_map.insert(key, value);
                    Ok((env, Expression::Map(new_map)))
                },
                _ => Err(error("Expected map")),
            }
          }
        ),
        "dissoc".to_string() => IntrinsicFunction(
          |env, args| {
            let (env, args) = evaluate_expressions(env, args)?;
            let (map, key) = (args[0].clone(), args[1].clone());
            match map {
                Expression::Map(m) => {
                    let mut new_map = m.clone();
                    new_map.remove(&key);
                    Ok((env, Expression::Map(new_map)))
                },
                _ => Err(error("Expected map")),
            }
          }
        ),
        "merge".to_string() => IntrinsicFunction(
          |env, args| {
            let (env, args) = evaluate_expressions(env, args)?;
            let (map1, map2) = (args[0].clone(), args[1].clone());
            match (map1, map2) {
                (Expression::Map(m1), Expression::Map(m2)) => {
                    let mut new_map = m1.clone();
                    new_map.extend(m2);
                    Ok((env, Expression::Map(new_map)))
                },
                _ => Err(error("Expected map")),
            }
          }
        ),
        "eval".to_string() => IntrinsicFunction(
          |env, args| {
            let (env, arg) = crate::evaluate(env, args[0].clone())?;
            crate::evaluate(env, arg)
          }
        ),
        "read-string".to_string() => IntrinsicFunction(
          |env, args| {
            let (env, arg) = crate::evaluate(env, args[0].clone())?;
            match arg {
              Expression::String(s) => {
                let tokens = crate::Tokens::from_str(&s);
                let expression = crate::parse(tokens);
                Ok((env, expression))
              },
              _ => Err(error("Expected string")),
            }
          }
        ),
        "html".to_string() => IntrinsicFunction(|env, args| {
            let (env, args) = evaluate_expressions(env, args)?;
            let mut string = String::new();
            html(args[0].clone(), &mut string)?;
            Ok((env, Expression::String(string)))
        }),
    }
}
