use htmlx::Element;

#[test]
pub fn should_render() {
    let value = htmlx::rsx! { <div id={"myid"} /> };
    assert_eq!(value.render(), r#"<div id="myid"/>"#);
}
