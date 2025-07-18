use common::collections::Map;

use crate::{Render, RenderOptions, html::esc};

/// Attributes
///
/// html element attributes
pub type Attributes = Map<String, String>;

impl Render for Map<String, String> {
    fn render_into(
        &self,
        writer: &mut dyn std::fmt::Write,
        _options: &RenderOptions,
    ) -> Result<(), std::fmt::Error> {
        for pair in self.iter() {
            write!(writer, " {}=\"", pair.key)?;
            esc(&pair.value, writer)?;
            write!(writer, "\"")?;
        }

        return Ok(());
    }
}
