# targ

## Summary

A very basic TUI application designed to explore TAR and GZIP-compressed TAR files.
It allows users to navigate the contents of these archives directly from the terminal.

## Usage

```shell
Usage: targ [OPTIONS] --tar-file <TAR_FILE>

Options:
  -t, --tar-file <TAR_FILE>  Path to the tar file
      --show-indicator       Show indicator files (e.g. ._* files)
  -d, --debug                Debug mode and save logs to a file, e.g. targ_debug.log
  -h, --help                 Print help
  -V, --version              Print version
```

## Demonstration

![demo](./assets/demo.png)
