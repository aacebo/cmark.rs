use std::fmt::{Result, Write};

use super::HTMLElement;

macro_rules! primitive_render {
    ($t:ty) => {
        impl HTMLElement for $t {
            fn render_into(&self, writer: &mut dyn Write) -> Result {
                write!(writer, "{}", self)
            }
        }
    };
}

primitive_render!(f32);
primitive_render!(f64);
primitive_render!(i128);
primitive_render!(i16);
primitive_render!(i32);
primitive_render!(i64);
primitive_render!(i8);
primitive_render!(isize);
primitive_render!(u128);
primitive_render!(u16);
primitive_render!(u32);
primitive_render!(u64);
primitive_render!(u8);
primitive_render!(usize);
primitive_render!(bool);
