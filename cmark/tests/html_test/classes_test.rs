use cmark::{Render, RenderOptions, html::Element};

#[test]
pub fn should_add_class() {
    let mut el = Element::new("p");
    el.add_class("bg-red");

    assert!(el.has_class("bg-red"));

    match el.render(&RenderOptions::default()) {
        Ok(v) => assert_eq!(v, r#"<p class="bg-red" />"#),
        Err(err) => panic!("{}", err),
    }
}
