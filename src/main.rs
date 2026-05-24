use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "devops",
    about = "量潮DevOps工具 — 发布状态查询",
    version
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 查看发布状态
    ReleaseStatus,
}

fn repo_path() -> PathBuf {
    std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::ReleaseStatus => {
            devops::commands::release_status::run(&repo_path())
                .map_err(|e| format!("{}", e))
        }
    };

    if let Err(e) = result {
        eprintln!("错误: {}", e);
        std::process::exit(1);
    }
}
