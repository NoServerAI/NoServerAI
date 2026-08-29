// src/memory_mapper.rs
// This module handles splitting large model files into smaller chunks
// and merging them back when needed.

use std::fs::File;
use std::io::{Read, Write, Result};
use std::path::Path;

/// Splits a large model file into smaller chunks of specified size (in MB)
/// Returns a vector of chunk file paths
pub fn split_model(model_path: &str, chunk_size_mb: usize) -> Result<Vec<String>> {
    let chunk_size_bytes = chunk_size_mb * 1024 * 1024;
    let mut input_file = File::open(model_path)?;
    let mut buffer = vec![0u8; chunk_size_bytes];
    let mut chunk_index = 0;
    let mut chunk_paths = Vec::new();

    loop {
        let bytes_read = input_file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        
        let chunk_path = format!("{}.part{}", model_path, chunk_index);
        let mut output_file = File::create(&chunk_path)?;
        output_file.write_all(&buffer[..bytes_read])?;
        
        chunk_paths.push(chunk_path);
        chunk_index += 1;
    }

    Ok(chunk_paths)
}

/// Merges chunk files back into the original model file
pub fn merge_model(chunk_paths: &[String], output_path: &str) -> Result<()> {
    let mut output_file = File::create(output_path)?;
    for chunk_path in chunk_paths {
        let mut chunk_file = File::open(chunk_path)?;
        let mut buffer = Vec::new();
        chunk_file.read_to_end(&mut buffer)?;
        output_file.write_all(&buffer)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_and_merge() {
        let test_content = b"Hello, this is a test model!";
        let test_path = "test_model.bin";
        
        // Create test file
        {
            let mut file = File::create(test_path).unwrap();
            file.write_all(test_content).unwrap();
        }

        // Split into chunks (1 MB each, but test content is smaller)
        let chunks = split_model(test_path, 1).unwrap();
        assert_eq!(chunks.len(), 1);

        // Merge back
        let merged_path = "merged_model.bin";
        merge_model(&chunks, merged_path).unwrap();

        // Verify content matches original
        let mut merged_file = File::open(merged_path).unwrap();
        let mut merged_content = Vec::new();
        merged_file.read_to_end(&mut merged_content).unwrap();
        assert_eq!(&merged_content, test_content);

        // Cleanup test files
        std::fs::remove_file(test_path).unwrap();
        for chunk in &chunks {
            std::fs::remove_file(chunk).unwrap();
        }
        std::fs::remove_file(merged_path).unwrap();
    }
}
