pub trait ToPNG {
  fn to_png(self) -> Vec<u8>;
}