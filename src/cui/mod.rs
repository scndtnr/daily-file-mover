use clap::Parser;

use self::options::{Commands, Opts};

mod options;

#[derive(Debug, Clone)]
pub(super) struct Cui {
    opts: Opts,
}

impl Cui {
    pub(super) async fn new() -> Self {
        Self {
            opts: Opts::parse(),
        }
    }

    pub(super) async fn process(&self) {
        match self.opts.command().clone() {
            Commands::Pack(args) => {
                println!("{:#?}", args);
                crate::commands::pack_daily_files(args.src, args.dst, args.dry_run)
                    .expect("Fail to pack daily files.");
            }
            Commands::Extract(args) => {
                println!("{:#?}", args);
                todo!("ファイル抽出機能を実装する")
            }
            Commands::Config => crate::commands::print_config(),
        }
    }
}
