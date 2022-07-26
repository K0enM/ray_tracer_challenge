use crate::two_dimensional::TwoDimensional;

pub trait ToPPM {
  fn header(&self) -> Vec<u8>
  where
    Self: TwoDimensional
  {
    let mut header = Vec::new();
    header.extend(String::from("P3\n").into_bytes());
    header.extend(format!("{} {}\n", self.width(), self.height()).into_bytes());
    header.extend("255\n".to_owned().into_bytes());

    header
  }

  fn to_ppm(&self) -> Vec<u8>;
}