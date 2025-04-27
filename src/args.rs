use clap::{Parser, Subcommand};

#[command(author, version, about, long_about = None)]
#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Encode a message into a PNG file
    Encode {
        /// Path to the input PNG file
        #[arg(short, long)]
        file_path: String,

        /// Chunk type to encode the message under
        #[arg(short, long)]
        chunk_type: String,

        /// Message to encode
        #[arg(short, long)]
        message: String,

        /// Optional path to save the output PNG file
        #[arg(short, long)]
        output_file: Option<String>,
    },

    /// Decode a message from a PNG file
    Decode {
        /// Path to the PNG file
        #[arg(short, long)]
        file_path: String,

        /// Chunk type to decode the message from
        #[arg(short, long)]
        chunk_type: String,
    },

    /// Remove a chunk from a PNG file
    Remove {
        /// Path to the PNG file
        #[arg(short, long)]
        file_path: String,

        /// Chunk type to remove
        #[arg(short, long)]
        chunk_type: String,
    },

    /// Print all chunks in a PNG file
    Print {
        /// Path to the PNG file
        #[arg(short, long)]
        file_path: String,
    },
}