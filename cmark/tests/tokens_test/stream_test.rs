use cmark::{Iter, TokenStream};

#[test]
pub fn should_parse() {
    let mut stream = TokenStream::from("!>= te-st");

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

    debug_assert_eq!(
        stream.next_if(" ").unwrap_or_default().as_str(),
        " ",
        "4. stream => {:#?}",
        stream
    );

    debug_assert_eq!(stream.next_if("test"), None, "5. stream => {:#?}", stream);

    debug_assert_eq!(
        stream.next_if("te").unwrap_or_default().as_str(),
        "te",
        "6. stream => {:#?}",
        stream
    );

    debug_assert_eq!(
        stream.next_if("-").unwrap_or_default().as_str(),
        "-",
        "7. stream => {:#?}",
        stream
    );

    debug_assert_eq!(
        stream.next_if("st").unwrap_or_default().as_str(),
        "st",
        "8. stream => {:#?}",
        stream
    );
}
