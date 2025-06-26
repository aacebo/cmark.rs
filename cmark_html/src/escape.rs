use std::fmt::{Result, Write};

/// Escape any HTML string
pub fn escape(html: &str, writer: &mut dyn Write) -> Result {
    for c in html.chars() {
        match c {
            '>' => write!(writer, "&gt;")?,
            '<' => write!(writer, "&lt;")?,
            '"' => write!(writer, "&quot;")?,
            '&' => write!(writer, "&amp;")?,
            '\'' => write!(writer, "&apos;")?,
            c => writer.write_char(c)?,
        };
    }

    return Ok(());
}
