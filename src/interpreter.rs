use crate::Error;
use crate::Result;
use core::f64;
use std::collections::HashMap;

use crate::Expression;

pub type LexpFn = Box<dyn Fn(&[f64]) -> Result<f64>>;

macro_rules! impl_single_arg {
    ($fn_name:ident, $method:ident) => {
        fn $fn_name(args: &[f64]) -> Result<f64> {
            let arg = args.first().ok_or(Error::other(format!(
                "Not enough arguments, expected 1, but got 0"
            )))?;
            Ok(arg.$method())
        }
    };
}

#[derive(Default)]
pub struct Interpreter {
    globals: HashMap<String, f64>,
    fns: HashMap<String, LexpFn>,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut obj = Self::default();
        obj.define_var("PI".to_string(), f64::consts::PI);
        obj.define_fn("sin".to_string(), sin);
        obj.define_fn("cos".to_string(), cos);
        obj.define_fn("tan".to_string(), tan);
        obj.define_fn("max".to_string(), max);
        obj.define_fn("min".to_string(), min);
        obj.define_fn("ceil".to_string(), ceil);
        obj.define_fn("floor".to_string(), floor);
        obj.define_fn("round".to_string(), round);
        obj.define_fn("cbrt".to_string(), cbrt);
        obj
    }

    pub fn define_var(&mut self, name: String, value: f64) {
        self.globals.insert(name, value);
    }

    pub fn define_fn<F: Fn(&[f64]) -> Result<f64> + 'static>(&mut self, name: String, function: F) {
        self.fns.insert(name, Box::new(function));
    }

    pub(crate) fn get_var(&self, name: &str) -> Option<f64> {
        self.globals.get(name).copied()
    }

    pub(crate) fn call_fn(&self, name: &str, args: &[f64]) -> Option<Result<f64>> {
        let func = self.fns.get(name)?;
        Some(func(args))
    }
}

impl Interpreter {
    pub fn evaluate(&self, expr: Expression) -> Result<f64> {
        expr.eval(self)
    }
}

// builtins
impl_single_arg!(sin, sin);
impl_single_arg!(cos, cos);
impl_single_arg!(tan, tan);
impl_single_arg!(ceil, ceil);
impl_single_arg!(floor, floor);
impl_single_arg!(cbrt, cbrt);
impl_single_arg!(round, round);

fn max(args: &[f64]) -> Result<f64> {
    if args.len() != 2 {
        return Err(Error::other(format!(
            "Not enough arguments, expected 2, but got {}",
            args.len()
        )));
    }

    let arg1 = args[0];
    let arg2 = args[1];
    Ok(arg1.max(arg2))
}

fn min(args: &[f64]) -> Result<f64> {
    if args.len() != 2 {
        return Err(Error::other(format!(
            "Not enough arguments, expected 2, but got {}",
            args.len()
        )));
    }

    let arg1 = args[0];
    let arg2 = args[1];

    Ok(arg1.min(arg2))
}
