#![feature(once_cell)]
#![feature(array_windows)]

use core::slice::{self, SliceIndex};
use hashbrown::hash_map;
use serde::{Serialize, Deserialize};

#[cfg(feature = "prove")]
pub mod prover_context;
//mod serializer;

pub mod context;

#[derive(Debug,Serialize,Deserialize,Clone)]
pub enum IndexProof {
    E(usize),
    NE(i32,i32),
    SWITCH,
}

pub struct SovMap {
    map: hash_map::HashMap<u32, u32>,
    insert_observed_get_count: usize,

    get_count: usize,
    write_flag: bool,
    store_array_snaps: Vec<(u32, u32)>,
    store_array_sort_proofs: Vec<usize>,

    // zk items: need to be 0 for the zk run, but can be used by prover
    original_input_array: Vec<(u32,u32)>,
    current_get_count: usize,
    store_array_index: usize,
}

impl SovMap {
    pub fn new()->Self {
        Self{
            map: hash_map::HashMap::new(),
            store_array_snaps: vec![],
            write_flag: true,
            store_array_sort_proofs: vec![],
            store_array_index: 0,
            insert_observed_get_count: 0,
            get_count: 0,
            original_input_array: vec![],
            current_get_count: 0,
        }
    }

    #[cfg(feature = "native")]
    pub fn insert(&mut self, k: u32, v: u32) {
        self.map.insert(k,v);
    }

    #[cfg(feature = "native")]
    pub fn get(&mut self, k: u32) -> Option<u32> {
        let v = self.map.get(&k);
        match v {
            Some(x) => Some(*x),
            None => None
        }
    }

    #[cfg(feature = "prove")]
    pub fn insert(&mut self, key: u32, val: u32) {

        self.write_flag = true;

        self.store_array_snaps.push((key.clone(),val.clone()));
        self.store_array_snaps.sort_by(|x,y| x.0.cmp(&y.0));
        self.original_input_array.push((key.clone(), val.clone()));
        self.store_array_sort_proofs = vec![];

        for ele in &self.store_array_snaps {
            // TODO: unwrapping on purpose because something is very wrong if an element is not found here
            // crash is preferable. will consider the error case and decide how to handle later
            let idx = self.original_input_array.iter().position(|x| x.0 == ele.0).unwrap();
            self.store_array_sort_proofs.push(idx);
        }
        self.map.insert(key,val);
    }

    #[cfg(feature = "prove")]
    pub fn get(&mut self, key: u32) -> Option<&u32> {
        // TODO: handle duplicate key insertion. avoiding dups for now
        if self.write_flag {
            prover_context::ZK_CONTEXT.lock().unwrap().write(&IndexProof::SWITCH);
            prover_context::ZK_CONTEXT.lock().unwrap().write(
                &(
                    self.store_array_snaps.clone(),
                    self.store_array_sort_proofs.clone(),
                )
            );
        }

        self.write_flag = false;
        let val = self.map.get(&key);
        let idx = self.bin_search(&key);
        prover_context::ZK_CONTEXT.lock().unwrap().write(&idx);
        val
    }


    #[cfg(any(target_os = "zkvm", doc))]
    pub fn insert(&mut self, k: u32, v: u32) {
        self.map.insert(k,v);
    }

    #[cfg(any(target_os = "zkvm", doc))]
    pub fn get(&mut self, key: u32) -> Option<&u32> {
        let idx: IndexProof = context::read();
        match idx {
            IndexProof::E(idx) => Some(&self.store_array_snaps[idx].1),
            IndexProof::NE(_,_) => None,
            IndexProof::SWITCH => {
                let (store_array_snaps, store_array_sort_proofs):
                    (Vec<(u32, u32)>, Vec<usize>)= context::read();
                self.store_array_snaps = store_array_snaps;
                self.store_array_sort_proofs = store_array_sort_proofs;
                let idx: IndexProof = context::read();
                match idx {
                    IndexProof::E(indx) =>
                        Some(&self.store_array_snaps[indx].1),
                    IndexProof::NE(_, _) => None,
                    IndexProof::SWITCH => {panic!("nothing to switch to")}
                }
            }
        }
    }

    pub fn bin_search(&self, target_value: &u32) -> IndexProof {
        let mut low = 0usize;
        let mut high = self.store_array_snaps.len() - 1;
        let a = &self.store_array_snaps;
        let mut mid= 0;
        while low <= high {
            mid = ((high - low) / 2) + low;
            let mid_index = mid as usize;
            let val = &a[mid_index].0;

            if val == target_value {
                return IndexProof::E(mid_index);
            }

            if val < target_value {
                low = mid + 1;
            }

            if val > target_value {
                if mid != 0 {
                    high = mid - 1;
                } else {
                    break
                }
            }
        }
        IndexProof::NE(mid as i32 -1, mid as i32)
    }
}

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
