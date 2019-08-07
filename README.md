# Obliviate

Small utility to delete files older than a certain amount of days.

Works in its basic functionalities, however error handling is not perfect.

## Usage

```
Obliviate 0.1.0
Paolo Marino
Simple utility that removes file older than a cerain amount of days.

USAGE:
    obliviate [FLAGS] [OPTIONS] <PATH>

FLAGS:
    -d, --dry-run    When provided no files are deleted.
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v               Sets the level of verbosity

OPTIONS:
    -a, --age <AGE>    Number of days the file should be old to be removed. [default: 30]

ARGS:
    <PATH>    Path where to look for file to delete
```
