use crate::ParseError;

pub trait Render {
    fn render_into(&self, _writer: &mut dyn std::io::Write) -> Result<(), ParseError>;
    fn render(&self) -> Result<String, ParseError> {
        let mut buf: Vec<u8> = vec![];
        self.render_into(&mut buf)?;
        return Ok(String::from_utf8(buf).unwrap());
    }
}
