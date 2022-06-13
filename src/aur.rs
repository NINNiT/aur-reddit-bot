use log::info;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AurApiResRoot {
    pub resultcount: i64,
    pub results: Vec<AurApiResResult>,
    #[serde(rename = "type")]
    pub type_field: String,
    pub version: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AurApiResResult {
    #[serde(rename = "Conflicts")]
    pub conflicts: Option<Vec<String>>,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "FirstSubmitted")]
    pub first_submitted: i64,
    #[serde(rename = "ID")]
    pub id: i64,
    #[serde(rename = "Keywords")]
    pub keywords: Vec<String>,
    #[serde(rename = "LastModified")]
    pub last_modified: i64,
    #[serde(rename = "License")]
    pub license: Vec<String>,
    #[serde(rename = "Maintainer")]
    pub maintainer: String,
    #[serde(rename = "Depends")]
    pub depends: Option<Vec<String>>,
    #[serde(rename = "MakeDepends")]
    pub make_depends: Option<Vec<String>>,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "NumVotes")]
    pub num_votes: i64,
    #[serde(rename = "OutOfDate")]
    pub out_of_date: Option<i64>,
    #[serde(rename = "PackageBase")]
    pub package_base: String,
    #[serde(rename = "PackageBaseID")]
    pub package_base_id: i64,
    #[serde(rename = "Popularity")]
    pub popularity: f64,
    #[serde(rename = "Provides")]
    pub provides: Vec<String>,
    #[serde(rename = "URL")]
    pub url: String,
    #[serde(rename = "URLPath")]
    pub urlpath: String,
    #[serde(rename = "Version")]
    pub version: String,
}

pub async fn fetch_pkgdata_from_aur(pkg_name: String) -> Result<AurApiResRoot, reqwest::Error> {
    let url = format!(
        "https://aur.archlinux.org/rpc/?v=5&type=info&arg[]={}",
        pkg_name
    )
    .to_string();

    let response = reqwest::get(&url).await?.json::<AurApiResRoot>().await?;
    info!("Fetched data from {}", &url);

    Ok(response)
}
