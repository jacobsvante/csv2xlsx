use std::{fs::File, io::{stdout, Write}};
use csv2xlsx::{cli, csv2xlsx};

fn main() -> anyhow::Result<()> {
    let opts = cli::parse();
    let out = csv2xlsx(
        File::open(opts.input_file)?,
        Some(opts.delimiter),
        opts.width_adjustment,
    )?;
    stdout().write(&out)?;
    Ok(())
}
