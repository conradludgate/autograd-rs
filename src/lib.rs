use std::{
    collections::HashMap,
    fmt::Debug,
    ops::{Add, Div, Mul, Sub},
};

use floats::var::Var;

mod float_ops {
    pub mod add;
    pub mod div;
    pub mod mul;
    pub mod pow;
    pub mod sub;
}

pub mod floats {
    pub mod f64;
    pub mod var;
}

pub type Float = f64;
pub struct Context {
    vars: HashMap<&'static str, Float>,
}

impl Context {
    pub fn new(vars: HashMap<&'static str, Float>) -> Self {
        Self { vars }
    }

    pub fn get_val(&self, name: &'static str) -> Float {
        *self.vars.get(name).unwrap()
    }
}

pub trait FloatLike: Sized + Mul + Add + Sub + Div + Pow + Copy + FloatComp {}

pub trait Pow<Exp = Self> {
    type Output;
    fn pow(self, exp: Exp) -> Self::Output;
}

pub trait FloatComp: Debug + 'static {
    fn eval(&self, ctx: &Context) -> Float;
    fn diff(&self, ctx: &Context, var: Var) -> Float;
}

#[cfg(test)]
mod tests {
    use crate::{Context, floats::var::Var, FloatComp};

    #[test]
    fn it_works() {
        let x: Var = "x".into();

        let y = x * x + x * 2.0 + 1.0;

        let ctx = Context::new([("x", 4.0)].into_iter().collect());

        assert_eq!(y.eval(&ctx), 25.0);
        assert_eq!(y.diff(&ctx, x), 10.0);
    }
}