#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate nom;

mod parser;
mod types;
mod errors;
mod keywords;

use std::path::PathBuf;
use types::{Config, FileDescriptor};

use errors::*;

pub fn generate(in_files: Vec<PathBuf>, import_search_path: Vec<PathBuf>, output_dir: &str) -> Result<()> {
    if in_files.is_empty() {
        bail!("no .proto files provided!");
    }

    for f in &in_files {
        if !f.exists() {
            bail!(format!("input file {:?} does not exist", f));
        }
    }

    for in_file in in_files {
        let mut out_file = in_file.with_extension("rs");

        let mut directory = PathBuf::from(output_dir);
        if !directory.is_dir() {
            // we can create? But only last dir
            bail!(format!("output directory {:?} does not exist", directory));
        }
        directory.push(out_file.file_name().unwrap());
        out_file = directory;

        let config = Config {
            in_file: in_file,
            out_file: out_file,
            single_module: false,
            import_search_path: import_search_path.clone(),
            no_output: false,
        };

        FileDescriptor::write_proto(&config).chain_err(|| {
            format!(
                "Could not convert {} into {}",
                config.in_file.display(),
                config.out_file.display()
            )
        })?;
    }
    Ok(())
}

