use crate::{Context, Float, floats::{FloatComp, var::Var}};
use std::fmt::{Debug, Display};

use super::mul::FloatMulComp;

pub trait FloatDivComp: FloatComp + Sized {
    fn div<Rhs: FloatComp>(self, rhs: Rhs) -> FloatDiv<Self, Rhs>;
}

impl<F: FloatComp> FloatDivComp for F {
    fn div<Rhs: FloatComp>(self, rhs: Rhs) -> FloatDiv<Self, Rhs> {
        FloatDiv::new(self, rhs)
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct FloatDiv<X: FloatComp, Y: FloatComp>(X, Y);

impl<X: FloatComp, Y: FloatComp> Display for FloatDiv<X, Y> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(z) = self.constant() {
            write!(f, "{:?}", z)
        } else {
            match (self.0.constant(), self.1.constant()) {
                (_, Some(y)) if y == 1.0 => write!(f, "{}", self.0),
                _ => write!(f, "({} / {})", self.0, self.1),
            }
        }
    }
}

impl<X: FloatComp, Y: FloatComp> FloatDiv<X, Y> {
    pub fn new(x: X, y: Y) -> Self {
        Self(x, y)
    }
}

impl<X: FloatComp, Y: FloatComp> FloatComp for FloatDiv<X, Y> {
    fn eval(&self, ctx: &Context) -> Float {
        self.0.eval(ctx) / self.1.eval(ctx)
    }

    type Diff = impl FloatComp;
    fn diff(&self, var: Var) -> Self::Diff {
        let x = self.0;
        let y = self.1;
        let x1 = self.0.diff(var);
        let y1 = self.1.diff(var);

        (y.mul(x1) - x.mul(y1)) / (y.mul(y))
    }

    fn constant(&self) -> Option<Float> {
        match (self.0.constant(), self.1.constant()) {
            (Some(x), Some(y)) => Some(x / y),
            (Some(x), _) if x == 0.0 => Some(0.0),
            (_, Some(y)) if y == 0.0 => Some(f64::INFINITY),
            _ => None,
        }
    }
}
