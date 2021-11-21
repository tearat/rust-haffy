#[cfg(test)]
pub mod tests {
  use crate::decoder::decode;
  use crate::encoder::encode;

  #[test]
  pub fn test() -> () { 
    let (encoded, alphabet) = encode("TEST");
    assert_eq!(encoded, "011100");
    let decoded = decode(&encoded, &alphabet);
    assert_eq!(decoded, "TEST");
  }
}