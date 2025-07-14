use std::fmt::{self, Display};

#[derive(Debug, Clone)]
pub struct Options {
    pub parse: ParseOptions,
    pub render: RenderOptions,
}

impl Default for Options {
    fn default() -> Self {
        return Self {
            parse: ParseOptions::default(),
            render: RenderOptions::default(),
        };
    }
}

#[derive(Debug, Clone)]
pub struct ParseOptions {
    /// support alternative syntax's
    pub alt: bool,

    /// support inline HTML
    pub html: bool,

    /// auto link parsing to anchor elements
    pub links: bool,
}

impl Default for ParseOptions {
    fn default() -> Self {
        return Self {
            alt: false,
            html: true,
            links: true,
        };
    }
}

#[derive(Debug, Clone)]
pub struct RenderOptions {
    pub escape: bool,
    pub indent_style: Indent,
    pub indent_size: u16,
    pub max_line_length: Option<u16>,
}

impl Default for RenderOptions {
    fn default() -> Self {
        return Self {
            escape: false,
            indent_style: Indent::Space,
            indent_size: 2,
            max_line_length: None,
        };
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Indent {
    Space,
    Tab,
}

impl Display for Indent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return match self {
            Self::Space => write!(f, "space"),
            Self::Tab => write!(f, "tab"),
        };
    }
}
