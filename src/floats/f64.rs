use crate::{Context, Float};

use super::{FloatComp, var::Var};

impl FloatComp for Float {
    fn eval(&self, _: &Context) -> Float {
        *self
    }

    type Diff = Float;
    fn diff(&self, _: Var) -> Float {
        0.0
    }

    fn constant(&self) -> Option<Float> {
        Some(*self)
    }
}
