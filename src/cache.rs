use std::path::{Path, PathBuf};
use std::env;
use std::fs;
use std::collections::HashMap;

use dirs;
use eyre::{Result, eyre};

pub struct Cache {
  pub root: PathBuf,
  pub product: String,
  tarball_cache_count: u16,
  uncompressed_cache_count: u16,
  axx_file: PathBuf,
  access: HashMap<String, String>,
}

impl Cache {
  pub fn new(product: &str) -> Result<Self> {
    let mut cache_root = match env::var("PLEX_DEPS_CACHE_DIR") {
      Ok(val) => PathBuf::from(val),
      Err(_) => {
        let mut home = dirs::home_dir().unwrap();
        home.push(".plex_deps");
        home
      }
    };

    cache_root.push(&product);
    if !cache_root.exists() || !cache_root.is_dir() {
      match fs::create_dir_all(&cache_root) {
        Ok(_) => (),
        Err(_) => return Err(eyre!("Could not create cache dir {:?}", cache_root))
      }
    }

    Ok(Self {
      root: PathBuf::from(cache_root.as_path()),
      product: String::from(product),
      tarball_cache_count: match env::var("PLEX_DEPS_CACHE_TARBALL_COUNT") {
        Ok(val) => val.parse()?,
        Err(_) => 10u16
      },
      uncompressed_cache_count: match env::var("PLEX_DEPS_CACHE_UNCOMPRESSED_COUNT") {
        Ok(val) => val.parse()?,
        Err(_) => 10u16
      },
      axx_file: {
        let mut fname = PathBuf::from(cache_root.as_path());
        fname.push("access.json");
        fname
      },
      access: HashMap::new()
    })
  }
}