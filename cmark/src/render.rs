use crate::html::esc;

pub trait Render: std::fmt::Debug {
    fn render_into(&self, _writer: &mut dyn std::fmt::Write) -> Result<(), std::fmt::Error>;
    fn render(&self) -> Result<String, std::fmt::Error> {
        let mut buf = String::new();
        self.render_into(&mut buf)?;
        return Ok(buf);
    }
}

impl Render for () {
    fn render_into(&self, _writer: &mut dyn std::fmt::Write) -> Result<(), std::fmt::Error> {
        return Ok(());
    }
}

impl<A: Render, B: Render> Render for (A, B) {
    fn render_into(&self, writer: &mut dyn std::fmt::Write) -> Result<(), std::fmt::Error> {
        self.0.render_into(writer)?;
        return self.1.render_into(writer);
    }
}

impl<A: Render, B: Render, C: Render> Render for (A, B, C) {
    fn render_into(&self, writer: &mut dyn std::fmt::Write) -> Result<(), std::fmt::Error> {
        self.0.render_into(writer)?;
        self.1.render_into(writer)?;
        return self.2.render_into(writer);
    }
}

impl<T: Render> Render for Option<T> {
    fn render_into(&self, writer: &mut dyn std::fmt::Write) -> Result<(), std::fmt::Error> {
        return match self {
            None => Ok(()),
            Some(x) => x.render_into(writer),
        };
    }
}

impl<T: Render> Render for Vec<T> {
    fn render_into(&self, writer: &mut dyn std::fmt::Write) -> Result<(), std::fmt::Error> {
        for elem in self {
            elem.render_into(writer)?;
        }

        return Ok(());
    }
}

impl<O: Render, E: Render> Render for std::result::Result<O, E> {
    fn render_into(&self, writer: &mut dyn std::fmt::Write) -> Result<(), std::fmt::Error> {
        return match self {
            Ok(o) => o.render_into(writer),
            Err(e) => e.render_into(writer),
        };
    }
}

impl Render for String {
    fn render_into(&self, writer: &mut dyn std::fmt::Write) -> Result<(), std::fmt::Error> {
        return esc(&self, writer);
    }
}

impl Render for &str {
    fn render_into(&self, writer: &mut dyn std::fmt::Write) -> Result<(), std::fmt::Error> {
        return esc(&self, writer);
    }
}

impl Render for std::borrow::Cow<'_, str> {
    fn render_into(&self, writer: &mut dyn std::fmt::Write) -> Result<(), std::fmt::Error> {
        return esc(&self, writer);
    }
}
