use std::fmt::{Result, Write};

pub trait Render: Sized {
    fn render_into<W: Write>(&self, _writer: &mut W) -> Result {
        return Ok(());
    }

    fn render(&self) -> String {
        let mut buf = String::new();
        self.render_into(&mut buf).unwrap();
        return buf;
    }
}