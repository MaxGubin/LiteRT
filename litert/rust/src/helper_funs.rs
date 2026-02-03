pub use crate::bindings::*;
use crate::error::{Error, ErrorCause};

use std::ffi::{c_char, CStr};

/// A helper function, that converts a const pointer to a C string to str
// SAFETY: The function is unsafe because it can get a not checked pointer from a C string.
pub unsafe fn c_str_to_str<'a>(c_str: *const c_char) -> Result<&'a str, Error> {
    if c_str == std::ptr::null() {
        return Err(Error::new(
            ErrorCause::InvalidStringEncoding,
            LiteRtStatus_kLiteRtStatusErrorRuntimeFailure,
        ));
    }
    // SAFETY: An UB is not possible as we check that the pointer is not null.
    unsafe {
        CStr::from_ptr(c_str).to_str().map_err(|_| {
            Error::new(
                ErrorCause::InvalidStringEncoding,
                LiteRtStatus_kLiteRtStatusErrorRuntimeFailure,
            )
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_nullptr_to_str() {
        // SAFETY: This is a test, so it is safe to pass a nullptr.
        unsafe {
            let result = c_str_to_str(std::ptr::null());
            assert_eq!(
                result,
                Err(Error::new(
                    ErrorCause::InvalidStringEncoding,
                    LiteRtStatus_kLiteRtStatusErrorRuntimeFailure
                ))
            );
        }
    }
}
