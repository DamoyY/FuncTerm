use super::decode_entry;
use std::ffi::OsString;
#[test]
fn ordinary_environment_entry_is_decoded() {
    let entry = "PATH=C:\\Windows".encode_utf16().collect::<Vec<_>>();
    assert_eq!(
        decode_entry(&entry).unwrap(),
        (OsString::from("PATH"), OsString::from("C:\\Windows"))
    );
}
#[test]
fn hidden_drive_environment_entry_is_decoded() {
    let entry = "=C:=C:\\work".encode_utf16().collect::<Vec<_>>();
    assert_eq!(
        decode_entry(&entry).unwrap(),
        (OsString::from("=C:"), OsString::from("C:\\work"))
    );
}
