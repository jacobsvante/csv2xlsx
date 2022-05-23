use csv2xlsx::{cli, csv2xlsx};
use std::{
    fs::File,
    io::{stdout, Write},
};

fn main() -> anyhow::Result<()> {
    let opts = cli::parse();
    let out = csv2xlsx(
        File::open(opts.input_file)?,
        Some(opts.delimiter),
        opts.width_adjustment,
    )?;
    stdout().write_all(&out)?;
    Ok(())
}
