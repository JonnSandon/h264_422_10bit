use thiserror::Error;

#[derive(Debug, Error)]
pub enum H264Error {
    #[error("Bitstream error: {0}")]
    Bitstream(String),

    #[error("NAL parsing error: {0}")]
    Nal(String),

    #[error("SPS/PPS parsing error: {0}")]
    ParameterSet(String),

    #[error("Slice parsing error: {0}")]
    Slice(String),

    #[error("Unsupported feature: {0}")]
    Unsupported(String),

    #[error("Internal decoder error: {0}")]
    Internal(String),
}
