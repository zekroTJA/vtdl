use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct ZipPacksResponse {
    pub status: String,
    pub link: String,
}

pub type Packages = HashMap<String, Vec<String>>;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ShareCodeType {
    Datapacks,
    CraftingTweaks,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PackageInfo {
    #[serde(rename = "type")]
    pub typ: ShareCodeType,
    pub version: Option<String>,
    pub packs: Packages,
}
