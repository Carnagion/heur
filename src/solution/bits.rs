use std::{collections::VecDeque, mem};

// TODO: 1. Add `#[diagnostic::on_unimplemented]`
//       2. Impl `Bits` for types from `smallvec`, `arrayvec`, `tinyvec`, `heapless`, `im`, and/or `bitvec`
pub trait Bits {
    #[must_use]
    fn len(&self) -> usize;

    #[must_use]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn get(&self, index: usize) -> Option<bool>;

    fn set(&mut self, index: usize, bit: bool) -> Option<bool>;

    fn flip(&mut self, index: usize) -> Option<bool> {
        let bit = self.get(index)?;
        self.set(index, !bit)
    }
}

// NOTE: See the note on the `Solution` impl for `[bool]`.
impl Bits for [bool] {
    fn len(&self) -> usize {
        self.len()
    }

    fn get(&self, index: usize) -> Option<bool> {
        self.get(index).copied()
    }

    fn set(&mut self, index: usize, bit: bool) -> Option<bool> {
        self.get_mut(index).map(|old| mem::replace(old, bit))
    }

    fn flip(&mut self, index: usize) -> Option<bool> {
        self.get_mut(index).map(|bit| mem::replace(bit, !*bit))
    }
}

impl Bits for Vec<bool> {
    fn len(&self) -> usize {
        self.len()
    }

    fn get(&self, index: usize) -> Option<bool> {
        self.as_slice().get(index).copied()
    }

    fn set(&mut self, index: usize, bit: bool) -> Option<bool> {
        self.as_mut_slice().set(index, bit)
    }

    fn flip(&mut self, index: usize) -> Option<bool> {
        self.as_mut_slice().flip(index)
    }
}

impl<const N: usize> Bits for [bool; N] {
    fn len(&self) -> usize {
        self.as_slice().len()
    }

    fn get(&self, index: usize) -> Option<bool> {
        self.as_slice().get(index).copied()
    }

    fn set(&mut self, index: usize, bit: bool) -> Option<bool> {
        self.as_mut_slice().set(index, bit)
    }

    fn flip(&mut self, index: usize) -> Option<bool> {
        self.as_mut_slice().flip(index)
    }
}

impl Bits for VecDeque<bool> {
    fn len(&self) -> usize {
        self.len()
    }

    fn get(&self, index: usize) -> Option<bool> {
        self.get(index).copied()
    }

    fn set(&mut self, index: usize, bit: bool) -> Option<bool> {
        self.get_mut(index).map(|old| mem::replace(old, bit))
    }

    fn flip(&mut self, index: usize) -> Option<bool> {
        self.get_mut(index).map(|bit| mem::replace(bit, !*bit))
    }
}

impl<B> Bits for Box<B>
where
    B: Bits + ?Sized,
{
    fn len(&self) -> usize {
        self.as_ref().len()
    }

    fn is_empty(&self) -> bool {
        self.as_ref().is_empty()
    }

    fn get(&self, index: usize) -> Option<bool> {
        self.as_ref().get(index)
    }

    fn set(&mut self, index: usize, bit: bool) -> Option<bool> {
        self.as_mut().set(index, bit)
    }

    fn flip(&mut self, index: usize) -> Option<bool> {
        self.as_mut().flip(index)
    }
}
