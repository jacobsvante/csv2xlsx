use csv2xlsx::{
    cli::{self, Subcommand},
    csv2xlsx, Options,
};
use std::{fs::File, io::Write};

fn main() -> csv2xlsx::Result<()> {
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
            Options {
                delimiter: opts.delimiter,
                width_adjustment: opts.width_adjustment,
                sheet_name: opts.sheet_name,
                explicit_column_types_map: opts.explicit_column_types.into(),
            },
        )?;
        let mut out = File::options()
            .write(true)
            .create(true)
            .open(opts.output_file)?;
        out.write_all(&data)?;
        Ok(())
    }
}
