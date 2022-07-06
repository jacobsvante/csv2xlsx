use std::io::Read;

use csv::StringRecord;
use simple_excel_writer::{Column, Row, Sheet, Workbook};

pub mod cli;

fn parse_delimiter(d: char) -> anyhow::Result<u8> {
    u8::try_from(u32::from(d)).map_err(anyhow::Error::from)
}

/// Reads input as CSV and returns Excel data as bytes
pub fn csv2xlsx<I: Read>(
    input: I,
    delimiter: Option<char>,
    width_adjustment: bool,
) -> anyhow::Result<Vec<u8>> {
    let delimiter = parse_delimiter(delimiter.unwrap_or(','))?;
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(delimiter)
        .has_headers(false)
        .from_reader(input);

    let mut workbook = Workbook::create_in_memory();

    let mut sheet = workbook.create_sheet("default");

    let mut records = Vec::new();
    for result in reader.records() {
        let record = result?;
        records.push(record);
    }

    if width_adjustment {
        adjust_column_widths(&mut sheet, &records)?;
    }

    workbook.write_sheet(&mut sheet, |sheet_writer| {
        for record in records {
            let mut row = Row::new();
            for value in record.into_iter() {
                row.add_cell(value);
            }
            sheet_writer.append_row(row)?;
        }
        Ok(())
    })?;

    let output = workbook.close()?.unwrap();
    Ok(output)
}

fn adjust_column_widths(sheet: &mut Sheet, records: &Vec<StringRecord>) -> anyhow::Result<()> {
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
