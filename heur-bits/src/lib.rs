#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![deny(rust_2018_idioms)]
// #![warn(missing_docs)] // TODO: Enable once finished
#![deny(rustdoc::broken_intra_doc_links)]
#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

use core::mem;

#[cfg(feature = "alloc")]
use alloc::{boxed::Box, vec::Vec};

mod flip;
pub use flip::{FlipAllBits, FlipBit};

mod climb;
pub use climb::{FirstAscentBitClimb, SteepestAscentBitClimb};

// TODO: 1. Add `#[diagnostic::on_unimplemented]`
//       2. Impl `Bits` for types from `smallvec`, `arrayvec`, `tinyvec`, `heapless`, `im`, and/or `bitvec`
pub trait Bits {
    #[must_use]
    fn len(&self) -> usize;

    #[must_use]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[must_use]
    fn get(&self, index: usize) -> Option<bool>;

    fn set(&mut self, index: usize, bit: bool) -> Option<bool>;
}

impl<const N: usize> Bits for [bool; N] {
    fn len(&self) -> usize {
        self.as_slice().len()
    }

    fn get(&self, index: usize) -> Option<bool> {
        self.as_slice().get(index).copied()
    }

    fn set(&mut self, index: usize, bit: bool) -> Option<bool> {
        self.get_mut(index).map(|prev| mem::replace(prev, bit))
    }
}

#[cfg(feature = "alloc")]
impl Bits for Vec<bool> {
    fn len(&self) -> usize {
        self.as_slice().len()
    }

    fn get(&self, index: usize) -> Option<bool> {
        self.as_slice().get(index).copied()
    }

    fn set(&mut self, index: usize, bit: bool) -> Option<bool> {
        self.get_mut(index).map(|prev| mem::replace(prev, bit))
    }
}

#[cfg(feature = "alloc")]
impl Bits for Box<[bool]> {
    fn len(&self) -> usize {
        self.as_ref().len()
    }

    fn get(&self, index: usize) -> Option<bool> {
        self.as_ref().get(index).copied()
    }

    fn set(&mut self, index: usize, bit: bool) -> Option<bool> {
        self.get_mut(index).map(|prev| mem::replace(prev, bit))
    }
}

#[cfg(feature = "alloc")]
impl<const N: usize> Bits for Box<[bool; N]> {
    fn len(&self) -> usize {
        self.as_ref().len()
    }

    fn get(&self, index: usize) -> Option<bool> {
        self.as_ref().get(index)
    }

    fn set(&mut self, index: usize, bit: bool) -> Option<bool> {
        self.as_mut().set(index, bit)
    }
}
