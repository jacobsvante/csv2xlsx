#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Provided delimiter is invalid")]
    InvalidDelimiter,
    #[error(transparent)]
    CsvError(#[from] ::csv::Error),
    #[error(transparent)]
    IoError(#[from] ::std::io::Error),
    #[error("Failed to parse explicit column type format from string (must follow: <column_index>=<cell_type>)")]
    UnparsableExplicitColumnType,
    #[error("Failed to parse cell type from string (should be one of: string, number, bool)")]
    UnparsableExplicitCellType,
    #[error("Failed to parse float as number")]
    UnparsableNumber,
    #[error("Failed to parse as boolean")]
    UnparsableBool,
    #[error("There was no explicit column type for the given index")]
    UnmatchedExplicitColumnTypeIndex,
}

pub type Result<T> = std::result::Result<T, Error>;
