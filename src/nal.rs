use crate::error::H264Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NalUnitType {
    Unspecified,
    CodedSliceNonIdr,
    CodedSliceDataPartitionA,
    CodedSliceDataPartitionB,
    CodedSliceDataPartitionC,
    CodedSliceIdr,
    Sei,
    Sps,
    Pps,
    AccessUnitDelimiter,
    EndOfSequence,
    EndOfStream,
    FillerData,
    Unknown(u8),
}

#[derive(Debug, Clone)]
pub struct NalUnit {
    pub forbidden_zero_bit: u8,
    pub nal_ref_idc: u8,
    pub nal_unit_type: NalUnitType,
    pub payload: Vec<u8>,
}

impl NalUnitType {
    pub fn from_u8(v: u8) -> Self {
        match v {
            1 => Self::CodedSliceNonIdr,
            2 => Self::CodedSliceDataPartitionA,
            3 => Self::CodedSliceDataPartitionB,
            4 => Self::CodedSliceDataPartitionC,
            5 => Self::CodedSliceIdr,
            6 => Self::Sei,
            7 => Self::Sps,
            8 => Self::Pps,
            9 => Self::AccessUnitDelimiter,
            10 => Self::EndOfSequence,
            11 => Self::EndOfStream,
            12 => Self::FillerData,
            0 => Self::Unspecified,
            x => Self::Unknown(x),
        }
    }
}

pub fn split_annex_b(data: &[u8]) -> Vec<&[u8]> {
    let mut units = Vec::new();
    let mut i = 0;
    while i + 3 < data.len() {
        let three = &data[i..];
        let start_len = if three.starts_with(&[0, 0, 1]) {
            3
        } else if three.len() >= 4 && three.starts_with(&[0, 0, 0, 1]) {
            4
        } else {
            i += 1;
            continue;
        };

        let start = i + start_len;
        i = start;
        let mut end = data.len();
        let mut j = i;
        while j + 3 < data.len() {
            if data[j..].starts_with(&[0, 0, 1]) || (j + 4 <= data.len() && data[j..].starts_with(&[0, 0, 0, 1])) {
                end = j;
                break;
            }
            j += 1;
        }
        units.push(&data[start..end]);
        i = j;
    }
    units
}

pub fn remove_emulation_prevention_bytes(data: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(data.len());
    let mut i = 0;
    while i < data.len() {
        if i + 2 < data.len() && data[i] == 0 && data[i + 1] == 0 && data[i + 2] == 3 {
            out.push(0);
            out.push(0);
            i += 3;
        } else {
            out.push(data[i]);
            i += 1;
        }
    }
    out
}

pub fn parse_nal_unit(raw: &[u8]) -> Result<NalUnit, H264Error> {
    if raw.is_empty() {
        return Err(H264Error::Nal("Empty NAL unit".into()));
    }
    let header = raw[0];
    let forbidden_zero_bit = (header & 0b1000_0000) >> 7;
    let nal_ref_idc = (header & 0b0110_0000) >> 5;
    let nal_unit_type = NalUnitType::from_u8(header & 0b0001_1111);

    if forbidden_zero_bit != 0 {
        return Err(H264Error::Nal("forbidden_zero_bit != 0".into()));
    }

    let payload = remove_emulation_prevention_bytes(&raw[1..]);

    Ok(NalUnit {
        forbidden_zero_bit,
        nal_ref_idc,
        nal_unit_type,
        payload,
    })
}
