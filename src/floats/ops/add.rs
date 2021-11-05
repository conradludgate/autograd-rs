use crate::{Context, Float, floats::{FloatComp, var::Var}};
use std::fmt::{Debug, Display};

pub trait FloatAddComp: FloatComp + Sized {
    fn add<Rhs: FloatComp>(self, rhs: Rhs) -> FloatAdd<Self, Rhs>;
}

impl<F: FloatComp> FloatAddComp for F {
    fn add<Rhs: FloatComp>(self, rhs: Rhs) -> FloatAdd<Self, Rhs> {
        FloatAdd::new(self, rhs)
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct FloatAdd<X: FloatComp, Y: FloatComp>(X, Y);

impl<X: FloatComp, Y: FloatComp> Display for FloatAdd<X, Y> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(z) = self.constant() {
            write!(f, "{:?}", z)
        } else {
            match (self.0.constant(), self.1.constant()) {
                (Some(x), _) if x == 0.0 => write!(f, "{}", self.1),
                (_, Some(y)) if y == 0.0 => write!(f, "{}", self.0),
                _ => write!(f, "({} + {})", self.0, self.1),
            }
        }
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

    type Diff = impl FloatComp;
    fn diff(&self, var: Var) -> Self::Diff {
        self.0.diff(var).add(self.1.diff(var))
    }

    fn constant(&self) -> Option<Float> {
        match (self.0.constant(), self.1.constant()) {
            (Some(x), Some(y)) => Some(x + y),
            _ => None,
        }
    }
}
