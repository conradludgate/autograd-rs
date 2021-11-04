use crate::{Context, Float, FloatComp, floats::var::Var};
use std::fmt::Debug;

#[derive(Debug)]
pub struct FloatMul(Box<dyn FloatComp>, Box<dyn FloatComp>);

impl FloatMul {
    pub fn new(x: impl FloatComp, y: impl FloatComp) -> Self {
        Self(Box::new(x), Box::new(y))
    }
}
impl FloatComp for FloatMul {
    fn eval(&self, ctx: &Context) -> Float {
        self.0.eval(ctx) * self.0.eval(ctx)
    }

    fn diff(&self, ctx: &Context, var: Var) -> Float {
        let x = self.0.eval(ctx);
        let y = self.1.eval(ctx);
        let x1 = self.0.diff(ctx, var);
        let y1 = self.1.diff(ctx, var);

        y * x1 + x * y1
    }
}
