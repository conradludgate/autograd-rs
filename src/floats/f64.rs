use crate::{Context, FloatComp, FloatLike, Pow};

use super::var::Var;

impl Pow for f64 {
    type Output = Self;
    fn pow(self, exp: Self) -> Self::Output {
        self.powf(exp)
    }
}

impl FloatLike for f64 {}

impl FloatComp for f64 {
    fn eval(&self, _: &Context) -> f64 {
        *self
    }

    fn diff(&self, _: &Context, _: Var) -> f64 {
        0.0
    }
}
