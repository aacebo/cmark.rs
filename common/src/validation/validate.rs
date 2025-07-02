use std::error::Error;

/// Validate
///
/// common validation implementation
pub trait Validate {
    fn validate(&self) -> Option<&(dyn Error + 'static)> {
        return None;
    }
}
