use cmark::Cursor;

#[test]
fn should_parse() {
    let mut cursor = Cursor::from(Vec::from("testing123"));

    assert_eq!(cursor.is_sof(), true);
    assert_eq!(cursor.start(), b't');
    assert_eq!(cursor.end(), b't');

    cursor.next();
    assert_eq!(cursor.end(), b'e');
    assert_eq!(cursor.to_str().unwrap_or_default(), "t");
}
