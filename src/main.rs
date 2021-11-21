use haffy::decoder::decode;
use haffy::encoder::encode;
use std::fs;

// Binary encodes text from text.txt with Huffman process 
// and creates code and aplphabet for for the reverse process
// then decodes code to original text

fn main() {
    let text = fs::read_to_string("./file.txt").expect("Unable to read file");
    println!("file.txt: {} \n", text);

    let (encoded, alphabet) = encode(&text);

    println!("encoded: {} \n", encoded);
    println!("alphabet: {:#?} \n", alphabet);
    
    let decoded = decode(&encoded, &alphabet);
    println!("decoded: {}", decoded);
}