use cmark::pointer::Pointer;

#[test]
fn should_be_sof() {
    let ptr = Pointer::new(Vec::from("testing123"));
    assert_eq!(ptr.is_sof(), true);
}
