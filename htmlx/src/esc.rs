use std::fmt::{Result, Write};

/// Simple HTML escaping, so strings can be safely rendered.
///
/// ```rust
/// # use htmlx::esc;
///
/// let mut buf = String::new();
/// esc(r#"<hello world="attribute" />"#, &mut buf).unwrap();
/// assert_eq!(buf, "&lt;hello world=&quot;attribute&quot; /&gt;");
/// ```
pub fn esc<W: Write>(html: &str, writer: &mut W) -> Result {
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
