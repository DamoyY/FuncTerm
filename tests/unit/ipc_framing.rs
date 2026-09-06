use super::{CONTINUATION_FLAG, FrameReader, FrameWriter};
use serde::{Deserialize, Serialize};
use std::io::{self, Cursor, Read as _, Write as _};
#[test]
fn incomplete_headers_and_bodies_are_errors_not_clean_eof() {
    assert!(
        super::read_or_eof::<String, _>(&mut Cursor::new([]))
            .unwrap()
            .is_none()
    );
    let mut wire = Vec::new();
    super::write(&mut wire, &"value").unwrap();
    for end in 1..wire.len() {
        let (prefix, _) = wire.split_at(end);
        drop(super::read_or_eof::<String, _>(&mut Cursor::new(prefix)).unwrap_err());
    }
}
#[test]
fn consecutive_frames_do_not_consume_each_other() {
    let mut wire = Vec::new();
    super::write(&mut wire, &"first").unwrap();
    super::write(&mut wire, &"second").unwrap();
    let mut cursor = Cursor::new(wire);
    assert_eq!(
        super::read_or_eof::<String, _>(&mut cursor)
            .unwrap()
            .as_deref(),
        Some("first")
    );
    assert_eq!(
        super::read_or_eof::<String, _>(&mut cursor)
            .unwrap()
            .as_deref(),
        Some("second")
    );
    assert!(
        super::read_or_eof::<String, _>(&mut cursor)
            .unwrap()
            .is_none()
    );
}
#[test]
fn writer_handles_short_writes_and_empty_input_at_chunk_boundary() {
    for length in [0, 1, 15, 16, 17, 32, 33] {
        let payload = vec![b'x'; length];
        let mut wire = Vec::new();
        let mut target = ShortWriter(&mut wire);
        let mut writer = FrameWriter::<_, 16>::new(&mut target);
        writer.write_all(&payload).unwrap();
        assert_eq!(writer.write(b"").unwrap(), 0);
        writer.finish().unwrap();
        assert_eq!(continuation_count(&wire), length.div_ceil(16).max(1));
        let mut cursor = Cursor::new(&wire);
        let mut reader = FrameReader::open(&mut cursor).unwrap().unwrap();
        let mut actual = Vec::new();
        reader.read_to_end(&mut actual).unwrap();
        assert_eq!(actual, payload);
    }
}
#[test]
fn continuation_must_be_nonempty_and_have_a_following_header() {
    let empty = CONTINUATION_FLAG.to_be_bytes();
    assert!(FrameReader::open(&mut Cursor::new(empty)).is_err());
    let mut wire = (CONTINUATION_FLAG | 1).to_be_bytes().to_vec();
    wire.push(b'x');
    let mut cursor = Cursor::new(wire);
    let mut reader = FrameReader::open(&mut cursor).unwrap().unwrap();
    let error = reader.read_to_end(&mut Vec::new()).unwrap_err();
    assert_eq!(error.kind(), io::ErrorKind::UnexpectedEof);
}
#[test]
fn read_exact_retries_interrupted_and_short_header_reads() {
    let mut wire = Vec::new();
    super::write(&mut wire, &"ok").unwrap();
    let mut reader = FragmentedReader {
        bytes: Cursor::new(wire),
        calls: 0,
    };
    assert_eq!(
        super::read_or_eof::<String, _>(&mut reader)
            .unwrap()
            .as_deref(),
        Some("ok")
    );
}
struct ShortWriter<'buffer>(&'buffer mut Vec<u8>);
impl io::Write for ShortWriter<'_> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let amount = buf.len().min(1);
        let (portion, _) = buf.split_at(amount);
        self.0.extend_from_slice(portion);
        Ok(amount)
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
struct FragmentedReader {
    bytes: Cursor<Vec<u8>>,
    calls: usize,
}
impl io::Read for FragmentedReader {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.calls += 1;
        if matches!(self.calls, 1 | 3 | 5) {
            return Err(io::ErrorKind::Interrupted.into());
        }
        let count = buf.len().min(1);
        let (portion, _) = buf.split_at_mut(count);
        self.bytes.read(portion)
    }
}
#[derive(Debug, Deserialize, PartialEq, Eq, Serialize)]
struct Message {
    value: String,
}
#[test]
fn large_values_stream_across_continuation_chunks() {
    let expected = Message {
        value: "streamed-value-".repeat(20),
    };
    let mut wire = Vec::new();
    let mut writer = FrameWriter::<_, 16>::new(&mut wire);
    let buffered = sonic_rs::writer::BufferedWriter::new(&mut writer);
    sonic_rs::to_writer(buffered, &expected).unwrap();
    writer.finish().unwrap();
    assert!(continuation_count(&wire) > 1);
    let mut cursor = Cursor::new(&wire);
    let mut reader = FrameReader::open(&mut cursor).unwrap().unwrap();
    let actual: Message = sonic_rs::from_reader(&mut reader).unwrap();
    assert_eq!(actual, expected);
    assert_eq!(usize::try_from(cursor.position()).unwrap(), wire.len());
}
#[test]
fn small_values_keep_the_single_frame_wire_format() {
    let mut wire = Vec::new();
    let mut writer = FrameWriter::<_, 16>::new(&mut wire);
    let buffered = sonic_rs::writer::BufferedWriter::new(&mut writer);
    sonic_rs::to_writer(buffered, &"ok").unwrap();
    writer.finish().unwrap();
    let (Some(header), Some(body)) = (wire.get(..4), wire.get(4..)) else {
        panic!("single-frame message is incomplete");
    };
    assert_eq!(header, &4_u32.to_be_bytes());
    assert_eq!(body, br#""ok""#);
}
fn continuation_count(wire: &[u8]) -> usize {
    let mut offset = 0_usize;
    let mut chunks = 0_usize;
    loop {
        let header_end = offset.saturating_add(4);
        let Some(header_slice) = wire.get(offset..header_end) else {
            panic!("continuation header is incomplete");
        };
        let Ok(header_bytes) = <[u8; 4]>::try_from(header_slice) else {
            panic!("continuation header has an invalid length");
        };
        let header = u32::from_be_bytes(header_bytes);
        offset = header_end;
        offset = offset.saturating_add(usize::try_from(header & !CONTINUATION_FLAG).unwrap());
        chunks = chunks.saturating_add(1);
        if header & CONTINUATION_FLAG == 0 {
            assert_eq!(offset, wire.len());
            return chunks;
        }
    }
}
