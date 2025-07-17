use cmark::Cursor;

#[test]
fn should_parse() {
    let mut cursor = Cursor::from(Vec::from("testing123"));

    assert_eq!(cursor.is_sof(), true);
    assert_eq!(cursor.curr(), b't');
    assert_eq!(cursor.peek(), b't');

    cursor.next();
    assert_eq!(cursor.peek(), b'e');
    assert_eq!(cursor.to_str().unwrap_or_default(), "t");
}
