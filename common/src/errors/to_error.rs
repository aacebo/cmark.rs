use std::error::Error;

pub trait ToError<Err: Error> {
    fn to_error(&self, message: &str) -> Err;
}
