// src/main.rs
// Command-line interface for NoServerAI

mod memory_mapper;

use std::env;
use memory_mapper::{split_model, merge_model};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("Usage:");
        println!("  noserverai split <model_path> <chunk_size_mb>");
        println!("  noserverai merge <output_path> <chunk1> <chunk2> ...");
        return;
    }

    match args[1].as_str() {
        "split" => {
            if args.len() < 4 {
                println!("Error: Need model path and chunk size");
                return;
            }
            let model_path = &args[2];
            let chunk_size = args[3].parse::<usize>().unwrap_or(100);
            
            match split_model(model_path, chunk_size) {
                Ok(chunks) => {
                    println!("Created {} chunks:", chunks.len());
                    for chunk in chunks {
                        println!("  {}", chunk);
                    }
                }
                Err(e) => println!("Error: {}", e),
            }
        }
        "merge" => {
            if args.len() < 4 {
                println!("Error: Need output path and at least one chunk");
                return;
            }
            let output_path = &args[2];
            let chunk_paths: Vec<String> = args[3..].to_vec();
            
            match merge_model(&chunk_paths, output_path) {
                Ok(_) => println!("Successfully merged into {}", output_path),
                Err(e) => println!("Error: {}", e),
            }
        }
        _ => println!("Unknown command. Use 'split' or 'merge'"),
    }
}
