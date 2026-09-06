use super::EnvironmentSnapshot;
use std::ffi::OsString;
#[test]
fn environment_variables_round_trip() {
    let value = OsString::from("environment value");
    let snapshot = EnvironmentSnapshot::from_variables([(
        OsString::from("FUNCTERM_TEST_NAME"),
        value.clone(),
    )]);
    assert_eq!(snapshot.value("FUNCTERM_TEST_NAME"), Some(value));
}
#[test]
fn snapshot_round_trips_through_ipc_json() {
    let snapshot = EnvironmentSnapshot::from_variables([(
        OsString::from("FUNCTERM_TEST_NAME"),
        OsString::from("snapshot value"),
    )]);
    let json = sonic_rs::to_string(&snapshot).unwrap();
    let decoded = sonic_rs::from_str::<EnvironmentSnapshot>(&json).unwrap();
    assert_eq!(
        decoded.value("FUNCTERM_TEST_NAME"),
        Some(OsString::from("snapshot value"))
    );
}
#[cfg(windows)]
#[test]
fn new_tab_request_does_not_send_client_environment() {
    assert_eq!(
        EnvironmentSnapshot::for_new_tab_request().variables(),
        Vec::new()
    );
}
