#[test]
pub fn should_render() {
    let value = cmark::rsx! { <div id={"myid"} /> };
    assert_eq!(value, r#"<div id="myid" />"#);
}

#[test]
pub fn should_render_element() {
    let id = "test";
    let value = cmark::rsx! { <div id={id} /> };
    assert_eq!(value, r#"<div id="test" />"#);
}

#[test]
pub fn should_render_custom() {
    let value = cmark::rsx! { <CustomElement /> };
    assert_eq!(value, r#"<CustomElement />"#);
}

#[test]
pub fn should_render_fragment() {
    let value = cmark::rsx! { <>{"test"}</> };
    assert_eq!(value, "test");
}

#[test]
pub fn should_render_fragment_element() {
    let value = cmark::rsx! {
        <>
            <>{"a"}</>
            <>{"b"}</>
            {"c"}
            {"d"}
        </>
    };

    assert_eq!(value, "abcd");
}

#[test]
pub fn should_render_with_children() {
    let value = cmark::rsx! { <p class={"text bold"}>{"test"}</p> };
    assert_eq!(value, r#"<p class="text bold">test</p>"#);
}
