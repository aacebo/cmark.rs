use cmark::{Render, html::Element};

#[test]
pub fn should_render() {
    let el = Element::new("p");

    match el.render() {
        Ok(v) => assert_eq!(v, "<p />"),
        Err(err) => panic!("{}", err),
    };
}
