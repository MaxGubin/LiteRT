use crate::bindings::*;

use std::ffi::CStr;
use std::fmt;

/// Reason of the error. It provides information where in the binding code the error occurred.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCause {
    Unknown,
    // compiled_model
    CreateOptions,
    SetOptionsHardwareAccelerators,
    CreateCompiledModel,
    GetCompiledModelInputBufferRequirements,
    GetCompiledModelOutputBufferRequirements,
    InputDoesntSupportAnyTensorBufferTypes,
    RunCompiledModel,
    // environment
    NotSupportedLiteRtAnyType,
    CreateEnvironment,
    // model
    GetSignatureKey,
    GetSignatureSubgraph,
    GetNumSignatureInputs,
    GetSignatureInputName,
    GetNumSignatureOutputs,
    GetSignatureOutputName,
    GetSignature,
    GetTensorTypeId,
    GetUnrankedTensorType,
    GetRankedTensorType,
    InvalidTensorTypeId,
    GetTensorName,
    GetNumSubgraphInputs,
    GetNumSubgraphOutputs,
    GetSubgraphInput,
    SubgraphInputTensorByNameNotFound,
    GetSubgraphOutput,
    SubgraphOutputTensorByNameNotFound,
    CreateModelFromFile,
    CreateModelFromBuffer,
    GetNumModelSubgraphs,
    GetNumModelSignatures,
    GetModelSignature,
    //tensor_buffer
    GetTensorBufferRequirementsBufferSize,
    GetNumTensorBufferRequirementsSupportedBufferTypes,
    GetTensorBufferRequirementsSupportedTensorBufferType,
    InvalidElementTypeEnumValue,
    InvalidTensorBufferTypeEnumValue,
    CreateManagedTensorBuffer,
    LockTensorBufferRead,
    LockTensorBufferWrite,
    GetTensorBufferPackedSize,
    IncompatibleWriteType,
    TensorBufferTooSmall,
    IncompatibleReadType,
    ReadBufferTooSmall,
    // util
    InvalidStringEncoding,
}

/// Error returned by the bindings.
///
/// It contains the reason of the error and the LiteRtStatus returned by the C code.
///
/// Error supports `fmt::Display` and `fmt::Debug` traits.
///
/// ```
/// match(litert_status) {
///     Ok(env) => { ... },
///     Err(error) => {
///         println!("Error: {:?}", error);
///     }
/// }
/// ```
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Error {
    cause: ErrorCause,
    litert_status: LiteRtStatus,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error: {:?}", self.cause)?;
        let status_description = self.litert_status_description();
        write!(f, " LiteRtStatus: {:?} [{}] ", self.litert_status, status_description)
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl std::error::Error for Error {}

impl Error {
    pub(crate) fn new(cause: ErrorCause, status: LiteRtStatus) -> Self {
        Error { cause, litert_status: status }
    }

    /// Returns the reason of the error from the binding code.
    pub fn cause(&self) -> ErrorCause {
        self.cause
    }

    /// Returns the LiteRtStatus returned by the C code.
    pub fn litert_status(&self) -> LiteRtStatus {
        self.litert_status
    }

    /// Returns a string description of the LiteRtStatus returned by the C code.
    pub fn litert_status_description(&self) -> String {
        // SAFETY: no input parameters, the output should be always a valid CStr.
        let status_description = unsafe {
            let description = LiteRtGetStatusString(self.litert_status);
            if description.is_null() {
                "???".to_string()
            } else {
                CStr::from_ptr(description).to_str().unwrap_or("???").to_string()
            }
        };
        status_description
    }
}
