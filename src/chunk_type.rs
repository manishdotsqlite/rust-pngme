#[derive(Debug)]
pub struct ChunkType {
    chunk_type: [u8; 4],
}

impl ChunkType {
    pub fn new(value: &str) -> Result<ChunkType, &str> {
        let characters: Vec<char> = value.chars().collect();
        if characters.len() != 4 {
            Err("The string isn't 4 characters long.")
        } else if !characters.iter().all(|&c| c.is_alphabetic()) {
            Err("Not all characters in the string are alphabetic.")
        } else if characters[2].is_lowercase() {
            Err("The third character is lowercase.")
        } else {
            let bytes: [u8; 4] = [
                characters[0] as u8,
                characters[1] as u8,
                characters[2] as u8,
                characters[3] as u8,
            ];
            Ok(ChunkType { chunk_type: bytes })
        }
    }

    fn is_critical(&self) -> bool {
        let alphabet: char = self.chunk_type[0] as char;
        alphabet.is_uppercase()
    }

    fn is_public(&self) -> bool {
        let alphabet: char = self.chunk_type[1] as char;
        alphabet.is_uppercase()
    }

    fn is_reserved(&self) -> bool {
        let alphabet: char = self.chunk_type[2] as char;
        alphabet.is_uppercase()
    }

    fn is_safe_to_copy(&self) -> bool {
        let alphabet: char = self.chunk_type[3] as char;
        alphabet.is_uppercase()
    }

    pub fn display_chunk(&self) {
        let chunktype_string = String::from_utf8_lossy(&self.chunk_type);
        println!("Chunk type: {}", chunktype_string);
        println!("Critical: {}", self.is_critical());
        println!("Public: {}", self.is_public());
        println!("Safe to copy: {}", self.is_safe_to_copy());
    }

    pub fn bytes(&self) -> &[u8] {
        &self.chunk_type
    }
}

