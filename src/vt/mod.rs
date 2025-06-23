use crate::vt::models::{PackageInfo, Packages, ZipPacksResponse};
use anyhow::Result;
use reqwest::blocking::Client;
use std::collections::HashMap;
use std::fs;
use std::io::{BufWriter, Cursor};
use std::path::Path;
use zip::ZipArchive;

pub mod models;

const BASE_ADDRESS: &str = "https://vanillatweaks.net";

pub fn decode_share_code(share_code: &str) -> Result<PackageInfo> {
    Ok(reqwest::blocking::get(format!(
        "{BASE_ADDRESS}/assets/server/sharecode.php?code={share_code}"
    ))?
    .error_for_status()?
    .json()?)
}

fn get_archive_link(version: &str, packages: Packages) -> Result<ZipPacksResponse> {
    let packs_str = serde_json::to_string(&packages)?;

    let mut form = HashMap::new();
    form.insert("version", version);
    form.insert("packs", &packs_str);

    let res: ZipPacksResponse = Client::new()
        .post(format!("{BASE_ADDRESS}/assets/server/zipdatapacks.php"))
        .form(&form)
        .send()?
        .error_for_status()?
        .json()?;

    if res.status != "success" {
        return Err(anyhow::anyhow!("status was not successful: {}", res.status));
    }

    Ok(res)
}

pub fn download_archive(version: &str, packages: Packages, out_dir: &Path) -> Result<()> {
    let link = get_archive_link(version, packages)?;

    let mut buf = vec![];

    {
        let mut wr = BufWriter::new(&mut buf);
        Client::new()
            .get(format!("{}/{}", BASE_ADDRESS, link.link))
            .send()?
            .copy_to(&mut wr)?;
    }

    if !out_dir.exists() {
        fs::create_dir_all(out_dir)?;
    }

    let mut archive = ZipArchive::new(Cursor::new(&buf))?;
    archive.extract(out_dir)?;
    Ok(())
}
