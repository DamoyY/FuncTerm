use std::path::Path;
#[test]
fn quotes_literal_paths_for_powershell() {
    let quoted = super::powershell_path(Path::new("F:\\dir with ' quote")).unwrap();
    assert_eq!(
        quoted,
        "([Text.Encoding]::UTF8.GetString([Convert]::FromBase64String('RjpcZGlyIHdpdGggJyBxdW90ZQ==')))"
    );
}
#[test]
fn quotes_single_quotes_for_posix_shells() {
    let quoted = super::posix_string("a'b");
    assert_eq!(quoted, "'a'\\''b'");
}
#[test]
fn quotes_single_quotes_for_nushell() {
    let quoted = super::nushell_string("a'b");
    assert_eq!(quoted, "('YSdi' | decode base64 | decode)");
}
#[test]
fn quotes_double_quotes_for_cmd() {
    let quoted = super::cmd_string("a\"b");
    assert_eq!(quoted, "\"a\"\"b\"");
}
