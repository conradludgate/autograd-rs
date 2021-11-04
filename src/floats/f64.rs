use crate::{Context, FloatComp, FloatLike};

use super::var::Var;

impl FloatLike for f64 {}

impl FloatComp for f64 {
    fn eval(&self, _: &Context) -> f64 {
        *self
    }

    type Diff = f64;
    fn diff(&self, _: Var) -> f64 {
        0.0
    }
}
