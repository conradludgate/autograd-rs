use self::{add::FloatAdd, div::FloatDiv, mul::FloatMul, sub::FloatSub};
use super::FloatComp;
use std::ops::{Add, Div, Mul, Sub};

pub mod add;
pub mod div;
pub mod mul;
pub mod sub;

macro_rules! impl_float_like {
    ($T:ident) => {
        impl<F: FloatComp, X: FloatComp, Y: FloatComp> Add<F> for $T<X, Y> {
            type Output = FloatAdd<Self, F>;

            fn add(self, rhs: F) -> Self::Output {
                FloatAdd::new(self, rhs)
            }
        }

        impl<F: FloatComp, X: FloatComp, Y: FloatComp> Sub<F> for $T<X, Y> {
            type Output = FloatSub<Self, F>;

            fn sub(self, rhs: F) -> Self::Output {
                FloatSub::new(self, rhs)
            }
        }

        impl<F: FloatComp, X: FloatComp, Y: FloatComp> Mul<F> for $T<X, Y> {
            type Output = FloatMul<Self, F>;

            fn mul(self, rhs: F) -> Self::Output {
                FloatMul::new(self, rhs)
            }
        }

        impl<F: FloatComp, X: FloatComp, Y: FloatComp> Div<F> for $T<X, Y> {
            type Output = FloatDiv<Self, F>;

            fn div(self, rhs: F) -> Self::Output {
                FloatDiv::new(self, rhs)
            }
        }
    };
}

impl_float_like!(FloatAdd);
impl_float_like!(FloatSub);
impl_float_like!(FloatMul);
impl_float_like!(FloatDiv);
