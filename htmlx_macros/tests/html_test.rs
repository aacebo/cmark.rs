use htmlx::Element;

#[test]
pub fn should_render() {
    let value = htmlx::html! { <div id={"myid"} /> };
    assert_eq!(value, r#"<div id="myid" />"#);
}

#[test]
pub fn should_render_element() {
    let id = "test";
    let value = htmlx::rsx! { <div id={id} /> };
    assert_eq!(value.render(), r#"<div id="test" />"#);
}

#[test]
pub fn should_render_custom() {
    let value = htmlx::html! { <CustomElement /> };
    assert_eq!(value, r#"<CustomElement />"#);
}

#[test]
pub fn should_render_fragment() {
    let value = htmlx::html! { <>{"test"}</> };
    assert_eq!(value, "test");
}

#[test]
pub fn should_render_fragment_element() {
    let value = htmlx::rsx! { <>{"test"}</> };
    assert_eq!(value.render(), "test");
}

#[test]
pub fn should_render_with_children() {
    let value = htmlx::html! { <p class={"text bold"}>{"test"}</p> };
    assert_eq!(value, r#"<p class="text bold">test</p>"#);
}
