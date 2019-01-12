#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate nom;

extern crate glob;

mod parser;
mod types;
mod errors;
mod keywords;

use std::path::PathBuf;
use std::env;
use types::{Config, FileDescriptor};

use glob::glob;

use errors::*;

pub fn generate_by_dirlist(base: &str, subfolders: &[&str]) -> Result<()> {
    cargo_rerun(base);

    for subf in subfolders {
        generate_by_dir(base, subf)?;
    }

    Ok(())
}

pub fn generate_by_dir(base: &str, path: &str) -> Result<()> {
    println!("\nGenerating for [{:?}]", path);

    let mut pb_files: Vec<PathBuf> = vec![];

    let pattern = format!("{base}/{folder}/**/*.proto", base = base, folder = path);
    for entry in glob(&pattern).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => pb_files.push(path),
            Err(e) => println!("cargo:warning={:?}", e),
        }
    }

    let root_out_dir = env::var("OUT_DIR").unwrap();
    let out_dir = format!("{root}/{folder}", root = root_out_dir, folder = path);
    std::fs::create_dir_all(&out_dir)?;

    let crate_root = env::var("CARGO_MANIFEST_DIR").unwrap();
    let import = format!(
        "{root}/{base}/{folder}",
        root = crate_root,
        base = base,
        folder = path
    );

    generate(pb_files, vec![import.into()], &out_dir)
}

pub fn cargo_rerun(base: &str) {
    println!("cargo:rerun-if-changed={base}", base = base);
    let pattern = format!("{base}/**/*", base = base);

    for entry in glob(&pattern).expect("Failed to read glob pattern") {
        if let Some(path) = entry
            .ok()
            .and_then(|path| path.to_str().map(|x| x.to_owned()))
        {
            println!("cargo:rerun-if-changed={}", path);
        }
    }
}

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

