mod config;
mod handler;
mod model;
mod server;
mod usecase;

use anyhow::bail;
use server::run_server;

use crate::model::{KanaForm, PI};

#[derive(Debug, clap::Parser)]
struct Cli {
    /// Print kana in katakana
    #[arg(long)]
    katakana: bool,
    /// Print katakana in half-width
    #[arg(long)]
    halfwidth: bool,
    /// Start HTTP server
    #[arg(long)]
    server: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = <Cli as clap::Parser>::parse();
    if cli.server {
        Ok(run_server().await?)
    } else {
        let kana_form = match (cli.katakana, cli.halfwidth) {
            (false, false) => KanaForm::Hiragana,
            (false, true) => bail!("--halfwidth is only valid with --katakana"),
            (true, false) => KanaForm::Katakana,
            (true, true) => KanaForm::HalfwidthKana,
        };
        let pi = PI::gen(kana_form).await?;
        println!("{}", serde_json::to_string(&pi)?);
        Ok(())
    }
}
