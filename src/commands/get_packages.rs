use super::Command;
use crate::vt;
use anyhow::Result;
use clap::Args;
use std::fs::File;
use std::io;
use std::path::PathBuf;

/// Decode a share code into a json object
#[derive(Args)]
#[command(visible_aliases = ["gp"])]
pub struct GetPackages {
    /// The share code
    share_code: String,

    /// Store the output into a JSON file
    #[arg(short, long)]
    out_file: Option<PathBuf>,
}

impl Command for GetPackages {
    fn run(&self) -> Result<()> {
        let res = vt::decode_share_code(&self.share_code)?;

        let writer: Box<dyn io::Write> = match &self.out_file {
            Some(p) => Box::new(File::open(p)?),
            None => Box::new(io::stdout()),
        };

        serde_json::to_writer_pretty(writer, &res)?;

        Ok(())
    }
}
