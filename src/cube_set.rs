use std::{fmt::Debug, marker::PhantomData, ops::Index};

use crate::{
  color::{Color, IndexedEnum, PrimaryColor},
  error::{ColorMixError, ColorMixResult},
};

pub struct ColorSet<C: IndexedEnum, const N: usize> {
  count: [u32; N],
  _phantom: PhantomData<C>,
}

impl<C: IndexedEnum + Copy, const N: usize> ColorSet<C, N> {
  pub fn with_initial_count(count: u32) -> Self {
    Self { count: [count; N], _phantom: PhantomData }
  }

  pub fn insert(&mut self, color: C) {
    self.count[color.to_idx()] += 1;
  }

  pub fn capacity(&self, color: C) -> u32 {
    self.count[color.to_idx()]
  }

  pub fn remove(&mut self, color: C) -> ColorMixResult
  where
    C: Debug,
  {
    let idx = color.to_idx();
    if self.count[idx] != 0 {
      self.count[idx] -= 1;
      Ok(())
    } else {
      Err(
        ColorMixError::InvalidAction(format!("Player does not have color {color:?} remaining"))
          .into(),
      )
    }
  }
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
