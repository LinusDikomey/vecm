use std::{mem::MaybeUninit, usize};

use crate::vec::*;


pub trait ByteConvert<const N: usize> {
    fn to_bytes(&self) -> [u8; N];
    fn from_bytes(b: &[u8]) -> Self;
}

impl ByteConvert<1> for u8 {
    #[inline]
    fn to_bytes(&self) -> [u8; 1] {
        [*self]
    }
    #[inline]
    fn from_bytes(b: &[u8]) -> Self {
        b[0]
    }
}
impl ByteConvert<1> for i8 {
    #[inline]
    fn to_bytes(&self) -> [u8; 1] {
        (*self as u8).to_bytes()
    }
    #[inline]
    fn from_bytes(b: &[u8]) -> Self {
        u8::from_bytes(b) as i8
    }
}

impl ByteConvert<2> for u16 {
    #[inline]
    fn to_bytes(&self) -> [u8; 2] {
        [
            (*self >> 8 & 255) as u8,
            (*self >> 0 & 255) as u8
        ]
    }
    #[inline]
    fn from_bytes(b: &[u8]) -> Self {
        ((b[0] as u16) << 8) +
        ((b[1] as u16) << 0)
    }
}
impl ByteConvert<2> for i16 {
    #[inline]
    fn to_bytes(&self) -> [u8; 2] { (*self as u16).to_bytes() }
    #[inline]
    fn from_bytes(b: &[u8]) -> i16 { u16::from_bytes(b) as i16 }
}



impl ByteConvert<4> for u32 {
    #[inline]
    fn to_bytes(&self) -> [u8; 4] {
        [
            (*self >> 24 & 255) as u8,
            (*self >> 16 & 255) as u8,
            (*self >>  8 & 255) as u8,
            (*self >>  0 & 255) as u8
        ]
    }
    #[inline]
    fn from_bytes(b: &[u8]) -> Self {
        ((b[0] as u32) << 24) +
        ((b[1] as u32) << 16) +
        ((b[2] as u32) <<  8) +
        ((b[3] as u32) <<  0)
    }
}
impl ByteConvert<4> for i32 {
    #[inline]
    fn to_bytes(&self) -> [u8; 4] { (*self as u32).to_bytes() }
    #[inline]
    fn from_bytes(b: &[u8]) -> i32 { u32::from_bytes(b) as i32 }
}

impl ByteConvert<8> for u64 {
    #[inline]
    fn to_bytes(&self) -> [u8; 8] {
        [
            (*self >> 56 & 255) as u8,
            (*self >> 48 & 255) as u8,
            (*self >> 40 & 255) as u8,
            (*self >> 32 & 255) as u8,
            (*self >> 24 & 255) as u8,
            (*self >> 16 & 255) as u8,
            (*self >>  8 & 255) as u8,
            (*self >>  0 & 255) as u8
        ]
    }
    #[inline]
    fn from_bytes(b: &[u8]) -> Self {
        ((b[0] as u64) << 56) +    
        ((b[1] as u64) << 48) +
        ((b[2] as u64) << 40) +
        ((b[3] as u64) << 32) +
        ((b[4] as u64) << 24) +
        ((b[5] as u64) << 16) +
        ((b[6] as u64) <<  8) +
        ((b[7] as u64) <<  0)
    }
}
impl ByteConvert<8> for i64 {
    #[inline]
    fn to_bytes(&self) -> [u8; 8] { (*self as u64).to_bytes() }
    #[inline]
    fn from_bytes(b: &[u8]) -> i64 { u64::from_bytes(b) as i64 }
}

// u128
impl ByteConvert<16> for u128 {
    #[inline]
    fn to_bytes(&self) -> [u8; 16] {
        [
            (*self >> 120 & 255) as u8,
            (*self >> 112 & 255) as u8,
            (*self >> 104 & 255) as u8,
            (*self >>  96 & 255) as u8,
            (*self >>  88 & 255) as u8,
            (*self >>  80 & 255) as u8,
            (*self >>  72 & 255) as u8,
            (*self >>  64 & 255) as u8,
            (*self >>  56 & 255) as u8,
            (*self >>  48 & 255) as u8,
            (*self >>  40 & 255) as u8,
            (*self >>  32 & 255) as u8,
            (*self >>  24 & 255) as u8,
            (*self >>  16 & 255) as u8,
            (*self >>   8 & 255) as u8,
            (*self >>   0 & 255) as u8
        ]
    }
    #[inline]
    fn from_bytes(b: &[u8]) -> Self {
        ((b[ 0] as u128) << 120) +    
        ((b[ 1] as u128) << 112) +
        ((b[ 2] as u128) << 104) +
        ((b[ 3] as u128) <<  96) +
        ((b[ 4] as u128) <<  88) +
        ((b[ 5] as u128) <<  80) +
        ((b[ 6] as u128) <<  72) +
        ((b[ 7] as u128) <<  64) +
        ((b[ 8] as u128) <<  56) +    
        ((b[ 9] as u128) <<  48) +
        ((b[10] as u128) <<  40) +
        ((b[11] as u128) <<  32) +
        ((b[12] as u128) <<  24) +
        ((b[13] as u128) <<  16) +
        ((b[14] as u128) <<   8) +
        ((b[15] as u128) <<   0)
    }
}
impl ByteConvert<16> for i128 {
    #[inline]
    fn to_bytes(&self) -> [u8; 16] { (*self as u128).to_bytes() }
    #[inline]
    fn from_bytes(b: &[u8]) -> i128 { u128::from_bytes(b) as i128 }
}

impl ByteConvert<4> for f32 {
    #[inline]
    fn to_bytes(&self) -> [u8; 4] { 
        unsafe { std::mem::transmute::<f32, u32>(*self) }.to_bytes() 
    }
    #[inline]
    fn from_bytes(b: &[u8]) -> Self { 
        unsafe { std::mem::transmute(u32::from_bytes(b)) } 
    }
}

impl ByteConvert<8> for f64 {
    #[inline]
    fn to_bytes(&self) -> [u8; 8] { 
        unsafe { std::mem::transmute::<f64, u64>(*self) }.to_bytes() 
    }
    #[inline]
    fn from_bytes(b: &[u8]) -> Self { 
        unsafe { std::mem::transmute(u64::from_bytes(b)) } 
    }
}

// The ugly combination with the ZST-array has to be done to bind S to the type. 
impl<B: ByteConvert<S>, const S: usize, const N: usize> ByteConvert<{S * N}> for ([B; N], [(); S]) {
    #[inline]
    fn to_bytes(&self) -> [u8; S * N] {
        let mut bytes: [u8; S * N] = unsafe { std::mem::MaybeUninit::uninit().assume_init() };
        for (i, elem) in self.0.iter().enumerate() {
            bytes[i*S..(i+1)*S].clone_from_slice(&elem.to_bytes())
        }
        bytes
    }
    #[inline]
    fn from_bytes(b: &[u8]) -> Self {
        let mut arr: [B; N] = unsafe { std::mem::MaybeUninit::uninit().assume_init() };
        for i in 0..N {
            arr[i] = B::from_bytes(&b[i*S..(i+1)*S]);
        }
        (arr, [(); S])
    }
}


// I don't know why these extra traits are needed but they are for some reason. Might work better in the future when const generics are finished
pub trait Vec2ByteConvert<const N: usize> {
    fn to_bytes(&self) -> [u8; 2*N];
    fn from_bytes(b: &[u8]) -> Self;
}

pub trait Vec3ByteConvert<const N: usize> {
    fn to_bytes(&self) -> [u8; 3*N];
    fn from_bytes(b: &[u8]) -> Self;
}
pub trait Vec4ByteConvert<const N: usize> {
    fn to_bytes(&self) -> [u8; 4*N];
    fn from_bytes(b: &[u8]) -> Self;
}

impl<T, const N: usize> Vec2ByteConvert<N> for PolyVec2<T> where T : ByteConvert<N> {
    #[inline]
    fn to_bytes(&self) -> [u8; 2*N] {
        let mut out: [u8; 2*N] = unsafe { MaybeUninit::uninit().assume_init() };
        out[..N].clone_from_slice(&self.x.to_bytes());
        out[N..].clone_from_slice(&self.y.to_bytes());
        out
    }
    #[inline]
    fn from_bytes(b: &[u8]) -> Self {
        Self {
            x: T::from_bytes(&b[0*N..1*N]),
            y: T::from_bytes(&b[1*N..2*N])
        }
    }
}

impl<T, const N: usize> Vec3ByteConvert<N> for PolyVec3<T> where T : ByteConvert<N> {
    #[inline]
    fn to_bytes(&self) -> [u8; 3*N] {
        let mut out: [u8; 3*N] = unsafe { MaybeUninit::uninit().assume_init() };
        out[0*N..1*N].clone_from_slice(&self.x.to_bytes());
        out[1*N..2*N].clone_from_slice(&self.y.to_bytes());
        out[2*N..3*N].clone_from_slice(&self.z.to_bytes());
        out
    }
    #[inline]
    fn from_bytes(b: &[u8]) -> Self {
        Self {
            x: T::from_bytes(&b[0*N..1*N]),
            y: T::from_bytes(&b[1*N..2*N]),
            z: T::from_bytes(&b[2*N..3*N])
        }
    }
}

impl<T, const N: usize> Vec4ByteConvert<N> for PolyVec4<T> where T : ByteConvert<N> {
    #[inline]
    fn to_bytes(&self) -> [u8; 4*N] {
        let mut out: [u8; 4*N] = unsafe { MaybeUninit::uninit().assume_init() };
        out[0*N..1*N].clone_from_slice(&self.x.to_bytes());
        out[1*N..2*N].clone_from_slice(&self.y.to_bytes());
        out[2*N..3*N].clone_from_slice(&self.z.to_bytes());
        out[3*N..4*N].clone_from_slice(&self.w.to_bytes());
        out
    }
    #[inline]
    fn from_bytes(b: &[u8]) -> Self {
        Self {
            x: T::from_bytes(&b[0*N..1*N]),
            y: T::from_bytes(&b[1*N..2*N]),
            z: T::from_bytes(&b[2*N..3*N]),
            w: T::from_bytes(&b[3*N..4*N])
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::vec::Vec4;
    use super::*;

    
    #[test]
    fn test_number_byte_convert() {
        const SAMPLE_SIZE: usize = 1_000_000;
        const VEC_SAMPLE_SIZE: usize = SAMPLE_SIZE;
        
        use rand::prelude::*;
        let mut rng = rand::thread_rng();

        println!("Testing u16...");
        for i in 0..=std::u16::MAX {
            assert_eq!(u16::from_bytes(&i.to_bytes()), i)
        }

        println!("Testing u32...");
        for _ in 0..SAMPLE_SIZE {
            let x = rng.gen::<u32>();
            assert_eq!(u32::from_bytes(&x.to_bytes()), x)
        }
        println!("Testing u64...");
        for _ in 0..SAMPLE_SIZE {
            let x = rng.gen::<u64>();
            assert_eq!(u64::from_bytes(&x.to_bytes()), x)
        }
        println!("Testing u128...");
        for _ in 0..SAMPLE_SIZE {
            let x = rng.gen::<u128>();
            assert_eq!(u128::from_bytes(&x.to_bytes()), x)
        }

        println!("Testing i16...");
        for i in std::i16::MIN..=std::i16::MAX {
            assert_eq!(i16::from_bytes(&i.to_bytes()), i)
        }

        println!("Testing i32...");
        for _ in 0..SAMPLE_SIZE {
            let x = rng.gen::<i32>();
            assert_eq!(i32::from_bytes(&x.to_bytes()), x)
        }
        println!("Testing i64...");
        for _ in 0..SAMPLE_SIZE {
            let x = rng.gen::<i64>();
            assert_eq!(i64::from_bytes(&x.to_bytes()), x)
        }
        println!("Testing i128...");
        for _ in 0..SAMPLE_SIZE {
            let x = rng.gen::<i128>();
            assert_eq!(i128::from_bytes(&x.to_bytes()), x)
        }
        
        println!("Testing Vec2");
        for _ in 0..VEC_SAMPLE_SIZE {
            let vec = Vec2::new(
                rng.gen(),
                rng.gen()
            );
            assert_eq!(Vec2::from_bytes(&vec.to_bytes()), vec)
        }
        println!("Testing Vec3");
        for _ in 0..VEC_SAMPLE_SIZE {
            let vec = Vec3::new(
                rng.gen(),
                rng.gen(),
                rng.gen()
            );
            assert_eq!(Vec3::from_bytes(&vec.to_bytes()), vec)
        }
        println!("Testing Vec4");
        for _ in 0..VEC_SAMPLE_SIZE {
            let vec = Vec4::new(
                rng.gen(),
                rng.gen(),
                rng.gen(),
                rng.gen()
            );
            assert_eq!(Vec4::from_bytes(&vec.to_bytes()), vec)
        }
    }
}