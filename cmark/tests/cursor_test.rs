use cmark::cursor::Cursor;

#[test]
fn should_be_sof() {
    let cursor = Cursor::new(Vec::from("testing123"));
    assert_eq!(cursor.is_sof(), true);
}
