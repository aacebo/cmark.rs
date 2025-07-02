use crate::Element;
use std::fmt::{Result, Write};

macro_rules! primitive_render_impl {
    ($t:ty) => {
        impl Element for $t {
            fn render_into<W: Write>(&self, writer: &mut W) -> Result {
                return write!(writer, "{}", self);
            }
        }
    };
}

primitive_render_impl!(f32);
primitive_render_impl!(f64);
primitive_render_impl!(i128);
primitive_render_impl!(i16);
primitive_render_impl!(i32);
primitive_render_impl!(i64);
primitive_render_impl!(i8);
primitive_render_impl!(isize);
primitive_render_impl!(u128);
primitive_render_impl!(u16);
primitive_render_impl!(u32);
primitive_render_impl!(u64);
primitive_render_impl!(u8);
primitive_render_impl!(usize);
primitive_render_impl!(bool);
