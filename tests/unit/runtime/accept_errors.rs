use std::io;
#[test]
fn interrupted_accept_error_is_recoverable() {
    let error = io::Error::from(io::ErrorKind::Interrupted);
    assert!(super::recoverable_accept_error(&error));
}
#[test]
fn permission_accept_error_is_fatal() {
    let error = io::Error::from(io::ErrorKind::PermissionDenied);
    assert!(!super::recoverable_accept_error(&error));
}
#[cfg(windows)]
#[test]
fn abandoned_named_pipe_accept_error_is_recoverable() {
    let code = i32::try_from(windows::Win32::Foundation::ERROR_OPERATION_ABORTED.0).unwrap();
    let error = io::Error::from_raw_os_error(code);
    assert!(super::recoverable_accept_error(&error));
}
