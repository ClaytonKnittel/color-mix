use std::{marker::PhantomData, ops::Index};

use crate::color::{Color, IndexedEnum, PrimaryColor};

pub struct ColorSet<C: IndexedEnum, const N: usize> {
  count: [u32; N],
  _phantom: PhantomData<C>,
}

impl<C: IndexedEnum, const N: usize> Default for ColorSet<C, N> {
  fn default() -> Self {
    Self { count: [0; N], _phantom: PhantomData }
  }
}

impl<C: IndexedEnum, const N: usize> Index<C> for ColorSet<C, N> {
  type Output = u32;

  fn index(&self, index: C) -> &u32 {
    &self.count[index.to_idx()]
  }
}

pub type PrimaryColorSet = ColorSet<PrimaryColor, { PrimaryColor::CARDINALITY }>;
pub type CubeSet = ColorSet<Color, { Color::CARDINALITY }>;
