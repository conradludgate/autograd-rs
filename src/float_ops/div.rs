use crate::{Context, Float, FloatComp, floats::var::Var};
use std::{fmt::Debug, ops::Sub};

use super::{mul::{FloatMul, FloatMulComp}, sub::FloatSub};

pub trait FloatDivComp: FloatComp + Sized {
    fn div<Rhs: FloatComp>(self, rhs: Rhs) -> FloatDiv<Self, Rhs>;
}

impl<F: FloatComp> FloatDivComp for F {
    fn div<Rhs: FloatComp>(self, rhs: Rhs) -> FloatDiv<Self, Rhs> {
        FloatDiv::new(self, rhs)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct FloatDiv<X: FloatComp, Y: FloatComp>(X, Y);

impl<X: FloatComp, Y: FloatComp> FloatDiv<X, Y> {
    pub fn new(x: X, y: Y) -> Self {
        Self(x, y)
    }
}

impl<X: FloatComp, Y: FloatComp> FloatComp for FloatDiv<X, Y> {
    fn eval(&self, ctx: &Context) -> Float {
        self.0.eval(ctx) / self.0.eval(ctx)
    }

    type Diff = FloatDiv<FloatSub<FloatMul<Y, X::Diff>, FloatMul<X, Y::Diff>>, FloatMul<Y, Y>>;
    fn diff(&self, var: Var) -> Self::Diff {
        let x = self.0;
        let y = self.1;
        let x1 = self.0.diff(var);
        let y1 = self.1.diff(var);

        y.mul(x1).sub(x.mul(y1)).div(y.mul(y))
    }
}
