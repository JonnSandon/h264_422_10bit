use bitstream_io::{BigEndian, BitReader, BitRead};
use std::io::Cursor;

pub struct Bitstream<'a> {
    reader: BitReader<Cursor<&'a [u8]>, BigEndian>,
}

impl<'a> Bitstream<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self {
            reader: BitReader::new(Cursor::new(data)),
        }
    }

    pub fn read_bit(&mut self) -> Result<u32, std::io::Error> {
        self.reader.read_bit().map(|b| u32::from(b))
    }

    pub fn read_bits(&mut self, n: u32) -> Result<u32, std::io::Error> {
        self.reader.read::<u32>(n)
    }

    pub fn read_ue(&mut self) -> Result<u32, std::io::Error> {
        let mut zeros = 0;
        while self.read_bit()? == 0 {
            zeros += 1;
        }
        if zeros == 0 {
            return Ok(0);
        }
        let value = self.read_bits(zeros)? + (1 << zeros) - 1;
        Ok(value)
    }

    pub fn read_se(&mut self) -> Result<i32, std::io::Error> {
        let ue = self.read_ue()?;
        let m = ((ue as i32) + 1) >> 1;
        if ue % 2 == 0 { Ok(-m) } else { Ok(m) }
    }
}
