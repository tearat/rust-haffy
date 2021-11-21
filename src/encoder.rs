use crate::types::Char;
use std::str::Split;

pub fn encode(text: &str) -> (String, Vec<Char>) {
  let alphabet = create_alphabet(text.split(""));
  let mut encoded = String::from("");

  for character in text.split("") {
    if character.is_empty() {
      continue;
    }
    let index = alphabet.iter().position(|_char| _char.title == character).expect("Char not found in alphabet");
    encoded += &alphabet[index].code;
  }
  (encoded, alphabet)
}

fn create_alphabet(all_characters: Split<&str>) -> Vec<Char> {
  let mut alphabet: Vec<Char> = Vec::new();
  
  for character in all_characters {
    add_or_increment(&mut alphabet, character)
  }

  alphabet.sort_by_key(|s| s.count);
  alphabet.reverse();

  set_codes(&mut alphabet);
  
  alphabet
}

fn add_or_increment(alphabet: &mut Vec<Char>, character: &str) {
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

fn set_codes(alphabet: &mut Vec<Char>) {
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