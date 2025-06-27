use std::ops::Neg;

use crate::{Error, Result, eval::Context, lexer::OperatorType};

/// Expression type, represents... an expression.  
/// It could be everything, from just a number like `2` till a function call
///
/// rexpr uses this one to recursively evaluate the "main" expression, the one you get from
/// [`rexpr::eval`][`crate::eval()`]. In that expression all other expressions are nested in and
/// evaluated recursively.
#[derive(Debug, PartialEq)]
pub enum Expression {
    /// Just a number, like "2"
    Number(f64),
    /// Some variable inside an expression, like "PI"
    Variable(String),

    /// Unary expression, like negotiation -> "-2"
    Unary {
        op: OperatorType,
        expr: Box<Expression>,
    },

    /// Binary expression, the most common -> "2 + 3"
    Binary {
        left: Box<Expression>,
        op: OperatorType,
        right: Box<Expression>,
    },

    /// A function call -> "sin(2)"
    Call {
        callee: String,
        args: Vec<Expression>,
    },
}

impl Expression {
    fn apply_unary_op(op: &OperatorType, n: f64) -> Result<f64> {
        match op {
            OperatorType::PLUS => Ok(n.abs()),
            OperatorType::MINUS => Ok(n.neg()),
            _ => Err(Error::invalid_unary_op(op)),
        }
    }

    fn apply_binary_op(op: &OperatorType, lhs: f64, rhs: f64) -> f64 {
        match op {
            OperatorType::PLUS => lhs + rhs,
            OperatorType::MINUS => lhs - rhs,
            OperatorType::MULTIPLY => lhs * rhs,
            OperatorType::DIVIDE => lhs / rhs,
            OperatorType::MODULO => lhs % rhs,
            OperatorType::POWER => lhs.powf(rhs),
        }
    }

    pub(crate) fn eval(&self, ctx: &Context) -> Result<f64> {
        match self {
            Self::Number(n) => Ok(*n),
            Self::Variable(var) => {
                let val = ctx.get_var(var);
                if let Some(val) = val {
                    Ok(val)
                } else {
                    panic!("Undefined variable '{}'", var);
                }
            }
            Self::Unary { op, expr } => {
                let res = expr.eval(ctx)?;
                Ok(Self::apply_unary_op(op, res)?)
            }
            Self::Binary { left, op, right } => {
                let left = left.eval(ctx)?;
                let right = right.eval(ctx)?;
                Ok(Self::apply_binary_op(op, left, right))
            }
            Self::Call { callee, args } => {
                // TODO: Better builtin functions
                match callee.as_str() {
                    "sin" => {
                        if args.len() != 1 {
                            panic!(
                                "Invalid amount of arguments. Expected: 1, but got: {}",
                                args.len()
                            );
                        }

                        let arg = args.first().unwrap().eval(ctx)?;
                        Ok(arg.sin())
                    }
                    _ => panic!("Undefined function: {}", &callee),
                }
            }
        }
    }
}
