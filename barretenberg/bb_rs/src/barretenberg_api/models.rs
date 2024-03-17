use super::traits::{DeserializeBuffer, SerializeBuffer};

pub struct Fr {
    pub data: [u8; 32],
}

impl SerializeBuffer for Fr {
    fn to_buffer(&self) -> Vec<u8> {
        self.data.to_vec()
    }
}

impl DeserializeBuffer for Fr {
    type Slice = [u8; 32];
    unsafe fn from_buffer(buf: Self::Slice) -> Self {
        Fr { data: buf }
    }
}

pub struct Fq {
    pub data: [u8; 32],
}

impl SerializeBuffer for Fq {
    fn to_buffer(&self) -> Vec<u8> {
        self.data.to_vec()
    }
}

impl DeserializeBuffer for Fq {
    type Slice = [u8; 32];
    unsafe fn from_buffer(buf: Self::Slice) -> Self {
        Fq { data: buf }
    }
}

pub struct Point {
    pub x: Fr,
    pub y: Fr,
}

impl SerializeBuffer for Point {
    fn to_buffer(&self) -> Vec<u8> {
        self.x
            .to_buffer()
            .into_iter()
            .chain(self.y.to_buffer().into_iter())
            .collect()
    }
}

impl DeserializeBuffer for Point {
    type Slice = [u8; 64];
    unsafe fn from_buffer(buf: Self::Slice) -> Self {
        let mut fr1: <Fr as DeserializeBuffer>::Slice = [0; 32];
        let mut fr2: <Fr as DeserializeBuffer>::Slice = [0; 32];
        fr1.clone_from_slice(&buf[..32]);
        fr2.clone_from_slice(&buf[32..]);
        Self {
            x: Fr::from_buffer(fr1),
            y: Fr::from_buffer(fr2),
        }
    }
}
