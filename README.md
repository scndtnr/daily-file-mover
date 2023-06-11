# daily-file-mover

指定期間の日次ファイルの移動や名称変更、移動先ディレクトリの作成を行うCLIツール

```sh
A CLI tool that moves and renames daily files. It also creates destination directories and performs file relocation based on specified time intervals.

Usage: daily-file-mover.exe <COMMAND>

Commands:
  pack     ファイルを日次ディレクトリ配下に格納する
  extract  日次ディレクトリ配下からファイルを取り出す
  config   設定情報を出力する
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```
