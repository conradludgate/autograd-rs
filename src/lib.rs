#![feature(type_alias_impl_trait)]

use std::collections::HashMap;

pub mod floats;

pub type Float = f64;
pub struct Context {
    vars: HashMap<&'static str, Float>,
}

impl Context {
    pub fn new(vars: HashMap<&'static str, Float>) -> Self {
        Self { vars }
    }

    pub fn get_val(&self, name: &'static str) -> Float {
        *self.vars.get(name).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::{Context, floats::{FloatComp, var::Var}};

    #[test]
    fn quadratic() {
        let x: Var = "x".into();

        let y = x * x + x * 2.0 + 1.0;
        let y_ = y.diff(x);

        println!("{:?}", y);
        println!("{:?}", y_);

        let ctx = Context::new([("x", 4.0)].into_iter().collect());

        assert_eq!(y.eval(&ctx), 25.0);
        assert_eq!(y_.eval(&ctx), 10.0);
    }

    #[test]
    fn multi() {
        let x: Var = "x".into();
        let y: Var = "y".into();

        let z = x * x * y;
        let z_x = z.diff(x);
        let z_y = z.diff(y);

        println!("{:?}", z);
        println!("{:?}", z_x);
        println!("{:?}", z_y);

        let ctx = Context::new([("x", 4.0), ("y", 3.0)].into_iter().collect());

        assert_eq!(z.eval(&ctx), 48.0);
        assert_eq!(z_x.eval(&ctx), 24.0);
        assert_eq!(z_y.eval(&ctx), 16.0);
    }
}
