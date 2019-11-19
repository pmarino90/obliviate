# Obliviate

Small utility to delete files older than a certain amount of days.

Works in its basic functionalities, however error handling is not perfect.

## How to install

At the moment is possible to install using `cargo install obliviate` or building from source.
I am also thinking to upload the binary release on Github as well.

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

## Changelog

`0.1.2`

- Main usable version

`0.1.3`

- Remove empty folder after removing files

## Note

The utility has been only tested on macOS 10.14.5. However the standard library is used so it should work
on other platforms as well.

## Contribute

Bug or feature requests are more than welcome, I will try to followup on them.
Also PRs are welcome in case you feel you want to fix/add features.
