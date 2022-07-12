use crate::{constants::*, options::ExplicitColumnType};
use crate::{Error, Result};
use clap::Parser;
use std::path::PathBuf;

pub fn parse() -> Opts {
    Opts::parse()
}

/// Convert CSV to Excel
#[derive(Debug, Parser)]
pub struct Opts {
    #[clap(short, long, parse(from_os_str), default_value = "/dev/stdin")]
    pub input_file: PathBuf,
    /// Where to write output. Defaults to standard output.
    #[clap(short, long, parse(from_os_str), default_value = "/dev/stdout")]
    pub output_file: PathBuf,
    /// Delimiter used in the CSV file
    #[clap(short, long, default_value_t = DEFAULT_DELIMITER, parse(try_from_str = unescape_delimiter))]
    pub delimiter: char,
    /// Automatically adjust widths for each column based on their content
    #[clap(short, long)]
    pub width_adjustment: bool,
    /// Specify a custom sheet name
    #[clap(short, long, default_value = DEFAULT_SHEET_NAME)]
    pub sheet_name: String,
    /// Specify explicit column types (0-index to type, which must be one that implements Type)
    #[clap(short, long)]
    pub explicit_column_types: Vec<ExplicitColumnType>,
    #[clap(subcommand)]
    pub subcommand: Option<Subcommand>,
}

#[derive(Debug, clap::Subcommand)]
pub enum Subcommand {
    Version,
}

fn unescape_delimiter(src: &str) -> Result<char> {
    if src.len() == 1 {
        let c: char = unescape::unescape(src)
            .ok_or(Error::InvalidDelimiter)?
            .chars()
            .next()
            .unwrap();
        Ok(c)
    } else {
        Err(Error::InvalidDelimiter)
    }
}
