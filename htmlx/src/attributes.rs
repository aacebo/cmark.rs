use std::{
    borrow::Cow,
    fmt::{Result, Write},
};

use common::collections::Map;

use crate::{Element, esc};

/// Attributes
///
/// html element attributes
pub type Attributes<'a> = Map<&'a str, Cow<'a, str>>;

impl<'a> Element for Map<&'a str, Cow<'a, str>> {
    fn render_into<W: Write>(&self, writer: &mut W) -> Result {
        for pair in self.iter() {
            write!(writer, " {}=\"", pair.key)?;
            esc(&pair.value, writer)?;
            write!(writer, "\"")?;
        }

        return Ok(());
    }
}
