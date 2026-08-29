# python/memory_mapper.py
import subprocess
import sys
from pathlib import Path

def split_model(model_path: str, chunk_size_mb: int = 100) -> list[str]:
    """
    Split a large model file into smaller chunks using the Rust CLI.
    
    Args:
        model_path: Path to the model file
        chunk_size_mb: Size of each chunk in MB (default 100)
    
    Returns:
        List of chunk file paths
    """
    # Build the CLI if not already built
    cli_path = Path(__file__).parent.parent / "target" / "debug" / "cli"
    if not cli_path.exists():
        subprocess.run(["cargo", "build"], cwd=Path(__file__).parent.parent, check=True)
    
    result = subprocess.run(
        [str(cli_path), "split", model_path, str(chunk_size_mb)],
        capture_output=True,
        text=True
    )
    
    if result.returncode != 0:
        raise RuntimeError(f"Split failed: {result.stderr}")
    
    # Parse output to get chunk paths
    lines = result.stdout.strip().split('\n')
    return [line.strip() for line in lines if line.startswith('  ') or line.endswith('.part')]

def merge_model(chunk_paths: list[str], output_path: str) -> None:
    """
    Merge chunks back into the original model file.
    
    Args:
        chunk_paths: List of chunk file paths
        output_path: Path for the merged model
    """
    cli_path = Path(__file__).parent.parent / "target" / "debug" / "cli"
    if not cli_path.exists():
        subprocess.run(["cargo", "build"], cwd=Path(__file__).parent.parent, check=True)
    
    result = subprocess.run(
        [str(cli_path), "merge", output_path] + chunk_paths,
        capture_output=True,
        text=True
    )
    
    if result.returncode != 0:
        raise RuntimeError(f"Merge failed: {result.stderr}")
