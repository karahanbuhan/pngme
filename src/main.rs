use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

use clap::{Parser, Subcommand};

use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;

mod chunk;
mod chunk_type;
mod png;
mod util;

/// Hide secret messages in PNG files.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
#[command(arg_required_else_help(true))]
enum Commands {
    /// Encode the secret message in the chunk.
    Encode {
        // Path of target PNG file to encode the secret message
        file: PathBuf,
        chunk_type: String,
        message: String,
    },

    /// Decode the secret message in the chunk.
    Decode {
        // Path of target PNG file to decode the secret message
        file: PathBuf,
        chunk_type: String,
    },

    /// Remove a chunk by its type.
    Remove {
        // Path of target PNG file to remove chunk
        file: PathBuf,
        chunk_type: String,
    },

    /// Print all chunks inside the PNG file.
    Print {
        /// Path of target PNG file to print its contents
        file: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Encode {
            file,
            chunk_type,
            message,
        }) => {
            let mut png = Png::from(file);
            png.append_chunk(Chunk::new(
                ChunkType::from_str(chunk_type).expect("Cannot create chunk"),
                message.clone().into_bytes(),
            ));
            fs::write(file, png.as_bytes()).expect("Cannot write PNG file");

            println!("Successfully added a secret message to file");
        }

        Some(Commands::Decode { file, chunk_type }) => {
            let png = Png::from(file);
            let chunk = png.chunk_by_type(chunk_type).expect("Chunk does not exist");

            println!(
                "{}",
                chunk
                    .data_as_string()
                    .expect("Chunk data is not in UTF-8 format")
            );
        }

        Some(Commands::Remove { file, chunk_type }) => {
            let mut png = Png::from(file);
            let removed_chunk = png.remove_chunk(chunk_type).expect("Cannot remove chunk");
            fs::write(file, png.as_bytes()).expect("Cannot write PNG file");

            println!("Chunk {} is successfully removed!", removed_chunk.r#type);
        }

        Some(Commands::Print { file }) => {
            println!(
                "{}",
                Png::from(file)
                    .chunks
                    .into_iter()
                    .map(|chunk| chunk.r#type.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            );
        }

        None => {
            unreachable!();
        }
    };
}
