# csv2xlsx

Convert CSV files to Excel (in XLSX format). Written in Rust for performance and safety.

Its main intended usage is through the provided CLI, but programmatic access should also work (which outputs in-memory Excel-data).

## CLI usage

### Simple convert with explicit filenames provided
```
csv2xlsx -i ~/example.csv -o ~/example.xlsx
```

### With column widths adjusted to their contents
```
cat ~/example.csv | csv2xlsx --width-adjustment > ~/example.xlsx
```

### CLI help for more info
```
csv2xlsx --help
```
