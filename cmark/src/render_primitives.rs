use crate::Render;
use std::fmt::{Result, Write};

macro_rules! define_primitive {
    ($t:ty) => {
        impl Render for $t {
            fn render_into(&self, writer: &mut dyn Write) -> Result {
                return write!(writer, "{}", self);
            }
        }
    };
}

define_primitive!(f32);
define_primitive!(f64);
define_primitive!(i128);
define_primitive!(i16);
define_primitive!(i32);
define_primitive!(i64);
define_primitive!(i8);
define_primitive!(isize);
define_primitive!(u128);
define_primitive!(u16);
define_primitive!(u32);
define_primitive!(u64);
define_primitive!(u8);
define_primitive!(usize);
define_primitive!(bool);
