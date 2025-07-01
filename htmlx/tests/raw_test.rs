use htmlx::*;

#[test]
pub fn should_decode() {
    let output = "<Hello />".render();
    assert_eq!(output, "&lt;Hello /&gt;");
}

#[test]
pub fn should_decode_raw() {
    let output = Raw::from("<Hello />").render();
    assert_eq!(output, "<Hello />");
}

#[test]
pub fn should_decode_raw_macro() {
    let output = raw!("<Hello />").render();
    assert_eq!(output, "<Hello />");
}
