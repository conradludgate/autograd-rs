use crate::{Context};

use super::{FloatComp, var::Var};

impl FloatComp for f64 {
    fn eval(&self, _: &Context) -> f64 {
        *self
    }

    type Diff = f64;
    fn diff(&self, _: Var) -> f64 {
        0.0
    }
}
