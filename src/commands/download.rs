use super::Command;
use crate::vt;
use crate::vt::models::PackageInfo;
use anyhow::Result;
use clap::Args;
use std::fs::File;
use std::path::PathBuf;

/// Download package contents by package file or share code
#[derive(Args)]
#[command(visible_aliases = ["dl"])]
pub struct Download {
    /// Package JSON file
    #[arg(
        short,
        long,
        conflicts_with = "share_code",
        required_unless_present = "share_code"
    )]
    packages: Option<PathBuf>,

    /// Share code
    #[arg(
        short,
        long,
        conflicts_with = "packages",
        required_unless_present = "packages"
    )]
    share_code: Option<String>,

    /// Minecraft version
    #[arg(short, long)]
    version: Option<String>,

    /// Output directory
    #[arg(short, long, default_value = "vt-packages")]
    out_dir: PathBuf,
}

impl Command for Download {
    fn run(&self) -> Result<()> {
        let info = self.get_package_info()?;

        let version = self
            .version
            .as_ref()
            .or(info.version.as_ref())
            .ok_or_else(|| {
                anyhow::anyhow!("no version is specified in the project info or given as parameter")
            })?;

        vt::download_archive(version, &info, &self.out_dir)
    }
}

impl Download {
    fn get_package_info(&self) -> Result<PackageInfo> {
        if let Some(packages) = &self.packages {
            let mut f = File::open(packages)?;
            return Ok(serde_json::from_reader(&mut f)?);
        }

        if let Some(code) = &self.share_code {
            return vt::decode_share_code(code);
        }

        panic!("neither package not share code is passed; this should not happen")
    }
}
