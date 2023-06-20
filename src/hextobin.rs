//
// Copyright 2016 ihex Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use crate::record::Record;

#[derive(PartialEq, Eq, Default, Debug, Clone, Hash)]
pub struct BinaryData {
    pub bytes: Vec<u8>,
    pub start_address: usize,
}

struct AbsoluteRecord<'a> {
    data: &'a [u8],
    start_address: usize,
}

struct DataRecordIterator<'a> {
    records: &'a [Record],
    current_record: usize,
    extended_linear_address: usize,
    extended_segment_address: usize,
}

impl<'a> From<&'a [Record]> for DataRecordIterator<'a> {
    fn from(records: &'a [Record]) -> Self {
        DataRecordIterator {
            records: records,
            current_record: 0,
            extended_linear_address: 0,
            extended_segment_address: 0,
        }
    }
}

impl<'a> Iterator for DataRecordIterator<'a> {
    type Item = AbsoluteRecord<'a>;

    // Only return Data records through the iterator.
    // ExtendedSegmentAddress and ExtendedLinearAddress offsets are stored internally,
    // and applied to the Data records when they are returned.
    fn next(&mut self) -> Option<AbsoluteRecord<'a>> {
        while let Some(record) = self.records.get(self.current_record) {
            self.current_record += 1;
            match record {
                Record::Data { offset, value } => {
                    let current_base = self.extended_linear_address + self.extended_segment_address;
                    let start = current_base + usize::from(*offset);
                    return Some(AbsoluteRecord {
                        data: &value,
                        start_address: start,
                    });
                }
                Record::ExtendedSegmentAddress(start) => {
                    self.extended_segment_address = usize::from(*start) << 4;
                }
                Record::ExtendedLinearAddress(start) => {
                    self.extended_linear_address = usize::from(*start) << 16;
                }
                Record::EndOfFile => {
                    return None;
                }
                _ => println!("Unsupported record type: {:?}", record.record_type()),
            }
        }

        None
    }
}

/// Gets the address space needed for a sequence of Records.
fn get_address_space(records: &[Record]) -> (usize, usize) {
    let mut min_address = usize::MAX;
    let mut max_address = usize::MIN;

    for record in DataRecordIterator::from(records) {
        let start = record.start_address;
        let end = start + record.data.len();
        min_address = min_address.min(start);
        max_address = max_address.max(end);
    }

    (min_address, max_address)
}

/// Converts a vector of Records into bytes.
/// Unused bytes are filled with 0xFF.
pub fn convert_to_bytes(records: &[Record]) -> BinaryData {
    let (space_start, space_end) = get_address_space(records);
    let mut bytes = vec![0xFF; space_end - space_start];

    for record in DataRecordIterator::from(records) {
        let start = record.start_address - space_start;
        let end = start + record.data.len();
        bytes[start..end].copy_from_slice(record.data);
    }

    BinaryData {
        bytes,
        start_address: space_start,
    }
}
