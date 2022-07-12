use std::{collections::HashMap, str::FromStr};

use simple_excel_writer::{CellValue, ToCellValue};

use crate::{constants, Error, Result};

#[derive(Default)]
pub struct Options {
    pub delimiter: char,
    pub width_adjustment: bool,
    pub sheet_name: String,
    pub explicit_column_types_map: ExplicitColumnTypesMap,
}

impl Options {
    pub fn new() -> Self {
        Self {
            delimiter: constants::DEFAULT_DELIMITER,
            width_adjustment: constants::DEFAULT_WIDTH_ADJUSTMENT,
            sheet_name: constants::DEFAULT_SHEET_NAME.to_string(),
            explicit_column_types_map: ExplicitColumnTypesMap::default(),
        }
    }

    pub fn with_delimiter(mut self, delimiter: char) -> Self {
        self.delimiter = delimiter;
        self
    }

    pub fn with_width_adjustment(mut self, width_adjustment: bool) -> Self {
        self.width_adjustment = width_adjustment;
        self
    }

    pub fn with_sheet_name(mut self, sheet_name: String) -> Self {
        self.sheet_name = sheet_name;
        self
    }

    pub fn with_explicit_column_types(
        mut self,
        explicit_column_types: ExplicitColumnTypesMap,
    ) -> Self {
        self.explicit_column_types_map = explicit_column_types;
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum ExplicitCellType {
    Bool,
    Number,
    String,
}

impl FromStr for ExplicitCellType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_ascii_lowercase().as_str() {
            "bool" => Ok(Self::Bool),
            "number" => Ok(Self::Number),
            "string" => Ok(Self::String),
            _ => Err(Error::UnparsableExplicitCellType),
        }
    }
}

#[derive(Debug)]
pub struct ExplicitColumnType(u16, ExplicitCellType);

impl FromStr for ExplicitColumnType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.split_once('=') {
            Some((idx, cell_type)) => {
                let idx: u16 = idx
                    .parse()
                    .map_err(|_| Error::UnparsableExplicitColumnType)?;
                Ok(Self(idx, ExplicitCellType::from_str(cell_type)?))
            }
            None => Err(Error::UnparsableExplicitColumnType),
        }
    }
}

#[derive(Debug, Default)]
pub struct ExplicitColumnTypesMap(HashMap<u16, ExplicitColumnType>);

impl From<Vec<ExplicitColumnType>> for ExplicitColumnTypesMap {
    fn from(vec: Vec<ExplicitColumnType>) -> Self {
        let mut map = HashMap::with_capacity(vec.len());
        for ect in vec {
            map.insert(ect.0, ect);
        }
        Self(map)
    }
}

impl ExplicitColumnTypesMap {
    pub(crate) fn get(&self, column_index: u16) -> Option<ExplicitCellType> {
        self.0.get(&column_index).map(|ect| ect.1)
    }

    pub(crate) fn to_cell_value(&self, column_index: u16, value: &str) -> Result<CellValue> {
        if let Some(ect) = self.get(column_index) {
            Ok(match ect {
                ExplicitCellType::Number => value
                    .parse::<f64>()
                    .map_err(|_| Error::UnparsableNumber)?
                    .to_cell_value(),
                ExplicitCellType::Bool => value
                    .parse::<bool>()
                    .map_err(|_| Error::UnparsableBool)?
                    .to_cell_value(),
                ExplicitCellType::String => value.to_cell_value(),
            })
        } else {
            Err(Error::UnmatchedExplicitColumnTypeIndex)
        }
    }
}
