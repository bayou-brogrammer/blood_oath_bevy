use crate::prelude::*;
use bitvec::{
    prelude::*,
    slice::{Iter, IterMut},
};
use grid_2d::Size;
use serde::{Deserialize, Serialize};

/// A width-by-height&-sized BitVec for convenient handling of a grid of boolean values.
#[derive(Default, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BitGrid {
    bv: BitVec,
    size: Size,
}

impl BitGrid {
    fn dimensions(&self) -> Point {
        self.size.to_point()
    }

    fn in_bounds(&self, pos: Coord) -> bool {
        self.size.is_valid(pos)
    }

    /// Convert a Point (x/y) to an array index. Defaults to an index based on an array
    /// strided X first.
    #[allow(dead_code)]
    fn coord_to_index(&self, coord: Coord) -> usize {
        let bounds = self.dimensions();
        ((coord.y * bounds.x) + coord.x)
            .try_into()
            .expect("Not a valid usize. Did something go negative?")
    }

    /// Convert an array index to a point. Defaults to an index based on an array
    /// strided X first.
    fn index_to_coord2d(&self, idx: usize) -> Coord {
        let bounds = self.dimensions();
        let w: usize =
            bounds.x.try_into().expect("Not a valid usize. Did something go negative?");
        Coord::new((idx % w) as i32, (idx / w) as i32)
    }
}

impl BitGrid {
    /// Create a new BitGrid with the given width and height.
    pub fn new(size: Size) -> Self {
        Self::new_with_bit(size, false)
    }

    pub fn new_with_bit(size: Size, bit: bool) -> Self {
        Self { size, bv: bitvec![bit as i32; size.count() as usize] }
    }

    pub fn iter(&self) -> Iter<'_, usize, Lsb0> {
        self.bv.iter()
    }

    #[allow(dead_code)]
    pub fn iter_mut(&mut self) -> IterMut<'_, usize, Lsb0> {
        self.bv.iter_mut()
    }

    pub fn as_bitslice(&mut self) -> &BitSlice {
        self.bv.as_bitslice()
    }

    #[allow(dead_code)]
    pub fn as_mut_bitslice(&mut self) -> &mut BitSlice {
        self.bv.as_mut_bitslice()
    }

    /// Reset all elements to false.
    #[inline]
    pub fn zero_out_bits(&mut self) {
        self.bv.set_elements(0);
    }

    #[allow(dead_code)]
    #[inline]
    pub fn apply_all_bits(&mut self, bit: bool) {
        self.bv.set_elements(bit as usize);
    }

    #[inline]
    pub fn copy_from_slice(&mut self, slice: &BitSlice) {
        self.bv.copy_from_bitslice(slice);
    }

    /// Get the bool at the given x and y.
    ///
    /// Returns false if out of bounds.
    #[inline]
    pub fn get_bit(&self, idx: usize) -> bool {
        let pt = self.index_to_coord2d(idx);
        if !self.in_bounds(pt) {
            false
        } else {
            self.bv[idx]
        }
    }

    /// Set the bool at the given x and y to value.
    ///
    /// Panics if out of bounds.
    #[inline]
    pub fn set_bit(&mut self, idx: usize, value: bool) {
        self.bv.set(idx, value);
    }
}

impl std::ops::Index<usize> for BitGrid {
    type Output = bool;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.bv[idx]
    }
}
