use crate::types::Alphabet;

pub fn decode(text: &str, alphabet: &Alphabet) -> String {
  let mut current_code = String::from("");
  let mut decoded = String::from("");
  for character in text.chars() {
    current_code.push_str(&character.to_string());
    let index = alphabet.iter().position(|_char| _char.code == current_code);
    match index {
      Some(index) => {
        decoded.push_str(&alphabet[index].title);
        current_code = String::from("");
      },
      None => {}
    }
  }

  decoded
}