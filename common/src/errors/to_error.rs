pub trait ToError {
    fn to_error(&self, message: &str) -> Box<dyn std::error::Error>;
}
