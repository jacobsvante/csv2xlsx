# csv2xlsx

Convert CSV files to Excel (in XLSX format). Written in Rust for performance and safety.

Its main intended usage is through the provided CLI, but programmatic access should also work (which outputs in-memory Excel-data).

## CLI usage

### Adjust to CSV
```
cat ~/example.csv | csv2xlsx
```


### With column widths adjusted to their contents
```
cat ~/example.csv | csv2xlsx --width-adjustment
```

### CLI help for more info
```
csv2xlsx --help
```
