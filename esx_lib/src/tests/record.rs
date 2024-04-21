use bytes::BytesMut;

use crate::record::*;

const SAMPLE: [u8; 24] = [
  0x54, 0x45, 0x53, 0x34, // 'TES4' as bytes
  0x00, 0x00, 0x00, 0x00, // Data field size in bytes (0)
  0x00, 0x00, 0x00, 0x00, // Flags (None)
  0xDE, 0xAD, 0xBE, 0xEF, // Form ID (DEADBEEF)
  0x00, 0x00, // Timestamp (unset)
  0x00, 0x00, // VCS (unset)
  0x83, 0x00, // Internal version 131
  0x00, 0x00, // Unknown u16
];

#[test]
fn record_from_buffer() {
  let buf: BytesMut = BytesMut::from(SAMPLE.as_slice());

  let record = Record::from_bytes(&mut buf.clone()).unwrap();
  println!("Record Test: {:#?}", record);
  let bytes = record.as_bytes();
  println!("Bytes: {:?}", bytes);
  assert_eq!(bytes, buf);
}
