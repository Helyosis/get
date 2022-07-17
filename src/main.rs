use clap::Parser;
use std::{path::{PathBuf}, fs::{copy, self}};
use tempfile::NamedTempFile;
use exitfailure::ExitFailure;
use std::ffi::OsStr;

pub mod download_file;
use download_file::download;

pub mod clean_file;
use clean_file::clean;

pub mod extract_files;
use extract_files::extract;

#[derive(Parser,Default,Debug)]
#[clap(author="Helyosis", version, about="A simple utility to download file from ctf platforms (root-me, ctfd)")]
struct Cli {
    url: String,

    #[clap(short, long, parse(from_os_str))]
    output: Option<PathBuf>,
}

fn main() -> Result<(), ExitFailure> {
    let args = Cli::parse();
    let tmp = NamedTempFile::new()?;

    download(&args.url, tmp.as_file()).unwrap();
    //println!("{:?}", args.url);

    let filename = match args.output {
        Some(val) => val,
        None => PathBuf::from(clean(&args.url)?),
    };

    // println!("{:?}", filename);

    copy(tmp, &filename)?;

    Ok(())
}
