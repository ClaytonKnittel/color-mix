pub struct Bank {
  colors: ColorSet,
}

impl Default for Bank {
  fn default() -> Self {
    Self { colors: ColorSet::new() }
  }
}
