mod options;

use self::options::{Commands, Opts};
use anyhow::{Context, Result};
use clap::Parser;

#[derive(Debug, Clone)]
pub(super) struct Cui {
    opts: Opts,
}

impl Cui {
    pub(super) fn new() -> Self {
        Self {
            opts: Opts::parse(),
        }
    }

    pub(super) fn process(&self) -> Result<()> {
        match self.opts.command().clone() {
            Commands::CreateNewDailyReport(args) => {
                println!("{:#?}", args);
                crate::commands::create_new_daily_report(
                    args.path,
                    args.template,
                    args.date,
                    args.dry_run,
                )
                .with_context(|| "Failed to create new daily report file")?;
                Ok(())
            }
            Commands::Pack(args) => {
                println!("{:#?}", args);
                crate::commands::pack_daily_files(args.src, args.dst, args.dry_run)
                    .with_context(|| "Failed to pack daily files")?;
                Ok(())
            }
            Commands::Extract(args) => {
                println!("{:#?}", args);
                crate::commands::extract_daily_files(args.src, args.dst, args.dry_run)
                    .with_context(|| "Failed to extract daily files")?;
                Ok(())
            }
            Commands::Config => {
                crate::commands::print_config();
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // テスト用のヘルパー関数
    fn create_test_cui_with_config() -> Cui {
        use crate::cui::options::{Commands, Opts};
        Cui {
            opts: Opts {
                command: Commands::Config,
            },
        }
    }

    #[test]
    fn test_process_method_returns_result_ok_for_config() {
        // process()メソッドがResult<(), anyhow::Error>を返すことを確認
        let cui = create_test_cui_with_config();

        // Config コマンドの場合は成功するはず
        let result: Result<(), anyhow::Error> = cui.process();

        // Config コマンドは正常に実行されるはず
        assert!(result.is_ok());
    }
}
