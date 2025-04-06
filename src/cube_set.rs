use std::{
  fmt::{Debug, Display},
  marker::PhantomData,
  ops::Index,
};

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

  pub fn insert_n(&mut self, color: C, count: u32) {
    self.count[color.to_idx()] += count;
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

  pub fn empty(&self) -> bool {
    self.count.iter().all(|&count| count == 0)
  }

  pub fn clear(&mut self) {
    for count in &mut self.count {
      *count = 0;
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

impl<C: IndexedEnum, const N: usize> Display for ColorSet<C, N>
where
  C: Display,
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut first = true;
    for (idx, &count) in self.count.iter().enumerate() {
      if count != 0 {
        if !first {
          write!(f, " ")?;
        }
        write!(f, "{}", C::from_idx(idx).unwrap())?;
        if count > 1 {
          write!(f, "({})", count)?;
        }
        first = false;
      }
    }
    Ok(())
  }
}

pub type PrimaryColorSet = ColorSet<PrimaryColor, { PrimaryColor::CARDINALITY }>;
pub type CubeSet = ColorSet<Color, { Color::CARDINALITY }>;
