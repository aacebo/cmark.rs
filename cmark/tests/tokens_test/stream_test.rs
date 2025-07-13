use cmark::{Iter, tokens};

#[test]
pub fn should_parse_literals() {
    let mut stream = tokens::Stream::from("!>=");

    debug_assert_eq!(
        stream.next_if("!").unwrap_or_default().as_str(),
        "!",
        "1. stream => {:#?}",
        stream
    );

    debug_assert_eq!(
        stream.next_if(">").unwrap_or_default().as_str(),
        ">",
        "2. stream => {:#?}",
        stream
    );

    debug_assert_eq!(
        stream.next_if("=").unwrap_or_default().as_str(),
        "=",
        "3. stream => {:#?}",
        stream
    );
}
