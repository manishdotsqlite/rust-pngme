
use crate::chunk_type::ChunkType;
use crc::Crc;


pub struct Chunk {
        length: u32, 
        chunk_type: ChunkType,
        data: Vec<u8>,
        crc: u32
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Result<Chunk, &'static str> {
        const MAX_LENGTH: usize = 1 << 31;
        if data.len() >= MAX_LENGTH {
                Err("The data length exceeds the PNG limit.")
        } else {
                const PNG_CRC: Crc<u32> = Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);
                let mut digest = PNG_CRC.digest();
                digest.update(&chunk_type.bytes());
                digest.update(&data);
                let crc = digest.finalize();

                Ok(Chunk {
                        length: data.len() as u32,
                        chunk_type,
                        data, 
                        crc
                })
        }
    }

        pub fn length(&self) -> u32 {
            self.length
        }

        pub fn chunk_type(&self) -> &ChunkType {
            &self.chunk_type
        }

        pub fn data(&self) -> &[u8] {
            &self.data
        }

        pub fn crc(&self) -> u32 {
            self.crc
        }

        fn data_as_string(&self) -> String {
            String::from_utf8_lossy(&self.data).to_string()
        }

        pub fn display_chunk(&self) {
            println!("Length: {}", self.length);
            self.chunk_type.display_chunk();
            println!("Data: {}", self.data_as_string());
            println!("CRC: {:#010x}", self.crc);
        }

        fn as_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.extend_from_slice(&self.length.to_be_bytes());
            bytes.extend_from_slice(self.chunk_type.bytes());
            bytes.extend_from_slice(&self.data);
            bytes.extend_from_slice(&self.crc.to_be_bytes());
            bytes
        }

    }
