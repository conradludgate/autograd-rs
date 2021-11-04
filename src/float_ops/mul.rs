use crate::{Context, Float, FloatComp, floats::var::Var};
use std::fmt::Debug;

use super::add::{FloatAdd, FloatAddComp};

pub trait FloatMulComp: FloatComp + Sized {
    fn mul<Rhs: FloatComp>(self, rhs: Rhs) -> FloatMul<Self, Rhs>;
}

impl<F: FloatComp> FloatMulComp for F {
    fn mul<Rhs: FloatComp>(self, rhs: Rhs) -> FloatMul<Self, Rhs> {
        FloatMul::new(self, rhs)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct FloatMul<X: FloatComp, Y: FloatComp>(X, Y);

impl<X: FloatComp, Y: FloatComp> FloatMul<X, Y> {
    pub fn new(x: X, y: Y) -> Self {
        Self(x, y)
    }
}
impl<X: FloatComp, Y: FloatComp> FloatComp for FloatMul<X, Y> {
    fn eval(&self, ctx: &Context) -> Float {
        self.0.eval(ctx) * self.0.eval(ctx)
    }

    type Diff = FloatAdd<FloatMul<Y, X::Diff>, FloatMul<X, Y::Diff>>;
    fn diff(&self, var: Var) -> Self::Diff {
        let x = self.0;
        let y = self.1;
        let x1 = self.0.diff(var);
        let y1 = self.1.diff(var);

        y.mul(x1).add(x.mul(y1))
    }
}
