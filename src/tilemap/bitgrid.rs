use crate::prelude::*;
use bitvec::{
    prelude::*,
    slice::{Iter, IterMut},
};
use serde::{Deserialize, Serialize};

/// A width-by-height&-sized BitVec for convenient handling of a grid of boolean values.
#[derive(Default, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BitGrid {
    width: i32,
    height: i32,
    bv: BitVec,
}

impl BitGrid {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }

    fn in_bounds(&self, pos: Point) -> bool {
        pos.x >= 0 && pos.x < self.width as i32 && pos.y >= 0 && pos.y < self.height as i32
    }

    /// Convert a Point (x/y) to an array index. Defaults to an index based on an array
    /// strided X first.
    #[allow(dead_code)]
    fn point2d_to_index(&self, pt: Point) -> usize {
        let bounds = self.dimensions();
        ((pt.y * bounds.x) + pt.x)
            .try_into()
            .expect("Not a valid usize. Did something go negative?")
    }

    /// Convert an array index to a point. Defaults to an index based on an array
    /// strided X first.
    fn index_to_point2d(&self, idx: usize) -> Point {
        let bounds = self.dimensions();
        let w: usize =
            bounds.x.try_into().expect("Not a valid usize. Did something go negative?");
        Point::new(idx % w, idx / w)
    }
}

impl BitGrid {
    /// Create a new BitGrid with the given width and height.
    pub fn new(width: i32, height: i32) -> Self {
        assert!(width >= 0);
        assert!(height >= 0);

        Self { width, height, bv: bitvec![0; (width * height) as usize] }
    }

    pub fn new_with_bit(width: i32, height: i32, bit: bool) -> Self {
        assert!(width >= 0);
        assert!(height >= 0);

        Self { width, height, bv: bitvec![bit as i32; (width * height) as usize] }
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
        let pt = self.index_to_point2d(idx);
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
