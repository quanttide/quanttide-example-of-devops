use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "qtcloud-devops",
    about = "量潮DevOps工具 — Release 发布管理",
    version
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 发布 Release
    ///
    /// 默认行为：创建 Git 标签并推送 + GitHub Release（仓库从 git remote 自动检测）。
    /// --tag-only：仅打标签，跳过 GitHub Release。
    /// --release-only：仅为已有标签创建 GitHub Release（跳过标签创建）。
    Release {
        /// 版本号（如 v0.1.0）
        #[arg(long, short = 'V')]
        version: String,

        /// CHANGELOG.md 路径
        #[arg(long, default_value = "CHANGELOG.md")]
        changelog: String,

        /// 仅检查，不执行
        #[arg(long)]
        dry_run: bool,

        /// 仅创建 Git 标签
        #[arg(long)]
        tag_only: bool,

        /// 仅创建 GitHub Release（需要标签已存在）
        #[arg(long)]
        release_only: bool,

        /// 跳过确认提示，直接发布
        #[arg(long, short = 'y')]
        yes: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Release {
            version,
            changelog,
            dry_run,
            tag_only,
            release_only,
            yes: _yes,
        } => {
            if tag_only && release_only {
                eprintln!("错误: --tag-only 和 --release-only 不能同时使用");
                std::process::exit(1);
            }

            if dry_run {
                println!("[预览] Release 命令 (待实现)");
                println!("  version:     {}", version);
                println!("  changelog:   {}", changelog);
                println!("  dry_run:     {}", dry_run);
                println!("  tag_only:    {}", tag_only);
                println!("  release_only: {}", release_only);
                return;
            }

            println!("Release 命令待实现: version={}", version);
        }
    }
}
