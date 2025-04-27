mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;
use args::{Cli, Commands};
use chunk::Chunk;
use chunk_type::ChunkType;
use png::PNG;
use std::fs;
use std::io;
use clap::Parser;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Encode {
            file_path,
            chunk_type,
            message,
            output_file,
        } => {
            println!(
                "Encoding message '{}' into file '{}' under chunk type '{}'",
                message, file_path, chunk_type
            );
            
            // encoding with error handling
            let mut png_data = fs::read(&file_path)?;
            let chunk_type = ChunkType::new(&chunk_type)?;
            let chunk = Chunk::new(chunk_type, message.into_bytes())?;
            png_data.extend_from_slice(&chunk.as_bytes());
            
            if let Some(output_file) = output_file {
                fs::write(output_file, png_data)?;
            } else {
                fs::write("new_file.png", png_data)?;
            }

            println!("Message encoded successfully.");
        }

        Commands::Decode {
            file_path,
            chunk_type,
        } => {
            println!(
                "Decoding message from file '{}' under chunk type '{}'",
                file_path, chunk_type
            );
            
            let png_data = fs::read(file_path)?;
            let mut png = PNG::from_bytes(&png_data)?;
            let chunktype: ChunkType = ChunkType::new(&chunk_type)?;
            let chunks = png.chunks();
            let chunk = chunks.iter().find(|c| c.chunk_type() == &chunktype);
            if chunk.is_none() {
                println!("No chunk found with type '{}'", chunk_type);
                return Ok(());
            }

            let chunk = chunk.unwrap();
            let data = chunk.data();
            let message = String::from_utf8_lossy(data);
            println!("Decoded message: {}", message);
        }

        Commands::Remove {
            file_path,
            chunk_type,
        } => {
            println!(
                "Removing chunk type '{}' from file '{}'",
                chunk_type, file_path
            );
            let mut png_data = fs::read(&file_path)?;
            let mut png = PNG::from_bytes(&png_data)?;
            let chunk = png.remove_first_chunk(&chunk_type);
            if chunk.is_err() {
                println!("No chunk found with type '{}'", chunk_type);
                return Ok(());
            }
            let chunk = chunk.unwrap();
            let data = chunk.data();
            let message = String::from_utf8_lossy(data);
            println!("Removed chunk: {}", message);
            png_data = png.as_bytes();
            fs::write(&file_path, png_data)?;
            println!("Chunk removed successfully.");

        }

        Commands::Print { file_path } => {
            println!("Printing all chunks in file '{}'", file_path);
            
            let png_data = fs::read(file_path)?;
            let png = PNG::from_bytes(&png_data)?;
            let chunks = png.chunks();
            for chunk in chunks {
                println!("Chunk type: {:?}", chunk.chunk_type());
                println!("Data: {:?}", String::from_utf8_lossy(chunk.data()));
                println!("CRC: {:#010x}", chunk.crc());
                println!("-----------------------------");
            }
        }
    }


    Ok(())
}