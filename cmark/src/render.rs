use crate::{RenderOptions, html::esc};

pub trait Render: std::fmt::Debug {
    fn render_into(
        &self,
        writer: &mut dyn std::fmt::Write,
        options: &RenderOptions,
    ) -> Result<(), std::fmt::Error>;
    fn render(&self, options: &RenderOptions) -> Result<String, std::fmt::Error> {
        let mut buf = String::new();
        self.render_into(&mut buf, options)?;
        return Ok(buf);
    }
}

impl Render for () {
    fn render_into(
        &self,
        _writer: &mut dyn std::fmt::Write,
        _options: &RenderOptions,
    ) -> Result<(), std::fmt::Error> {
        return Ok(());
    }
}

impl<A: Render, B: Render> Render for (A, B) {
    fn render_into(
        &self,
        writer: &mut dyn std::fmt::Write,
        options: &RenderOptions,
    ) -> Result<(), std::fmt::Error> {
        self.0.render_into(writer, options)?;
        return self.1.render_into(writer, options);
    }
}

impl<A: Render, B: Render, C: Render> Render for (A, B, C) {
    fn render_into(
        &self,
        writer: &mut dyn std::fmt::Write,
        options: &RenderOptions,
    ) -> Result<(), std::fmt::Error> {
        self.0.render_into(writer, options)?;
        self.1.render_into(writer, options)?;
        return self.2.render_into(writer, options);
    }
}

impl<T: Render> Render for Option<T> {
    fn render_into(
        &self,
        writer: &mut dyn std::fmt::Write,
        options: &RenderOptions,
    ) -> Result<(), std::fmt::Error> {
        return match self {
            None => Ok(()),
            Some(x) => x.render_into(writer, options),
        };
    }
}

impl<T: Render> Render for Vec<T> {
    fn render_into(
        &self,
        writer: &mut dyn std::fmt::Write,
        options: &RenderOptions,
    ) -> Result<(), std::fmt::Error> {
        for elem in self {
            elem.render_into(writer, options)?;
        }

        return Ok(());
    }
}

impl<O: Render, E: Render> Render for std::result::Result<O, E> {
    fn render_into(
        &self,
        writer: &mut dyn std::fmt::Write,
        options: &RenderOptions,
    ) -> Result<(), std::fmt::Error> {
        return match self {
            Ok(o) => o.render_into(writer, options),
            Err(e) => e.render_into(writer, options),
        };
    }
}

impl Render for String {
    fn render_into(
        &self,
        writer: &mut dyn std::fmt::Write,
        _options: &RenderOptions,
    ) -> Result<(), std::fmt::Error> {
        return esc(&self, writer);
    }
}

impl Render for &str {
    fn render_into(
        &self,
        writer: &mut dyn std::fmt::Write,
        _options: &RenderOptions,
    ) -> Result<(), std::fmt::Error> {
        return esc(&self, writer);
    }
}

impl Render for std::borrow::Cow<'_, str> {
    fn render_into(
        &self,
        writer: &mut dyn std::fmt::Write,
        _options: &RenderOptions,
    ) -> Result<(), std::fmt::Error> {
        return esc(&self, writer);
    }
}

macro_rules! define_primitive {
    ($t:ty) => {
        impl Render for $t {
            fn render_into(
                &self,
                writer: &mut dyn std::fmt::Write,
                _options: &RenderOptions,
            ) -> Result<(), std::fmt::Error> {
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
