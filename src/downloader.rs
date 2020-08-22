use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::fs::{File};

use eyre::{eyre, Result};
use indicatif::{ProgressBar, ProgressStyle};

pub struct Downloader {
  base_url: String,
}

impl Downloader {
  pub fn new(base_url: &str) -> Self {
    Self {
      base_url: String::from(base_url),
    }
  }

  fn get_filename<'a>(&self, url: &'a str) -> Result<&'a str> {
    let slash_pos = match url.rfind("/") {
      Some(pos) => pos,
      None => return Err(eyre!("No filename in url."))
    };
    Ok(&url[slash_pos+1..])
  }

  pub fn download(&self, path: &str) -> Result<String> {
    let url = format!("{}/{}", self.base_url, path);

    let resp = reqwest::get(&url)?;
    let headers = resp.headers();

    let total_size: u64 = match headers.get("Content-Length") {
      Some(h) => h.to_str()?.parse()?,
      None => return Err(eyre!("No Content-Length header from server while downloading."))
    };

    let pb = ProgressBar::new(total_size);
    pb.set_style(
      ProgressStyle::default_bar()
        .template(" |{bar:40.white}| {bytes}/{total_bytes} ({eta})")
        .progress_chars("█▉▊▋▌▍▎▏ "),
    );

    let mut downloaded = 0u64;
    const CHUNK_SIZE: usize = 512 * 1024;
    let mut buffer = [0; CHUNK_SIZE];

    let filename = self.get_filename(&url)?;
    let fp = File::create(&filename)?;
    let mut reader = BufReader::new(resp);
    let mut writer = BufWriter::new(fp);

    while downloaded < total_size {
      let read_cnt = reader.read(&mut buffer)?;
      downloaded += read_cnt as u64;
      writer.write_all(&buffer[..read_cnt])?;
      pb.set_position(downloaded);
    }

    Ok(filename.to_string())
  }
}
