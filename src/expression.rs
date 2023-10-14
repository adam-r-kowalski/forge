extern crate alloc;

use crate::numerics::Float;
use alloc::boxed::Box;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use core::fmt::{self, Display, Formatter};
use im::{HashMap, Vector};
use rug::{Integer, Rational};

type Expressions = Vector<Expression>;

pub type Environment = HashMap<String, Expression>;

#[derive(Debug)]
pub struct RaisedEffect {
    pub environment: Environment,
    pub effect: String,
    pub arguments: Vector<Expression>,
}

pub type Result = core::result::Result<(Environment, Expression), RaisedEffect>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expression {
    Symbol(String),
    Keyword(String),
    String(String),
    Integer(Integer),
    Float(Float),
    Ratio(Rational),
    Bool(bool),
    Nil,
    Array(Expressions),
    Map(HashMap<Expression, Expression>),
    Call {
        function: Box<Expression>,
        arguments: Expressions,
    },
    Function {
        parameters: Expressions,
        body: Box<Expression>,
    },
    IntrinsicFunction(fn(Environment, Expressions) -> Result),
    Quote(Box<Expression>),
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Map(map) => {
                write!(f, "{{")?;
                for (i, (k, v)) in map.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", {} {}", k, v)?;
                    } else {
                        write!(f, "{} {}", k, v)?;
                    }
                }
                write!(f, "}}")
            }
            Expression::Array(arr) => {
                write!(f, "[")?;
                for (i, e) in arr.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", {}", e)?;
                    } else {
                        write!(f, "{}", e)?;
                    }
                }
                write!(f, "]")
            }
            Expression::Symbol(s) => write!(f, "{}", s),
            Expression::Keyword(k) => write!(f, "{}", k),
            Expression::String(s) => write!(f, "\"{}\"", s),
            Expression::Integer(i) => write!(f, "{}", i),
            Expression::Float(fl) => write!(f, "{}", fl),
            Expression::Ratio(r) => write!(f, "{}/{}", r.numer(), r.denom()),
            Expression::Bool(b) => write!(f, "{}", if *b { "true" } else { "false" }),
            Expression::Nil => write!(f, "nil"),
            Expression::Call {
                function,
                arguments,
            } => {
                let arg_strs: Vec<String> = arguments.iter().map(|e| format!("{}", e)).collect();
                write!(f, "({} {})", function, arg_strs.join(" "))
            }
            Expression::Function { parameters, body } => {
                let param_strs: Vec<String> = parameters.iter().map(|e| format!("{}", e)).collect();
                write!(f, "(fn [{}] {})", param_strs.join(" "), body)
            }
            Expression::IntrinsicFunction(_) => write!(f, "#intrinsic"),
            Expression::Quote(e) => write!(f, "'{}", e),
        }
    }
}
