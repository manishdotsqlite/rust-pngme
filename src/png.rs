use crate::{chunk::Chunk, chunk_type::ChunkType};

#[derive(Debug)]
pub struct PNG {
        chunks:  Vec<Chunk>
}

impl PNG {
        pub const STANDARD_HEADER: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];

        pub fn from_chunks(chunks: Vec<Chunk>) -> Result<PNG, &'static str> {
                if chunks.is_empty() {
                        Err("The PNG file must contain at least one chunk.")
                } else {
                        Ok(PNG { chunks: chunks })
                }
        }

        pub fn from_bytes(bytes: &[u8]) -> Result<PNG, &'static str> {
                if bytes.len() < 8 || &bytes[..8] != PNG::STANDARD_HEADER {
                    return Err("Invalid PNG header");
                }
        
                let mut chunks = Vec::new();
                let mut offset = 8;
        
                while offset < bytes.len() {
                    // Read the length of the chunk (4 bytes)
                    if offset + 4 > bytes.len() {
                        return Err("Incomplete chunk length");
                    }
                    let length = u32::from_be_bytes(
                        bytes[offset..offset + 4]
                            .try_into()
                            .map_err(|_| "Failed to read chunk length")?,
                    );
                    offset += 4;
        
                    // Read the chunk type (4 bytes)
                    if offset + 4 > bytes.len() {
                        return Err("Incomplete chunk type");
                    }
                    let chunk_type_bytes: [u8; 4] = bytes[offset..offset + 4]
                        .try_into()
                        .map_err(|_| "Failed to read chunk type")?;
                    let chunk_type = ChunkType::new(std::str::from_utf8(&chunk_type_bytes).unwrap())
                        .map_err(|_| "Invalid chunk type")?;
                    offset += 4;
        
                    // Read the chunk data (length bytes)
                    if offset + length as usize > bytes.len() {
                        return Err("Incomplete chunk data");
                    }
                    let data = bytes[offset..offset + length as usize].to_vec();
                    offset += length as usize;
        
                    // Read the CRC (4 bytes)
                    if offset + 4 > bytes.len() {
                        return Err("Incomplete chunk CRC");
                    }
                    let crc = u32::from_be_bytes(
                        bytes[offset..offset + 4]
                            .try_into()
                            .map_err(|_| "Failed to read chunk CRC")?,
                    );
                    offset += 4;
        
                    // Create the chunk and add it to the list
                    let chunk = Chunk::new(chunk_type, data).map_err(|_| "Failed to create chunk")?;
                    if chunk.crc() != crc {
                        return Err("CRC mismatch");
                    }
                    chunks.push(chunk);
                }
        
                PNG::from_chunks(chunks)
            }

        pub fn append_chunk(&mut self, chunk: Chunk) {
                self.chunks.push(chunk);
        } 

        pub fn remove_first_chunk(&mut self, chunk_type: &str) -> Result<Chunk, &'static str> {
                let target_bytes = chunk_type.as_bytes();
                if target_bytes.len() != 4 {
                    return Err("Invalid chunk type length");
                }
            
                if let Some(index) = self.chunks.iter().position(|chunk| {
                    chunk.chunk_type().bytes() == target_bytes
                }) {
                    Ok(self.chunks.remove(index))
                } else {
                    // No matching chunk found
                    Err("Chunk not found")
                }
            }

        pub fn header(&self) -> &[u8; 8] {
               &Self::STANDARD_HEADER
        }

        pub fn chunks(&self) -> &[Chunk] {
                &self.chunks
        }

        pub fn chunk_by_type(&self, chunk_type: &str) -> Option<&Chunk> {
                let target_type = match ChunkType::new(chunk_type) {
                        Ok(chunktype) => chunktype,
                        Err(_) => return None,
                };

                self.chunks.iter().find(|chunk| *chunk.chunk_type() == target_type)
        }

        pub fn as_bytes(&self) -> Vec<u8> {
                let mut bytes = Vec::new();
                bytes.extend_from_slice(&Self::STANDARD_HEADER);
                for chunk in &self.chunks {
                        bytes.extend_from_slice(&chunk.length().to_be_bytes());
                        bytes.extend_from_slice(chunk.chunk_type().bytes());
                        bytes.extend_from_slice(chunk.data());
                        bytes.extend_from_slice(&chunk.crc().to_be_bytes());
                }
                bytes
        }

        
        pub fn find_chunk(&self, chunk_type: &str) -> Option<&Chunk> {
                let target_bytes = chunk_type.as_bytes();
                if target_bytes.len() != 4 {
                    return None;
                }
            
                self.chunks.iter().find(|chunk| chunk.chunk_type().bytes() == target_bytes)
            }

}

