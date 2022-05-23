use csv2xlsx::{cli, csv2xlsx};
use std::{fs::File, io::Write};

fn main() -> anyhow::Result<()> {
    let opts = cli::parse();
    let data = csv2xlsx(
        File::open(opts.input_file)?,
        Some(opts.delimiter),
        opts.width_adjustment,
    )?;
    let mut out = File::options()
        .write(true)
        .create(true)
        .open(opts.output_file)?;
    out.write_all(&data)?;
    Ok(())
}
