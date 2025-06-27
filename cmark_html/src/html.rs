use std::fmt::{Result, Write};

use crate::HTMLElement;

#[derive(Debug)]
pub struct HTMLDocument;

impl HTMLElement for HTMLDocument {
    fn render_into(&self, writer: &mut dyn Write) -> Result {
        return write!(writer, "<!DOCTYPE html>");
    }
}
