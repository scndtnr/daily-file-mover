use clap::{Args, Parser, Subcommand};

/// コマンドライン引数のパース用構造体
#[derive(Debug, Clone, Parser)]
#[clap(
    name = "Daily File Mover",
    version = "0.1.0",
    author = "zumi",
    about = "A CLI tool that moves and renames daily files. It also creates destination directories and performs file relocation based on specified time intervals."
)]
#[clap(propagate_version = true)]
pub(crate) struct Opts {
    #[clap(subcommand)]
    pub(crate) command: Commands,
}

impl Opts {
    pub(super) fn command(&self) -> &Commands {
        &self.command
    }
}

#[derive(Debug, Clone, Subcommand)]
pub(crate) enum Commands {
    /// 新規の日報ファイルを作成する
    CreateNewDailyReport(CreateNewDailyReportArgs),
    /// ファイルを日次ディレクトリ配下に格納する
    Pack(PackArgs),
    /// 日次ディレクトリ配下からファイルを取り出す
    Extract(ExtractArgs),
    /// 設定情報を出力する
    Config,
}

#[derive(Debug, Clone, Args)]
pub(super) struct CreateNewDailyReportArgs {
    /// 新規作成する日報ファイルの親ディレクトリを指定する。
    /// 指定しない場合、設定ファイルを参照する。
    #[arg(long)]
    pub(super) path: Option<String>,

    /// テンプレートファイルのパスを指定する。
    /// 指定しない場合、設定ファイルを参照する。
    #[arg(long)]
    pub(super) template: Option<String>,

    /// 作成対象日付を指定する。
    /// 指定しない場合、実行時当日とする。
    #[arg(long)]
    pub(super) date: Option<String>,

    #[clap(short, long, help = "仮実行フラグ")]
    pub(super) dry_run: bool,
}

#[derive(Debug, Clone, Args)]
pub(super) struct PackArgs {
    pub(super) src: String,
    pub(super) dst: String,
    #[clap(short, long, help = "仮実行フラグ")]
    pub(super) dry_run: bool,
}

#[derive(Debug, Clone, Args)]
pub(super) struct ExtractArgs {
    pub(super) src: String,
    pub(super) dst: String,
    #[clap(short, long, help = "仮実行フラグ")]
    pub(super) dry_run: bool,
}
