#![feature(once_cell)]
#![feature(array_windows)]

use core::slice::{self, SliceIndex};

#[cfg(feature = "prove")]
pub mod prover_context;
//mod serializer;

pub mod context;

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
        //hint generation for prover
        //hint get for zk
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
        //hint write for prover write hint1 then hint2
        //hit get for zk. get hint2 and hint1
    }

    #[cfg(feature = "prove")]
    pub fn sorted(&self) -> Self {
        //use u32 as indice to avoid x86 usize (u64) to  risk0 (u32) usize implicit conversion
        let mut vals_with_idx: Vec<(u32, u32)> = self
            .buffer
            .clone()
            .into_iter()
            .enumerate()
            .map(|(indice, val)| (indice as u32, val))
            .collect();
        vals_with_idx.sort_by(|(_, a_value), (_, b_value)| a_value.cmp(b_value));
        let indices: Vec<u32> = vals_with_idx.iter().map(|(idx, _)| *idx).collect();
        let values: Vec<u32> = vals_with_idx.into_iter().map(|(_, val)| val).collect();
        prover_context::ZK_CONTEXT
            .lock()
            .unwrap()
            .write_data(indices);
        let buffer = heapless::Vec::<u32, N>::from_slice(&values[..]).unwrap();
        prover_context::ZK_CONTEXT
            .lock()
            .unwrap()
            .write_data(values);
        SovVec { buffer }
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
        SovVec { buffer }
    }

    #[cfg(any(target_os = "zkvm", doc))]
    pub fn sorted(&self) -> Self {
        let indices: Vec<u32> = context::read();
        let sorted_values: Vec<u32> = context::read();
        for [a, b] in sorted_values.array_windows::<2>() {
            assert!(a <= b)
        }
        // Make additional checks with indice.
        // ...
        Self {
            buffer: heapless::Vec::<u32, N>::from_slice(&sorted_values[..]).unwrap(),
        }
    }
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
