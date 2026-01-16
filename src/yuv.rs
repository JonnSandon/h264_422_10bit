// src/yuv.rs 
#[derive(Debug, Clone)]
pub struct Yuv422p10 {
    pub width: u32,
    pub height: u32,
    pub bit_depth: u8, // always 10 for now

    /// Luma plane (Y), full resolution
    pub y: Vec<u16>,

    /// Chroma planes (U and V), half horizontal resolution
    pub u: Vec<u16>,
    pub v: Vec<u16>,
}

impl Yuv422p10 {
    pub fn new(width: u32, height: u32, bit_depth: u8) -> Self {
        let y_size = (width * height) as usize;
        let uv_size = ((width / 2) * height) as usize;

        Self {
            width,
            height,
            bit_depth,
            y: vec![0; y_size],
            u: vec![0; uv_size],
            v: vec![0; uv_size],
        }
    }

    #[inline]
    pub fn y_index(&self, x: u32, y: u32) -> usize {
        (y * self.width + x) as usize
    }

    #[inline]
    pub fn uv_index(&self, x: u32, y: u32) -> usize {
        // x is in full-res coordinates; divide by 2 for chroma
        (y * (self.width / 2) + (x / 2)) as usize
    }
}
