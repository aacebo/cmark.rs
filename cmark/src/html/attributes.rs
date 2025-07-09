use common::collections::Map;

use crate::{Render, html::esc};

/// Attributes
///
/// html element attributes
pub type Attributes<'a> = Map<&'a str, String>;

impl<'a> Render for Map<&'a str, String> {
    fn render_into(&self, writer: &mut dyn std::fmt::Write) -> Result<(), std::fmt::Error> {
        for pair in self.iter() {
            write!(writer, " {}=\"", pair.key)?;
            esc(&pair.value, writer)?;
            write!(writer, "\"")?;
        }

        return Ok(());
    }
}
