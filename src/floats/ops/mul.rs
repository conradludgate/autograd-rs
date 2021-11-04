use crate::{Context, Float, floats::{FloatComp, var::Var}};
use std::fmt::Debug;

pub trait FloatMulComp: FloatComp + Sized {
    fn mul<Rhs: FloatComp>(self, rhs: Rhs) -> FloatMul<Self, Rhs>;
}

impl<F: FloatComp> FloatMulComp for F {
    fn mul<Rhs: FloatComp>(self, rhs: Rhs) -> FloatMul<Self, Rhs> {
        FloatMul::new(self, rhs)
    }
}

#[derive(Copy, Clone)]
pub struct FloatMul<X: FloatComp, Y: FloatComp>(X, Y);

impl<X: FloatComp, Y: FloatComp> Debug for FloatMul<X, Y> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?} * {:?})", self.0, self.1)
    }
}

impl<X: FloatComp, Y: FloatComp> FloatMul<X, Y> {
    pub fn new(x: X, y: Y) -> Self {
        Self(x, y)
    }
}
impl<X: FloatComp, Y: FloatComp> FloatComp for FloatMul<X, Y> {
    fn eval(&self, ctx: &Context) -> Float {
        self.0.eval(ctx) * self.1.eval(ctx)
    }

    type Diff = impl FloatComp;
    fn diff(&self, var: Var) -> Self::Diff {
        let x = self.0;
        let y = self.1;
        let x1 = self.0.diff(var);
        let y1 = self.1.diff(var);

        y.mul(x1) + x.mul(y1)
    }
}
