use crate::CMarkError;

pub trait Render: std::fmt::Debug {
    fn render_into(&self, _writer: &mut dyn std::fmt::Write) -> Result<(), CMarkError>;
    fn render(&self) -> Result<String, CMarkError> {
        let mut buf = String::new();
        self.render_into(&mut buf)?;
        return Ok(buf);
    }
}
