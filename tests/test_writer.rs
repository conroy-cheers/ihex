
extern crate ihex;

use ihex::record::Record;
use ihex::writer::*;

#[test]
fn test_record_to_string_for_data_record() {
  let empty_data_record = Record::Data { offset: 0x0000, value: vec![] };
  assert_eq!(empty_data_record.to_string(), String::from(":0000000000"));

  let data = vec![0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x20, 0x67, 0x61, 0x70];
  let populated_data_record = Record::Data { offset: 0x0010, value: data };
  assert_eq!(populated_data_record.to_string(), String::from(":0B0010006164647265737320676170A7"));

  // Validating that the maximum length data record will not panic on serialization.
  let max_length_data = (0..255).map(|_| 0u8).collect::<Vec<u8>>();
  let max_length_data_record = Record::Data { offset: 0x0000, value: max_length_data };
  let _ = max_length_data_record.to_string();
}

#[test]
#[should_panic]
fn test_record_to_string_for_data_record_with_invalid_data() {
  let invalid_data = (0..256).map(|_| 0u8).collect::<Vec<u8>>();
  let invalid_data_record = Record::Data { offset: 0x0010, value: invalid_data };
  let _ = invalid_data_record.to_string();
}

#[test]
fn test_record_to_string_for_eof_record() {
  let eof_record = Record::EndOfFile;
  assert_eq!(eof_record.to_string(), String::from(":00000001FF"));
}

#[test]
fn test_record_to_string_for_esa_record() {
  let esa_record_1 = Record::ExtendedSegmentAddress(0x1200);
  assert_eq!(esa_record_1.to_string(), String::from(":020000021200EA"));

  let esa_record_2 = Record::ExtendedSegmentAddress(0x55AA);
  assert_eq!(esa_record_2.to_string(), String::from(":0200000255AAFD"));
}

#[test]
fn test_record_to_string_for_ssa_record() {
  let ssa_record_1 = Record::StartSegmentAddress { cs: 0x0110, ip: 0x3801 };
  assert_eq!(ssa_record_1.to_string(), String::from(":0400000301103801AF"));

  let ssa_record_2 = Record::StartSegmentAddress { cs: 0x0000, ip: 0x3800 };
  assert_eq!(ssa_record_2.to_string(), String::from(":0400000300003800C1"));
}

#[test]
fn test_record_to_string_for_ela_record() {
  let ela_record_1 = Record::ExtendedLinearAddress(0xFFFF);
  assert_eq!(ela_record_1.to_string(), String::from(":02000004FFFFFC"));

  let ela_record_2 = Record::ExtendedLinearAddress(0x0F55);
  assert_eq!(ela_record_2.to_string(), String::from(":020000040F5596"));
}

#[test]
fn test_record_to_string_for_sla_record() {
  let sla_record_1 = Record::StartLinearAddress(0x000000CD);
  assert_eq!(sla_record_1.to_string(), String::from(":04000005000000CD2A"));

  let sla_record_2 = Record::StartLinearAddress(0x11223344);
  assert_eq!(sla_record_2.to_string(), String::from(":04000005112233444D"));
}

#[test]
fn test_create_object_file_representation_incorrect_termination() {
  assert_eq!(create_object_file_representation(&[]), None);
  assert_eq!(create_object_file_representation(&[Record::ExtendedLinearAddress(0)]), None);
  assert_eq!(create_object_file_representation(&[Record::EndOfFile, Record::ExtendedLinearAddress(0)]), None);
}

#[test]
fn test_create_object_file_representation_eof_only() {
  let records = &[Record::EndOfFile];
  let expected_result = String::from(":00000001FF");
  assert_eq!(create_object_file_representation(records).unwrap(), expected_result);
}

#[test]
fn test_create_object_file_representation_all_types() {
  let records = &[
    Record::Data { offset: 0x0010, value: vec![0x61,0x64,0x64,0x72,0x65,0x73,0x73,0x20,0x67,0x61,0x70] },
    Record::ExtendedSegmentAddress(0x1200),
    Record::StartSegmentAddress { cs: 0x0000, ip: 0x3800 },
    Record::ExtendedLinearAddress(0xFFFF),
    Record::StartLinearAddress(0x000000CD),
    Record::EndOfFile
  ];

  let expected_result = String::new() +
    &":0B0010006164647265737320676170A7\n" +
    &":020000021200EA\n" +
    &":0400000300003800C1\n" +
    &":02000004FFFFFC\n" +
    &":04000005000000CD2A\n" +
    &":00000001FF";

  assert_eq!(create_object_file_representation(records).unwrap(), expected_result);
}