use crate::{Context, Float, FloatComp, floats::var::Var};

pub trait FloatSubComp: FloatComp + Sized {
    fn sub<Rhs: FloatComp>(self, rhs: Rhs) -> FloatSub<Self, Rhs>;
}

impl<F: FloatComp> FloatSubComp for F {
    fn sub<Rhs: FloatComp>(self, rhs: Rhs) -> FloatSub<Self, Rhs> {
        FloatSub::new(self, rhs)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct FloatSub<X: FloatComp, Y: FloatComp>(X, Y);

impl<X: FloatComp, Y: FloatComp> FloatSub<X, Y> {
    pub fn new(x: X, y: Y) -> Self {
        Self(x, y)
    }
}

impl<X: FloatComp, Y: FloatComp> FloatComp for FloatSub<X, Y> {
    fn eval(&self, ctx: &Context) -> Float {
        self.0.eval(ctx) - self.1.eval(ctx)
    }

    type Diff = FloatSub<X::Diff, Y::Diff>;
    fn diff(&self, var: Var) -> Self::Diff {
        self.0.diff(var).sub(self.1.diff(var))
    }
}
