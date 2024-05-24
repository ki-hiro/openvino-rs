use thiserror::Error;

/// Enumerate errors returned by the OpenVINO implementation. See
/// [`OvStatusCode`](https://docs.openvino.ai/2023.3/api/c_cpp_api/group__ov__base__c__api.html#_CPPv411ov_status_e).
// TODO This could be auto-generated (https://github.com/intel/openvino-rs/issues/20).
#[allow(missing_docs)]
#[derive(Debug, Error, PartialEq, Eq)]
pub enum InferenceError {
    #[error("general error: ({message})")]
    GeneralError { message: String },
    #[error("not implemented: ({message})")]
    NotImplemented { message: String },
    #[error("network not loaded: ({message})")]
    NetworkNotLoaded { message: String },
    #[error("parameter mismatch: ({message})")]
    ParameterMismatch { message: String },
    #[error("not found: ({message})")]
    NotFound { message: String },
    #[error("out of bounds: ({message})")]
    OutOfBounds { message: String },
    #[error("unexpected: ({message})")]
    Unexpected { message: String },
    #[error("request busy: ({message})")]
    RequestBusy { message: String },
    #[error("result not ready: ({message})")]
    ResultNotReady { message: String },
    #[error("not allocated: ({message})")]
    NotAllocated { message: String },
    #[error("infer not started: ({message})")]
    InferNotStarted { message: String },
    #[error("network not read: ({message})")]
    NetworkNotRead { message: String },
    #[error("infer cancelled: ({message})")]
    InferCancelled { message: String },
    #[error("invalid c parameter: ({message})")]
    InvalidCParam { message: String },
    #[error("unknown C error: ({message})")]
    UnknownCError { message: String },
    #[error("not implemented C method: ({message})")]
    NotImplementCMethod { message: String },
    #[error("unknown exception: ({message})")]
    UnknownException { message: String },
    #[error("undefined error code: {0}")]
    Undefined(i32),
}

impl InferenceError {
    /// Convert an `error_code` to a [`Result`]:
    /// - `0` becomes `Ok`
    /// - anything else becomes `Err` containing an [`InferenceError`]
    pub fn from(error_code: i32) -> Result<(), InferenceError> {
        #[allow(clippy::enum_glob_use)]
        use InferenceError::*;

        if error_code == openvino_sys::ov_status_e_OK {
            return Ok(());
        }

        let message = unsafe {
            std::ffi::CStr::from_ptr(openvino_sys::ov_get_last_err_msg())
                .to_string_lossy()
                .into_owned()
        };

        match error_code {
            openvino_sys::ov_status_e_GENERAL_ERROR => Err(GeneralError { message }),
            openvino_sys::ov_status_e_NOT_IMPLEMENTED => Err(NotImplemented { message }),
            openvino_sys::ov_status_e_NETWORK_NOT_LOADED => Err(NetworkNotLoaded { message }),
            openvino_sys::ov_status_e_PARAMETER_MISMATCH => Err(ParameterMismatch { message }),
            openvino_sys::ov_status_e_NOT_FOUND => Err(NotFound { message }),
            openvino_sys::ov_status_e_OUT_OF_BOUNDS => Err(OutOfBounds { message }),
            openvino_sys::ov_status_e_UNEXPECTED => Err(Unexpected { message }),
            openvino_sys::ov_status_e_REQUEST_BUSY => Err(RequestBusy { message }),
            openvino_sys::ov_status_e_RESULT_NOT_READY => Err(ResultNotReady { message }),
            openvino_sys::ov_status_e_NOT_ALLOCATED => Err(NotAllocated { message }),
            openvino_sys::ov_status_e_INFER_NOT_STARTED => Err(InferNotStarted { message }),
            openvino_sys::ov_status_e_NETWORK_NOT_READ => Err(NetworkNotRead { message }),
            openvino_sys::ov_status_e_INFER_CANCELLED => Err(InferCancelled { message }),
            openvino_sys::ov_status_e_INVALID_C_PARAM => Err(InvalidCParam { message }),
            openvino_sys::ov_status_e_UNKNOWN_C_ERROR => Err(UnknownCError { message }),
            openvino_sys::ov_status_e_NOT_IMPLEMENT_C_METHOD => {
                Err(NotImplementCMethod { message })
            }
            openvino_sys::ov_status_e_UNKNOW_EXCEPTION => Err(UnknownException { message }),
            _ => Err(Undefined(error_code)),
        }
    }
}

/// Enumerate setup failures: in some cases, this library will call library-loading code that may
/// fail in a different way (i.e., [`LoadingError`]) than the calls to the OpenVINO libraries (i.e.,
/// [`InferenceError`]).
#[allow(missing_docs)]
#[derive(Debug, Error)]
pub enum SetupError {
    #[error("inference error")]
    Inference(#[from] InferenceError),
    #[error("library loading error")]
    Loading(#[from] LoadingError),
}

/// Enumerate the ways that library loading can fail.
#[allow(missing_docs)]
#[derive(Debug, Error)]
pub enum LoadingError {
    #[error("system failed to load shared libraries (see https://github.com/intel/openvino-rs/blob/main/crates/openvino-finder): {0}")]
    SystemFailure(String),
    #[error("cannot find path to shared libraries (see https://github.com/intel/openvino-rs/blob/main/crates/openvino-finder)")]
    CannotFindLibraryPath,
    #[error("cannot find path to XML plugin configuration (see https://github.com/intel/openvino-rs/blob/main/crates/openvino-finder)")]
    CannotFindPluginPath,
    #[error("unable to convert path to a UTF-8 string (see https://doc.rust-lang.org/std/path/struct.Path.html#method.to_str)")]
    CannotStringifyPath,
}
