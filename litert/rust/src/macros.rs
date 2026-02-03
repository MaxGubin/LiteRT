#[macro_export]
macro_rules! call_check_status {
    // This matcher captures the function call as an expression (`$call`)
    // and the error cause as a path (`$error_cause`).
    ($call:expr, $error_cause:path) => {
        // The code that will be generated
        let status = $call;
        if status != $crate::bindings::LiteRtStatus_kLiteRtStatusOk {
            return Err($crate::Error::new($error_cause, status));
        }
    };
}
