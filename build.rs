use glob::glob;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_files: Vec<PathBuf> = glob("nitric/**/*.proto")?
    .filter_map(Result::ok)
    .collect();

    for proto_file in proto_files {
        tonic_build::configure().out_dir("src/nitric/proto").compile(&[proto_file], &["."])?;
    }
    
    Ok(())
}