use crate::encoding::Encoding;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub struct BinaryReader {
    buf: Vec<u8>,
    index: usize,
}

impl BinaryReader {
    pub fn tell(&self) -> usize {
        self.index
    }

    pub fn len(&self) -> usize {
        self.buf.len()
    }

    pub fn new<P: AsRef<Path> + Clone>(path: P) -> Result<BinaryReader, std::io::Error> {
        let mut buf = BinaryReader {
            buf: Vec::new(),
            index: 0,
        };
        let _read = File::open(path)?.read_to_end(&mut buf.buf)?;
        return Ok(buf);
    }

    pub fn from_vec(vec: Vec<u8>) -> BinaryReader {
        BinaryReader { buf: vec, index: 0 }
    }

    pub fn next_u8(&mut self) -> u8 {
        self.index += 1;
        self.buf[self.index - 1]
    }

    pub fn next_f32(&mut self) -> f32 {
        let bytes = [
            self.buf[self.index],
            self.buf[self.index + 1],
            self.buf[self.index + 2],
            self.buf[self.index + 3],
        ];
        self.index += 4;
        unsafe { std::mem::transmute(bytes) }
    }

    pub fn next_i32(&mut self) -> i32 {
        let bytes = [
            self.buf[self.index],
            self.buf[self.index + 1],
            self.buf[self.index + 2],
            self.buf[self.index + 3],
        ];
        self.index += 4;
        i32::from_le_bytes(bytes)
    }

    pub fn next_u32(&mut self) -> u32 {
        let bytes = [
            self.buf[self.index],
            self.buf[self.index + 1],
            self.buf[self.index + 2],
            self.buf[self.index + 3],
        ];
        self.index += 4;
        u32::from_le_bytes(bytes)
    }

    pub fn next_u16(&mut self) -> u16 {
        let bytes = [self.buf[self.index], self.buf[self.index + 1]];
        self.index += 2;
        u16::from_le_bytes(bytes)
    }

    pub fn string(&mut self, max_len: u32) -> String {
        let i = self.index;
        self.index += max_len as usize;
        let bytes: Vec<u8> = self
            .buf
            .iter()
            .skip(i)
            .take(max_len as usize)
            .take_while(|b| **b != 0)
            .map(|b| *b)
            .collect();
        let decoded = encoding::all::WINDOWS_1252
            .decode(&bytes, encoding::DecoderTrap::Strict)
            .unwrap();
        //        return String::from_utf8(encoding::all::UTF_8.encode(&decoded, EncoderTrap::Strict).unwrap()).unwrap();
        decoded
    }

    pub fn skip(&mut self, size: u32) {
        self.index += size as usize;
    }

    pub fn next(&mut self, size: u32) -> Vec<u8> {
        let from = self.index;
        self.index += size as usize;
        self.buf[from..self.index].to_vec()
    }
}
