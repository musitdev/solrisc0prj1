//copied from
use core::slice::{self, SliceIndex};

pub struct Vec<T, const N: usize> {
    buffer: heapless::Vec<T, N>,
}

impl<T: std::cmp::Ord + std::clone::Clone + std::fmt::Debug, const N: usize> Vec<T, N> {
    pub const fn new() -> Self {
        Self {
            buffer: heapless::Vec::<T, N>::new(),
        }
    }

    pub fn push(&mut self, item: T) -> Result<(), T> {
        self.buffer.push(item)
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    pub fn remove(&mut self, index: usize) -> T {
        self.buffer.remove(index)
    }

    pub fn into_array<const M: usize>(self) -> Result<[T; M], Self> {
        self.buffer.into_array().map_err(|buffer| Self { buffer })
    }

    pub fn get<I>(&self, index: I) -> Option<&<I as SliceIndex<[T]>>::Output>
    where
        I: SliceIndex<[T]>,
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
    fn sorted(&self) -> Self {
        let mut vals_with_idx = self.0.iter().enumerate().copied().collect();
        vals_with_idx.sort_by(|(a_idx, a_value), (b_idx, b_value)| a <= b);
        let indices = vals_with_idx.iter().map(|(idx, val)| *idx).collect();
        let values = vals_with_idx.into_iter().map(|(idx, val)| idx).collect();
        ZkEnv::write(&indices[..]);
        ZkEnv::write(&values[..]);
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

impl<'a, T, const N: usize> IntoIterator for &'a Vec<T, N> {
    type Item = &'a T;
    type IntoIter = slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.buffer.iter()
    }
}

impl<T, const N: usize> Clone for Vec<T, N>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            buffer: self.buffer.clone(),
        }
    }
}
