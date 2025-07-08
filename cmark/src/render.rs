use crate::parse_error::ParseError;

pub trait Render {
    fn render_into<W: std::io::Write>(&self, _writer: &mut W) -> Result<(), ParseError> {
        return Ok(());
    }

    fn render(&self) -> Result<String, ParseError> {
        let mut buf: Vec<u8> = vec![];
        self.render_into(&mut buf)?;
        return Ok(String::from_utf8(buf).unwrap());
    }
}
