# Freemap MBPack

Tool to create MBTiles file from a directory with `/zoom/x/y.ext` structure.

## Building and installing

```sh
cargo install --path .
```

## Command options

Use `-h` or `--help` to get description of all available options:

```
Usage: freemap-mbpack [OPTIONS] <SOURCE_DIR> <TARGET_FILE>

Arguments:
  <SOURCE_DIR>   Input directory
  <TARGET_FILE>  Output *.mbtiles file

Options:
  -n, --name <NAME>      Name
  -s, --scheme <SCHEME>  Tile scheme in the directory [default: xyz] [possible values: xyz, tms]
  -v, --verbose          Verbose
  -h, --help             Print help
  -V, --version          Print version
```
