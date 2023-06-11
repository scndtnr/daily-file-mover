# daily-file-mover

指定期間の日次ファイルの移動や名称変更、移動先ディレクトリの作成を行うCLIツール
また、テンプレートファイルを参照した新規の日報ファイルの作成も行う。

# help option

## daily-file-mover --help

```sh
A CLI tool that moves and renames daily files. It also creates destination directories and performs file relocation based on specified time intervals.

Usage: daily-file-mover.exe <COMMAND>

Commands:
  create-new-daily-report  新規の日報ファイルを作成する
  pack                     ファイルを日次ディレクトリ配下に格納する
  extract                  日次ディレクトリ配下からファイルを取り出す
  config                   設定情報を出力する
  help                     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## daily-file-mover create-new-daily-report --help

```sh
新規の日報ファイルを作成する

Usage: daily-file-mover.exe create-new-daily-report [OPTIONS]

Options:
      --path <PATH>          新規作成する日報ファイルの親ディレクトリを指定する。 指定しない場合、設定ファイルを参照する。
      --template <TEMPLATE>  テンプレートファイルのパスを指定する。 指定しない場合、設定ファイルを参照する。
      --date <DATE>          作成対象日付を指定する。 指定しない場合、実行時当日とする。
  -d, --dry-run              仮実行フラグ
  -h, --help                 Print help
  -V, --version              Print version
```

## daily-file-mover pack --help

```sh
ファイルを日次ディレクトリ配下に格納する

Usage: daily-file-mover.exe pack [OPTIONS] <SRC> <DST>

Arguments:
  <SRC>
  <DST>

Options:
  -d, --dry-run  仮実行フラグ
  -h, --help     Print help
  -V, --version  Print version
```

## daily-file-mover extract --help

```sh
日次ディレクトリ配下からファイルを取り出す

Usage: daily-file-mover.exe extract [OPTIONS] <SRC> <DST>

Arguments:
  <SRC>
  <DST>

Options:
  -d, --dry-run  仮実行フラグ
  -h, --help     Print help
  -V, --version  Print version
```

## daily-file-mover config --help

```sh
設定情報を出力する

Usage: daily-file-mover.exe config

Options:
  -h, --help     Print help
  -V, --version  Print version
```