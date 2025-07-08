pub trait Render: Sized {
    type Error: std::error::Error;

    fn render_into<W: std::io::Write>(&self, _writer: &mut W) -> Result<(), Self::Error> {
        return Ok(());
    }

    fn render(&self) -> Result<String, Self::Error> {
        let mut buf: Vec<u8> = vec![];
        self.render_into(&mut buf)?;
        return Ok(String::from_utf8(buf).unwrap());
    }
}