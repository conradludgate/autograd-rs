use crate::{Context, Float, FloatComp, floats::var::Var};
use std::fmt::Debug;

pub trait FloatAddComp: FloatComp + Sized {
    fn add<Rhs: FloatComp>(self, rhs: Rhs) -> FloatAdd<Self, Rhs>;
}

impl<F: FloatComp> FloatAddComp for F {
    fn add<Rhs: FloatComp>(self, rhs: Rhs) -> FloatAdd<Self, Rhs> {
        FloatAdd::new(self, rhs)
    }
}

#[derive(Copy, Clone)]
pub struct FloatAdd<X: FloatComp, Y: FloatComp>(X, Y);

impl<X: FloatComp, Y: FloatComp> Debug for FloatAdd<X, Y> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?} + {:?})", self.0, self.1)
    }
}

impl<X: FloatComp, Y: FloatComp> FloatAdd<X, Y> {
    pub fn new(x: X, y: Y) -> Self {
        Self(x, y)
    }
}

impl<X: FloatComp, Y: FloatComp> FloatComp for FloatAdd<X, Y> {
    fn eval(&self, ctx: &Context) -> Float {
        self.0.eval(ctx) + self.1.eval(ctx)
    }

    type Diff = FloatAdd<X::Diff, Y::Diff>;
    fn diff(&self, var: Var) -> Self::Diff {
        self.0.diff(var).add(self.1.diff(var))
    }
}
