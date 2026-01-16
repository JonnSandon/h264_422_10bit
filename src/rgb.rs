// src/rgb.rs 
use crate::yuv::Yuv422p10;

pub struct Rgb8Frame {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>, // RGBRGBRGB...
}

impl Rgb8Frame {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            data: vec![0; (width * height * 3) as usize],
        }
    }
}

impl Yuv422p10 {
    pub fn to_rgb8(&self) -> Rgb8Frame {
        let mut out = Rgb8Frame::new(self.width, self.height);

        // TODO: implement proper YUV -> RGB conversion
        // For now, fill with grey for testing
        for px in out.data.chunks_mut(3) {
            px[0] = 128;
            px[1] = 128;
            px[2] = 128;
        }

        out
    }
}
