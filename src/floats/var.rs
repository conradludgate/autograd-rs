use std::{fmt::Debug, ops::{Add, Div, Mul, Sub}};

use crate::{
    floats::ops::{add::FloatAdd, div::FloatDiv, mul::FloatMul, sub::FloatSub},
    Context, Float,
};

use super::FloatComp;

#[derive(Copy, Clone, PartialEq)]
pub struct Var {
    name: &'static str,
}

impl Debug for Var {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl From<&'static str> for Var {
    fn from(name: &'static str) -> Self {
        Self {
            name
        }
    }
}

macro_rules! impl_float_like {
    ($T:ident) => {
        impl<F: FloatComp> Add<F> for $T {
            type Output = FloatAdd<$T, F>;

            fn add(self, rhs: F) -> Self::Output {
                FloatAdd::new(self, rhs)
            }
        }

        impl<F: FloatComp> Sub<F> for $T {
            type Output = FloatSub<$T, F>;

            fn sub(self, rhs: F) -> Self::Output {
                FloatSub::new(self, rhs)
            }
        }

        impl<F: FloatComp> Mul<F> for $T {
            type Output = FloatMul<$T, F>;

            fn mul(self, rhs: F) -> Self::Output {
                FloatMul::new(self, rhs)
            }
        }

        impl<F: FloatComp> Div<F> for $T {
            type Output = FloatDiv<$T, F>;

            fn div(self, rhs: F) -> Self::Output {
                FloatDiv::new(self, rhs)
            }
        }
    };
}

impl_float_like!(Var);

impl FloatComp for Var {
    fn eval(&self, ctx: &Context) -> Float {
        ctx.get_val(self.name)
    }

    type Diff = f64;
    fn diff(&self, var: Var) -> Self::Diff {
        if *self == var { 1.0 } else { 0.0 }
    }
}
