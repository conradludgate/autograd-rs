use std::fmt::{Debug, Display};

use crate::{
    floats::{var::Var, FloatComp},
    Context, Float,
};

pub trait FloatSubComp: FloatComp + Sized {
    fn sub<Rhs: FloatComp>(self, rhs: Rhs) -> FloatSub<Self, Rhs>;
}

impl<F: FloatComp> FloatSubComp for F {
    fn sub<Rhs: FloatComp>(self, rhs: Rhs) -> FloatSub<Self, Rhs> {
        FloatSub::new(self, rhs)
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct FloatSub<X: FloatComp, Y: FloatComp>(X, Y);

impl<X: FloatComp, Y: FloatComp> Display for FloatSub<X, Y> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(z) = self.constant() {
            write!(f, "{:?}", z)
        } else {
            match (self.0.constant(), self.1.constant()) {
                (Some(x), _) if x == 0.0 => write!(f, "-{}", self.1),
                (_, Some(y)) if y == 0.0 => write!(f, "{}", self.0),
                _ => write!(f, "({} - {})", self.0, self.1),
            }
        }
    }
}

impl<X: FloatComp, Y: FloatComp> FloatSub<X, Y> {
    pub fn new(x: X, y: Y) -> Self {
        Self(x, y)
    }
}

impl<X: FloatComp, Y: FloatComp> FloatComp for FloatSub<X, Y> {
    fn eval(&self, ctx: &Context) -> Float {
        self.0.eval(ctx) - self.1.eval(ctx)
    }

    type Diff = impl FloatComp;
    fn diff(&self, var: Var) -> Self::Diff {
        self.0.diff(var).sub(self.1.diff(var))
    }

    fn constant(&self) -> Option<Float> {
        self.0.constant().zip(self.1.constant()).map(|(x, y)| x - y)
    }
}
