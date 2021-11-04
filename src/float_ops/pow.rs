use crate::{Context, Float, FloatComp, floats::var::Var};
use std::fmt::Debug;

#[derive(Debug)]
pub struct FloatPow(Box<dyn FloatComp>, Box<dyn FloatComp>);

impl FloatPow {
    pub fn new(x: impl FloatComp, y: impl FloatComp) -> Self {
        Self(Box::new(x), Box::new(y))
    }
}

impl FloatComp for FloatPow {
    fn eval(&self, ctx: &Context) -> Float {
        self.0.eval(ctx).powf(self.1.eval(ctx))
    }

    fn diff(&self, ctx: &Context, var: Var) -> Float {
        self.0.diff(ctx, var) + self.1.diff(ctx, var)
    }
}
