use std::fmt::Display;

pub trait IndexedEnum {
  const CARDINALITY: usize;

  fn to_idx(self) -> usize;

  fn from_idx(idx: usize) -> Option<Self>
  where
    Self: Sized;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimaryColor {
  Red,
  Yellow,
  Blue,
}

impl PrimaryColor {
  pub fn all_colors() -> impl Iterator<Item = Self> {
    PrimaryColorIter { val: Some(PrimaryColor::Red) }
  }

  pub fn mix(self, other: Self) -> Color {
    const COLORS: [Color; 9] = [
      Color::Red,
      Color::Orange,
      Color::Purple,
      Color::Orange,
      Color::Yellow,
      Color::Green,
      Color::Purple,
      Color::Green,
      Color::Blue,
    ];

    COLORS[self.to_idx() + other.to_idx() * Self::CARDINALITY]
  }

  #[cfg(test)]
  pub fn testonly_mix(self, other: Self) -> Color {
    match (self, other) {
      (Self::Red, Self::Red) => Color::Red,
      (Self::Yellow, Self::Yellow) => Color::Yellow,
      (Self::Blue, Self::Blue) => Color::Blue,
      (Self::Red, Self::Yellow) | (Self::Yellow, Self::Red) => Color::Orange,
      (Self::Red, Self::Blue) | (Self::Blue, Self::Red) => Color::Purple,
      (Self::Yellow, Self::Blue) | (Self::Blue, Self::Yellow) => Color::Green,
    }
  }
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

  fn from_idx(idx: usize) -> Option<Self> {
    match idx {
      0 => Some(Self::Red),
      1 => Some(Self::Yellow),
      2 => Some(Self::Blue),
      _ => None,
    }
  }
}

struct PrimaryColorIter {
  val: Option<PrimaryColor>,
}

impl Iterator for PrimaryColorIter {
  type Item = PrimaryColor;

  fn next(&mut self) -> Option<Self::Item> {
    let res = self.val;
    self.val = match self.val {
      Some(PrimaryColor::Red) => Some(PrimaryColor::Yellow),
      Some(PrimaryColor::Yellow) => Some(PrimaryColor::Blue),
      Some(PrimaryColor::Blue) | None => None,
    };
    res
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
  Red,
  Yellow,
  Blue,
  Orange,
  Purple,
  Green,
}

impl Color {
  pub fn decompose(self) -> &'static [PrimaryColor; 2] {
    match self {
      Self::Red => &[PrimaryColor::Red, PrimaryColor::Red],
      Self::Yellow => &[PrimaryColor::Yellow, PrimaryColor::Yellow],
      Self::Blue => &[PrimaryColor::Blue, PrimaryColor::Blue],
      Self::Orange => &[PrimaryColor::Red, PrimaryColor::Yellow],
      Self::Purple => &[PrimaryColor::Red, PrimaryColor::Blue],
      Self::Green => &[PrimaryColor::Yellow, PrimaryColor::Blue],
    }
  }

  pub fn decompose_iter(self) -> impl Iterator<Item = (PrimaryColor, u32)> {
    let mut counts = [0, 0, 0];
    for color in self.decompose() {
      counts[color.to_idx()] += 1;
    }
    counts
      .into_iter()
      .enumerate()
      .map(|(idx, count)| (PrimaryColor::from_idx(idx).unwrap(), count))
  }
}

impl Display for Color {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Self::Red => 'R',
        Self::Yellow => 'Y',
        Self::Blue => 'B',
        Self::Orange => 'O',
        Self::Purple => 'P',
        Self::Green => 'G',
      }
    )
  }
}

impl IndexedEnum for Color {
  const CARDINALITY: usize = 6;

  fn to_idx(self) -> usize {
    match self {
      Self::Red => 0,
      Self::Yellow => 1,
      Self::Blue => 2,
      Self::Orange => 3,
      Self::Purple => 4,
      Self::Green => 5,
    }
  }

  fn from_idx(idx: usize) -> Option<Self> {
    match idx {
      0 => Some(Self::Red),
      1 => Some(Self::Yellow),
      2 => Some(Self::Blue),
      3 => Some(Self::Orange),
      4 => Some(Self::Purple),
      5 => Some(Self::Green),
      _ => None,
    }
  }
}

impl From<PrimaryColor> for Color {
  fn from(value: PrimaryColor) -> Self {
    match value {
      PrimaryColor::Red => Self::Red,
      PrimaryColor::Yellow => Self::Yellow,
      PrimaryColor::Blue => Self::Blue,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::PrimaryColor;

  #[test]
  fn test_primary_color_iter() {
    assert_eq!(
      PrimaryColor::all_colors().collect::<Vec<_>>(),
      vec![PrimaryColor::Red, PrimaryColor::Yellow, PrimaryColor::Blue]
    );
  }

  #[test]
  fn test_mix() {
    for c1 in PrimaryColor::all_colors() {
      for c2 in PrimaryColor::all_colors() {
        assert_eq!(c1.mix(c2), c2.mix(c1));
        assert_eq!(c1.mix(c2), c1.testonly_mix(c2));
      }
    }
  }
}
