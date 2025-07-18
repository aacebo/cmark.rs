use cmark::Stream;

#[test]
pub fn should_parse() {
    let mut stream = Stream::from("!>= te-st");

    debug_assert!(stream.next_if("!"), "1. stream => {:#?}", stream);
    debug_assert!(stream.next_if(">"), "2. stream => {:#?}", stream);
    debug_assert!(stream.next_if("="), "3. stream => {:#?}", stream);
    debug_assert!(stream.next_if(" "), "4. stream => {:#?}", stream);
    debug_assert_eq!(stream.next_if("test"), false, "5. stream => {:#?}", stream);
    debug_assert!(stream.next_if("te"), "6. stream => {:#?}", stream);
    debug_assert!(stream.next_if("-"), "7. stream => {:#?}", stream);
    debug_assert!(stream.next_if("st"), "8. stream => {:#?}", stream);
}
