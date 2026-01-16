use std::collections::HashMap;

use crate::error::H264Error;
use crate::nal::{self, NalUnitType};
use crate::sps_pps::{PictureParameterSet, SequenceParameterSet};

#[derive(Debug, Clone)]
pub struct DecodedFrame {
    pub width: u32,
    pub height: u32,
    pub is_field: bool,
    pub is_idr: bool,
    // TODO: YUV planes, timestamps, etc.
}

pub struct H264Decoder {
    sps_map: HashMap<u32, SequenceParameterSet>,
    pps_map: HashMap<u32, PictureParameterSet>,
}

impl H264Decoder {
    pub fn new() -> Self {
        Self {
            sps_map: HashMap::new(),
            pps_map: HashMap::new(),
        }
    }

    pub fn push_annex_b(&mut self, data: &[u8]) -> Result<Vec<DecodedFrame>, H264Error> {
        let mut frames = Vec::new();

        let units = nal::split_annex_b(data);
        for raw_nal in units {
            let nal = nal::parse_nal_unit(raw_nal)?;
            match nal.nal_unit_type {
                NalUnitType::Sps => {
                    let sps = SequenceParameterSet::parse(&nal.payload)?;
                    self.sps_map.insert(sps.seq_parameter_set_id, sps);
                }
                NalUnitType::Pps => {
                    let pps = PictureParameterSet::parse(&nal.payload)?;
                    self.pps_map.insert(pps.pic_parameter_set_id, pps);
                }
                NalUnitType::CodedSliceIdr | NalUnitType::CodedSliceNonIdr => {
                    let frame = DecodedFrame {
                        width: 1920,
                        height: 1080,
                        is_field: false,
                        is_idr: nal.nal_unit_type == NalUnitType::CodedSliceIdr,
                    };
                    frames.push(frame);
                }
                _ => {}
            }
        }

        Ok(frames)
    }
}
