/// Simple HTML escaping, so strings can be safely rendered.
///
/// ```rust
/// # use cmark::html::esc;
///
/// let mut buf = String::new();
/// esc(r#"<hello world="attribute" />"#, &mut buf).unwrap();
/// assert_eq!(buf, "&lt;hello world=&quot;attribute&quot; /&gt;");
/// ```
pub fn esc(html: &str, writer: &mut dyn std::fmt::Write) -> std::fmt::Result {
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
