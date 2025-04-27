PNG Message Encoder/Decoder

A command-line tool to encode and decode secret messages into PNG files using custom chunks. This tool allows you to embed text into PNG metadata, retrieve it later, or remove it entirely without altering the visual appearance of the image.

Features

    Encode : Embed a secret message into a PNG file as a custom chunk (e.g., tEXt).
    Decode : Extract a secret message from a PNG file.
    Remove : Remove a specific chunk from a PNG file.
    Print : Display all chunks in a PNG file for inspection.
    Non-Destructive : The tool ensures the visual integrity of the PNG file remains unchanged.

Installation
Prerequisites

    Rust (version 1.70 or higher)
    Cargo (Rust's package manager)

Steps

    Clone this repository:
    bash

git clone https://github.com/manishdotsqlite/rust-pngme.git
cd png-message-tool

Build the project:

bash

        cargo build --release

Run the tool:
bash

    ./target/release/png-message-tool --help

Alternatively, install it globally:
bash

cargo install --path .

Usage

The tool provides four main commands: encode, decode, remove, and print.
Encode a Message

Embed a secret message into a PNG file.
bash

        png-message-tool encode \
                --file-path: Path to the input PNG file.
                --chunk-type: Type of chunk to encode the message under (e.g., tEXt).
                --message: The message to encode.
                --output-file: (Optional) Path to save the modified PNG file. Defaults to output_encoded.png.

Decode a Message

Extract a secret message from a PNG file.
bash

        png-message-tool decode \
                --file-path output_encoded.png \
                --chunk-type tEXt

                --file-path: Path to the PNG file containing the encoded message.
                --chunk-type: Type of chunk to decode the message from.

Remove a Chunk

Remove a specific chunk from a PNG file.
bash

        png-message-tool remove \
                --file-path output_encoded.png \
                --chunk-type tEXt

                --file-path: Path to the PNG file.
                --chunk-type: Type of chunk to remove.

Print All Chunks

Display all chunks in a PNG file for inspection.
bash

        png-message-tool print \
                --file-path input.png

                --file-path: Path to the PNG file.
