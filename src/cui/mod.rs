mod options;

use self::options::{Commands, Opts};
use clap::Parser;

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
                crate::commands::extract_daily_files(args.src, args.dst, args.dry_run)
                    .expect("Fail to extract daily files.");
            }
            Commands::Config => crate::commands::print_config(),
        }
    }
}
