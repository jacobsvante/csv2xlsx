use csv::StringRecord;
use simple_excel_writer::{Column, Row, Sheet, Workbook};
use std::io::Read;

pub mod cli;
mod constants;
mod errors;
mod options;

pub use errors::*;
pub use options::*;

fn parse_delimiter(d: char) -> Result<u8> {
    u8::try_from(u32::from(d)).map_err(|_| Error::InvalidDelimiter)
}

/// Reads input as CSV and returns Excel data as bytes
pub fn csv2xlsx<I: Read>(input: I, options: Options) -> Result<Vec<u8>> {
    let delimiter = parse_delimiter(options.delimiter)?;

    let mut reader = csv::ReaderBuilder::new()
        .delimiter(delimiter)
        .has_headers(false)
        .from_reader(input);

    let mut workbook = Workbook::create_in_memory();

    let mut sheet = workbook.create_sheet(&options.sheet_name);

    let mut records = Vec::new();
    for result in reader.records() {
        let record = result?;
        records.push(record);
    }

    if options.width_adjustment {
        adjust_column_widths(&mut sheet, &records)?;
    }

    workbook.write_sheet(&mut sheet, |sheet_writer| {
        for record in records.iter() {
            let mut row = Row::new();
            for (col_idx, value) in record.into_iter().enumerate() {
                let value = String::from_utf8(strip_ansi_escapes::strip(value)?)
                    .expect("stripping ANSI escapes from a UTF-8 string always results in UTF-8");
                if let Ok(cell_value) = options
                    .explicit_column_types_map
                    .to_cell_value(col_idx as u16, &value)
                {
                    row.add_cell(cell_value);
                } else {
                    row.add_cell(value);
                }
            }
            sheet_writer.append_row(row)?;
        }
        Ok(())
    })?;

    let output = workbook.close()?.unwrap();
    Ok(output)
}

fn adjust_column_widths(sheet: &mut Sheet, records: &Vec<StringRecord>) -> Result<()> {
    if records.is_empty() {
        return Ok(());
    }

    let record0 = &records[0];
    // + 2 is to adjust first row for autofilter icons
    let mut max_chars: Vec<usize> = record0
        .iter()
        .map(|cell| cell.char_indices().count() + 3)
        .collect();

    for record in records {
        for (cell_idx, cell) in record.iter().enumerate() {
            let char_count = cell.char_indices().count();
            let current_max = max_chars.get(cell_idx).unwrap_or(&0).to_owned();
            if char_count > current_max {
                max_chars[cell_idx] = char_count;
            }
        }
    }

    for max in max_chars {
        sheet.add_column(Column { width: max as f32 })
    }
    Ok(())
}
