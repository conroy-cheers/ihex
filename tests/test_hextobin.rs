use ihex::convert_to_bytes;
use ihex::Record;

#[test]
fn test_hextobin() {
    let records = [
        Record::ExtendedSegmentAddress(0x00),
        Record::Data {
            offset: 0x0000,
            value: vec![0x00, 0x00, 0x00, 0x00],
        },
        Record::Data {
            offset: 0x0010,
            value: vec![0x10, 0x10, 0x10, 0x10],
        },
        Record::ExtendedSegmentAddress(0x01),
        Record::Data {
            offset: 0x0020,
            value: vec![0x20, 0x20, 0x20, 0x20],
        },
        Record::EndOfFile,
    ];
    let data = convert_to_bytes(&records);
    assert_eq!(data[0x0000..0x0004], [0x00, 0x00, 0x00, 0x00]);
    assert_eq!(data[0x0010..0x0014], [0x10, 0x10, 0x10, 0x10]);
    assert_eq!(data[0x0030..0x0034], [0x20, 0x20, 0x20, 0x20]);  // 0x0020 + 0x0010 from ExtendedSegmentAddress(0x01)
}
