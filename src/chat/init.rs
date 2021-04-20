use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use async_std::io;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
pub     id: crate::db::types::Id,
pub     public_key: fcpv2::types::SSK,
pub     private_key: fcpv2::types::SSK,
}

pub fn init_config(path: &Path) -> io::Result<File> {
    let config = File::create(&path).unwrap();
    return Ok(config);
}

pub fn update_config(path: &Path, val: &str) -> io::Result<()> {
    let mut f = std::fs::File::create(&path)?;
    log::debug!("write \n {}", &val);
    f.write_all(&val.as_bytes())?;
    Ok(())
}
