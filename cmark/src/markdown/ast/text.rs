use std::fmt;

use crate::{
    ParseError, ParseOptions, Render, Stream,
    html::{self, ToHtml},
};

#[derive(Debug, Clone)]
pub struct Text(Vec<super::Inline>);

impl Text {
    pub fn parse(_stream: &mut Stream, _options: &ParseOptions) -> Result<Self, ParseError> {
        unimplemented!()
    }
}

impl Render for Text {
    fn render_into(&self, writer: &mut dyn fmt::Write) -> Result<(), fmt::Error> {
        return self.to_html().render_into(writer);
    }
}

impl html::ToHtml for Text {
    fn to_html(&self) -> html::Node {
        let mut el = html::Raw::new();

        for child in self.0.iter() {
            match child.render() {
                Ok(v) => el.push(v.as_str()),
                Err(_) => {}
            };
        }

        return html::Node::Raw(el);
    }
}

impl fmt::Display for Text {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return self.render_into(f);
    }
}
