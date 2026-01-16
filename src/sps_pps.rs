use crate::bitstream::Bitstream;
use crate::error::H264Error;

#[derive(Debug, Clone)]
pub struct SequenceParameterSet {
    pub profile_idc: u8,
    pub level_idc: u8,
    pub seq_parameter_set_id: u32,
    pub chroma_format_idc: u32,
    pub bit_depth_luma: u32,
    pub bit_depth_chroma: u32,
}

#[derive(Debug, Clone)]
pub struct PictureParameterSet {
    pub pic_parameter_set_id: u32,
    pub seq_parameter_set_id: u32,
}

impl SequenceParameterSet {
    pub fn parse(payload: &[u8]) -> Result<Self, H264Error> {
        let mut bs = Bitstream::new(payload);

        let profile_idc = bs.read_bits(8).map_err(|e| H264Error::ParameterSet(e.to_string()))? as u8;
        let _constraint_flags = bs.read_bits(8).map_err(|e| H264Error::ParameterSet(e.to_string()))?;
        let level_idc = bs.read_bits(8).map_err(|e| H264Error::ParameterSet(e.to_string()))? as u8;
        let seq_parameter_set_id = bs.read_ue().map_err(|e| H264Error::ParameterSet(e.to_string()))?;

        let mut chroma_format_idc = 1;
        let mut bit_depth_luma = 8;
        let mut bit_depth_chroma = 8;

        if matches!(profile_idc, 100 | 110 | 122 | 244 | 44 | 83 | 86 | 118 | 128 | 138 | 139 | 134) {
            chroma_format_idc = bs.read_ue().map_err(|e| H264Error::ParameterSet(e.to_string()))?;
            if chroma_format_idc == 3 {
                let _separate_colour_plane_flag = bs.read_bit().map_err(|e| H264Error::ParameterSet(e.to_string()))?;
            }
            bit_depth_luma = bs.read_ue().map_err(|e| H264Error::ParameterSet(e.to_string()))? + 8;
            bit_depth_chroma = bs.read_ue().map_err(|e| H264Error::ParameterSet(e.to_string()))? + 8;
            let _qpprime_y_zero_transform_bypass_flag = bs.read_bit().map_err(|e| H264Error::ParameterSet(e.to_string()))?;
            let _seq_scaling_matrix_present_flag = bs.read_bit().map_err(|e| H264Error::ParameterSet(e.to_string()))?;
        }

        Ok(Self {
            profile_idc,
            level_idc,
            seq_parameter_set_id,
            chroma_format_idc,
            bit_depth_luma,
            bit_depth_chroma,
        })
    }
}

impl PictureParameterSet {
    pub fn parse(payload: &[u8]) -> Result<Self, H264Error> {
        let mut bs = Bitstream::new(payload);

        let pic_parameter_set_id = bs.read_ue().map_err(|e| H264Error::ParameterSet(e.to_string()))?;
        let seq_parameter_set_id = bs.read_ue().map_err(|e| H264Error::ParameterSet(e.to_string()))?;

        Ok(Self {
            pic_parameter_set_id,
            seq_parameter_set_id,
        })
    }
}
