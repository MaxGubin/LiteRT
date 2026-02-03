#![allow(dead_code, clippy::all)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unnecessary_transmutes)]
//pub use self::root::*;

#[cfg(bindgen_rs_file)]
include!(env!("BINDGEN_RS_FILE"));

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bindings_are_exported() {
        assert_eq!(LiteRtStatus_kLiteRtStatusErrorInvalidArgument, 1);
    }
}
