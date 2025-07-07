use htmlx::Element;

#[test]
pub fn should_render() {
    let value = htmlx::html! { <div id={"myid"} /> };
    assert_eq!(value, r#"<div id="myid"/>"#);
}

#[test]
pub fn should_render_element() {
    let id = "test";
    let value = htmlx::rsx! { <div id={id} /> };
    assert_eq!(value.render(), r#"<div id="test"/>"#);
}
