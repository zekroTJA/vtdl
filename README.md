# vtdl

CLI tool to download packages from vanillatweaks.net

## Usage

```
$ vtdl help
CLI tool to download packages from vanillatweaks.net

Usage: vtdl <COMMAND>

Commands:
  get-packages  Decode a share code into a json object [aliases: gp]
  download      [aliases: dl]
  help          Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## `get-packages` Command

```
$ vtdl help get-packages
Decode a share code into a json object

Usage: vtdl get-packages [OPTIONS] <SHARE_CODE>

Arguments:
  <SHARE_CODE>  The share code

Options:
  -o, --out-file <OUT_FILE>  Store the output into a JSON file
  -h, --help                 Print help
```

## `download` Command

```
$ vtdl help get-packages
Download package contents by package file or share code

Usage: vtdl download [OPTIONS]

Options:
  -p, --packages <PACKAGES>      Package JSON file
  -s, --share-code <SHARE_CODE>  Share code
  -v, --version <VERSION>        Minecraft version
  -o, --out-dir <OUT_DIR>        Output directory [default: vt-packages]
  -h, --help                     Print help
```

## Install

You can either download the latest release builds form the [Releases page](https://github.com/zekrotja/vtdl/releases) or you can install it using cargo install.
```
cargo install --git https://github.com/zekrotja/vtdl
```