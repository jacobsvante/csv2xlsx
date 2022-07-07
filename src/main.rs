use csv2xlsx::{
    cli::{self, Subcommand},
    csv2xlsx, Options,
};
use std::{fs::File, io::Write};

fn main() -> anyhow::Result<()> {
    let opts = cli::parse();
    if let Some(subcmd) = opts.subcommand {
        match subcmd {
            Subcommand::Version => {
                println!("{}", env!("CARGO_PKG_VERSION"));
                Ok(())
            }
        }
    } else {
        let data = csv2xlsx(
            File::open(opts.input_file)?,
            Options::new(
                Some(opts.delimiter),
                Some(opts.width_adjustment),
                Some(opts.sheet_name),
            ),
        )?;
        let mut out = File::options()
            .write(true)
            .create(true)
            .open(opts.output_file)?;
        out.write_all(&data)?;
        Ok(())
    }
}
