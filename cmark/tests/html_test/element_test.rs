use cmark::{Render, RenderOptions, html::Element};

#[test]
pub fn should_render() {
    let el = Element::new("p");

    match el.render(&RenderOptions::default()) {
        Ok(v) => assert_eq!(v, "<p />"),
        Err(err) => panic!("{}", err),
    };
}
