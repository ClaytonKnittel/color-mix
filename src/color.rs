pub trait IndexedEnum {
  const CARDINALITY: usize;

  fn to_idx(self) -> usize;
}

#[derive(Clone, Copy)]
pub enum PrimaryColor {
  Red,
  Yellow,
  Blue,
}

impl IndexedEnum for PrimaryColor {
  const CARDINALITY: usize = 3;

  fn to_idx(self) -> usize {
    match self {
      Self::Red => 0,
      Self::Yellow => 1,
      Self::Blue => 2,
    }
  }
}

#[derive(Clone, Copy)]
pub enum SecondaryColor {
  Orange,
  Purple,
  Green,
}

impl IndexedEnum for SecondaryColor {
  const CARDINALITY: usize = 3;

  fn to_idx(self) -> usize {
    match self {
      Self::Orange => 0,
      Self::Purple => 1,
      Self::Green => 2,
    }
  }
}

#[derive(Clone, Copy)]
pub enum Color {
  PrimaryColor(PrimaryColor),
  SecondaryColor(SecondaryColor),
}

impl IndexedEnum for Color {
  const CARDINALITY: usize = PrimaryColor::CARDINALITY + SecondaryColor::CARDINALITY;

  fn to_idx(self) -> usize {
    match self {
      Self::PrimaryColor(color) => color.to_idx(),
      Self::SecondaryColor(color) => PrimaryColor::CARDINALITY + color.to_idx(),
    }
  }
}
