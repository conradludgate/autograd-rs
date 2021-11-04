use std::ops::{Add, Div, Mul, Sub};

use crate::{
    float_ops::{add::FloatAdd, div::FloatDiv, mul::FloatMul, pow::FloatPow, sub::FloatSub},
    Context, Float, FloatComp, FloatLike, Pow,
};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Var {
    name: &'static str,
}

impl From<&'static str> for Var {
    fn from(name: &'static str) -> Self {
        Self {
            name
        }
    }
}
impl FloatLike for Var {}

macro_rules! impl_float_like {
    ($T:ident) => {
        impl<F: FloatComp> Add<F> for $T {
            type Output = FloatAdd;

            fn add(self, rhs: F) -> Self::Output {
                FloatAdd::new(self, rhs)
            }
        }

        impl<F: FloatComp> Sub<F> for $T {
            type Output = FloatSub;

            fn sub(self, rhs: F) -> Self::Output {
                FloatSub::new(self, rhs)
            }
        }

        impl<F: FloatComp> Mul<F> for $T {
            type Output = FloatMul;

            fn mul(self, rhs: F) -> Self::Output {
                FloatMul::new(self, rhs)
            }
        }
        
        impl<F: FloatComp> Div<F> for $T {
            type Output = FloatDiv;

            fn div(self, rhs: F) -> Self::Output {
                FloatDiv::new(self, rhs)
            }
        }

        impl<F: FloatComp> Pow<F> for $T {
            type Output = FloatPow;

            fn pow(self, exp: F) -> Self::Output {
                FloatPow::new(self, exp)
            }
        }
    };
}

impl_float_like!(Var);
impl_float_like!(FloatAdd);
impl_float_like!(FloatSub);
impl_float_like!(FloatMul);
impl_float_like!(FloatDiv);
impl_float_like!(FloatPow);

impl FloatComp for Var {
    fn eval(&self, ctx: &Context) -> Float {
        ctx.get_val(self.name)
    }

    fn diff(&self, _: &Context, var: Var) -> Float {
        if *self == var { 1.0 } else { 0.0 }
    }
}
