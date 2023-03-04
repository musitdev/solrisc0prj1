//copied from
use core::slice::{self, SliceIndex};
use risc0_zkvm::Prover;

pub struct SovVec<const N: usize> {
    buffer: heapless::Vec<u32, N>,
}

impl<const N: usize> SovVec<N> {
    pub const fn new() -> Self {
        Self {
            buffer: heapless::Vec::<u32, N>::new(),
        }
    }

    pub fn push(&mut self, item: u32) -> Result<(), u32> {
        self.buffer.push(item)
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    pub fn remove(&mut self, index: usize) -> u32 {
        self.buffer.remove(index)
    }

    pub fn into_array<const M: usize>(self) -> Result<[u32; M], Self> {
        self.buffer.into_array().map_err(|buffer| Self { buffer })
    }

    pub fn get<I>(&self, index: I) -> Option<&<I as SliceIndex<[u32]>>::Output>
    where
        I: SliceIndex<[u32]>,
    {
        self.buffer.get(index)
    }

    //Risc0 specific method
    #[cfg(feature = "native")]
    pub fn sorted(&self) -> Self {
        let mut slice: [T; N] = self
            .buffer
            .clone()
            .into_array()
            .expect("Vec can't be converted to an array");
        slice.sort();
        let buffer = heapless::Vec::<T, N>::from_slice(&slice).unwrap();
        Self { buffer }
    }

    #[cfg(feature = "prover")]
    fn sorted(&self, prover: Prover) -> Self {
        let mut vals_with_idx: Vec<(usize, u32)> =
            self.buffer.clone().into_iter().enumerate().collect();
        vals_with_idx.sort_by(|(a_idx, a_value), (b_idx, b_value)| a_value.cmp(b_value));
        let indices: Vec<usize> = vals_with_idx.iter().map(|(idx, val)| *idx).collect();
        let values = vals_with_idx.into_iter().map(|(idx, val)| idx).collect();
        prover.add_input_u32_slice(&indices[..]);
        prover.add_input_u32_slice(&values[..]);
        Self(values)
    }

    // #[cfg(feature = "zk")]
    // fn sorted(&self) -> Self {
    //         let sorted_values: Vec<T> = ZkEnv::read();
    //   let indices: Vec<usize> = ZkEnv::read();
    //   for [a, b] in sorted_values.0.array_windows::<2>() {
    //     assert!(a <= b)
    //   }
    //   // Make additional checks
    //   // ...
    //   Self(sorted_values)
    // }
}

impl<'a, const N: usize> IntoIterator for &'a SovVec<N> {
    type Item = &'a u32;
    type IntoIter = slice::Iter<'a, u32>;

    fn into_iter(self) -> Self::IntoIter {
        self.buffer.iter()
    }
}

impl<const N: usize> FromIterator<u32> for SovVec<N> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = u32>,
    {
        let mut vec = SovVec::new();
        for i in iter {
            vec.push(i).ok().expect("SovVec::from_iter overflow");
        }
        vec
    }
}

impl<const N: usize> Clone for SovVec<N> {
    fn clone(&self) -> Self {
        Self {
            buffer: self.buffer.clone(),
        }
    }
}
