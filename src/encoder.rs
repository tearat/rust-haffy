use crate::types::Alphabet;
use crate::types::Char;

pub fn encode(text: &str) -> (String, Alphabet) {
  let alphabet = create_alphabet(text);
  let mut encoded = String::from("");

  for character in text.chars() {
    let index = alphabet.iter().position(|_char| _char.title == character.to_string()).unwrap();
    encoded.push_str(&alphabet[index].code);
  }
  (encoded, alphabet)
}

fn create_alphabet(text: &str) -> Alphabet {
  let mut alphabet: Alphabet = Vec::new();
  
  for character in text.chars() {
    add_or_increment(&mut alphabet, &character.to_string())
  }

  alphabet.sort_by_key(|s| s.count);
  alphabet.reverse();

  set_codes(&mut alphabet);
  
  alphabet
}

fn add_or_increment(alphabet: &mut Alphabet, character: &str) {
  if character.is_empty() {
    return
  }
  let index = alphabet.iter().position(|_char| _char.title == character);
  match index {
      None => {
        alphabet.push(Char { 
          title: String::from(character), 
          count: 1,
          code: String::from("")
        });
      },
      Some(index) => {
        alphabet[index].count += 1;
      }
  }
}

fn set_codes(alphabet: &mut Alphabet) {
  let mut next_code = String::from("0");
  let length = alphabet.len();
  
  for (index, character) in alphabet.iter_mut().enumerate() {
    character.code = next_code.clone();
    if &(index+2) < &length {
      next_code = "1".to_string() + &next_code;
    } else {
      next_code = next_code.replace("0", "1");
    }
  };
}