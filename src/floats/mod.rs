use std::fmt::{Debug, Display};

use crate::{Context, Float};

use self::var::Var;

pub mod f64;
pub mod ops;
pub mod var;

pub trait FloatComp: Sized + Copy + Debug + Display + 'static {
    fn eval(&self, ctx: &Context) -> Float;

    type Diff: FloatComp;
    fn diff(&self, var: Var) -> Self::Diff;

    fn constant(&self) -> Option<Float>;
}
