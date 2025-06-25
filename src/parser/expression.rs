use std::ops::Neg;

use crate::{eval::Context, lexer::OperatorType};

#[derive(Debug, PartialEq)]
pub enum Expression {
    Number(f64),
    Variable(String), // for future
    Unary {
        op: OperatorType,
        expr: Box<Expression>,
    },

    Binary {
        left: Box<Expression>,
        op: OperatorType,
        right: Box<Expression>,
    },

    Call {
        callee: String,
        args: Vec<Expression>,
    },
}

impl Expression {
    fn apply_unary_op(op: &OperatorType, n: f64) -> f64 {
        match op {
            OperatorType::PLUS => n.abs(),
            OperatorType::MINUS => n.neg(),
            _ => panic!(
                "Invalid usage of unary operator. Expected '+' or '-', but got: {:?}",
                op
            ),
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

    pub fn eval(&self, ctx: &Context) -> f64 {
        match self {
            Self::Number(n) => *n,
            Self::Variable(var) => {
                let val = ctx.get_var(var);
                if let Some(val) = val {
                    val
                } else {
                    panic!("Undefined variable '{}'", var);
                }
            }
            Self::Unary { op, expr } => {
                let res = expr.eval(ctx);
                Self::apply_unary_op(op, res)
            }
            Self::Binary { left, op, right } => {
                let left = left.eval(ctx);
                let right = right.eval(ctx);
                Self::apply_binary_op(op, left, right)
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

                        let arg = args.first().unwrap().eval(ctx);
                        arg.sin()
                    }
                    _ => panic!("Undefined function: {}", &callee),
                }
            }
        }
    }
}
