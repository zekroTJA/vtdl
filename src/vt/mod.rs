use crate::vt::models::{PackageInfo, ShareCodeType, ZipPacksResponse};
use anyhow::Result;
use reqwest::blocking::Client;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, BufWriter, Cursor, Write};
use std::path::Path;
use zip::ZipArchive;
use zip::result::ZipError;

pub mod models;

const BASE_ADDRESS: &str = "https://vanillatweaks.net";
const SHARE_PREFIX: &str = "https://vanillatweaks.net/share#";

pub fn decode_share_code(share_code: &str) -> Result<PackageInfo> {
    let share_code = strip_sharecode(share_code);

    let info = reqwest::blocking::get(format!(
        "{BASE_ADDRESS}/assets/server/sharecode.php?code={share_code}"
    ))?
    .error_for_status()?
    .json()?;

    Ok(info)
}

fn get_archive_link(version: &str, info: &PackageInfo) -> Result<ZipPacksResponse> {
    let packs_str = serde_json::to_string(&info.packs)?;

    let mut form = HashMap::new();
    form.insert("version", version);
    form.insert("packs", &packs_str);

    let url = match info.typ {
        ShareCodeType::Datapacks => format!("{BASE_ADDRESS}/assets/server/zipdatapacks.php"),
        ShareCodeType::CraftingTweaks => {
            format!("{BASE_ADDRESS}/assets/server/zipcraftingtweaks.php")
        }
    };

    let res: ZipPacksResponse = Client::new()
        .post(url)
        .form(&form)
        .send()?
        .error_for_status()?
        .json()?;

    if res.status != "success" {
        return Err(anyhow::anyhow!("status was not successful: {}", res.status));
    }

    Ok(res)
}

pub fn download_archive(version: &str, info: &PackageInfo, out_dir: &Path) -> Result<()> {
    let link = get_archive_link(version, info)?;

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

    match info.typ {
        ShareCodeType::Datapacks => extract_archive(&buf, out_dir)?,
        ShareCodeType::CraftingTweaks => store_file(&buf, &out_dir.join("craftingtweaks.zip"))?,
    }

    Ok(())
}

fn extract_archive(buf: &[u8], out_dir: &Path) -> Result<(), ZipError> {
    let mut archive = ZipArchive::new(Cursor::new(&buf))?;
    archive.extract(out_dir)
}

fn store_file(buf: &[u8], out_dir: &Path) -> Result<(), io::Error> {
    let mut f = File::create(out_dir)?;
    f.write_all(buf)
}

fn strip_sharecode(code: &str) -> &str {
    match code.strip_prefix(SHARE_PREFIX) {
        Some(v) => v,
        None => code,
    }
}
