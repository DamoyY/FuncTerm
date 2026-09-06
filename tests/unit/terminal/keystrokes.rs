use super::InputBatch;
#[test]
fn empty_input_has_no_segments_or_interrupt() {
    let batch = InputBatch::from_bytes(b"");
    assert_eq!(batch.segments().count(), 0);
    assert!(!batch.delivery().interrupted());
}
#[test]
fn data_segments_borrow_the_input_and_preserve_interrupt_order() {
    let input = b"\x03before\x03\x03after\x03";
    let batch = InputBatch::from_bytes(input);
    let segments = batch.segments().collect::<Vec<_>>();
    let interrupt = super::interrupt_bytes();
    let mut data = input
        .split(|byte| *byte == 3)
        .filter(|chunk| !chunk.is_empty());
    let before = data.next().unwrap();
    let after = data.next().unwrap();
    assert_eq!(
        segments,
        [interrupt, before, interrupt, interrupt, after, interrupt]
    );
    let &[_, borrowed_before, _, _, borrowed_after, _] = segments.as_slice() else {
        panic!("expected two data segments and four interrupts");
    };
    assert!(core::ptr::eq(borrowed_before, before));
    assert!(core::ptr::eq(borrowed_after, after));
    assert!(batch.delivery().interrupted());
}
#[test]
fn all_non_interrupt_bytes_are_preserved_without_copying() {
    let input = (0_u8..=255).filter(|byte| *byte != 3).collect::<Vec<_>>();
    let batch = InputBatch::from_bytes(&input);
    let mut segments = batch.segments();
    assert!(core::ptr::eq(segments.next().unwrap(), input.as_slice()));
    assert!(segments.next().is_none());
}
fn encoded_input(bytes: &[u8]) -> (Vec<u8>, super::InputDelivery) {
    let batch = InputBatch::from_bytes(bytes);
    let mut encoded = Vec::new();
    for segment in batch.segments() {
        encoded.extend_from_slice(segment);
    }
    (encoded, batch.delivery())
}
#[test]
fn ordinary_text_is_unchanged() {
    let (encoded, delivery) = encoded_input(b"typed input");
    assert_eq!(encoded, b"typed input");
    assert!(!delivery.interrupted());
}
#[cfg(not(windows))]
#[test]
fn terminal_reply_is_unchanged_on_unix() {
    assert_eq!(
        super::host_reply_bytes(b"\x1b[1;2R", true)
            .unwrap()
            .as_ref(),
        b"\x1b[1;2R"
    );
}
#[cfg(windows)]
#[test]
fn ctrl_c_uses_win32_input_mode_key_events() {
    let (encoded, delivery) = encoded_input(b"\x03");
    assert_eq!(
        encoded,
        concat!(
            "\x1b[17;29;0;1;8;1_",
            "\x1b[67;46;3;1;8;1_",
            "\x1b[67;46;3;0;8;1_",
            "\x1b[17;29;0;0;0;1_",
        )
        .as_bytes()
    );
    assert!(delivery.interrupted());
}
#[cfg(windows)]
#[test]
fn ctrl_c_can_be_embedded_between_text_chunks() {
    let (encoded, delivery) = encoded_input(b"before\x03after");
    assert_eq!(
        encoded,
        concat!(
            "before",
            "\x1b[17;29;0;1;8;1_",
            "\x1b[67;46;3;1;8;1_",
            "\x1b[67;46;3;0;8;1_",
            "\x1b[17;29;0;0;0;1_",
            "after",
        )
        .as_bytes()
    );
    assert!(delivery.interrupted());
}
#[cfg(not(windows))]
#[test]
fn ctrl_c_remains_etx_on_unix() {
    let (encoded, delivery) = encoded_input(b"before\x03after");
    assert_eq!(encoded, b"before\x03after");
    assert!(delivery.interrupted());
}
#[cfg(windows)]
#[test]
fn terminal_reply_is_encoded_as_literal_win32_input() {
    assert_eq!(
        super::host_reply_bytes(b"\x1b[1;2R", true)
            .unwrap()
            .as_ref(),
        concat!(
            "\x1b[0;0;27;1;0;1_",
            "\x1b[0;0;91;1;0;1_",
            "\x1b[0;0;49;1;0;1_",
            "\x1b[0;0;59;1;0;1_",
            "\x1b[0;0;50;1;0;1_",
            "\x1b[0;0;82;1;0;1_",
        )
        .as_bytes()
    );
}
