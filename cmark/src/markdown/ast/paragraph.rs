use std::fmt;

use crate::{
    ParseError, ParseOptions, Render, Stream,
    html::{self, ToHtml},
};

#[derive(Debug, Clone)]
pub struct Paragraph(Vec<super::Inline>);

impl Paragraph {
    pub fn new() -> Self {
        return Self(vec![]);
    }

    pub fn push(&mut self, child: super::Inline) {
        self.0.push(child);
    }

    pub fn parse(stream: &mut Stream, options: &ParseOptions) -> Result<Self, ParseError> {
        let mut value = Self::new();

        while !stream.tokens().is_eof() {
            match super::Inline::parse(stream, options) {
                Ok(v) => value.push(v),
                Err(err) => {
                    if err.is_ignore() {
                        break;
                    }

                    return Err(err);
                }
            };
        }

        log::debug!(target: "cmark:md:paragraph", "parse");
        return Ok(value);
    }
}

impl Render for Paragraph {
    fn render_into(&self, writer: &mut dyn fmt::Write) -> Result<(), fmt::Error> {
        return self.to_html().render_into(writer);
    }
}

impl html::ToHtml for Paragraph {
    fn to_html(&self) -> html::Node {
        let mut el = html::Element::new("p");

        for child in self.0.iter() {
            el.push(child.to_html());
        }

        return el.to_html();
    }
}

impl fmt::Display for Paragraph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return self.render_into(f);
    }
}
