use std::fs;
use std::path::Path;

use eyre::Result;
use tar::Archive;
use zstd::Decoder;

pub fn untar<P: AsRef<Path>>(tarball: P, out_dir: P) -> Result<()> {
  // if out_dir.is_dir() {
  //   out_dir.remove_dir_all()?;
  // }

  // fs::create_dir_all(out_dir)?;

  {
    let tarball = fs::File::open(&tarball)?;
    let tar = Decoder::new(tarball)?;
    let mut archive = Archive::new(tar);
    archive.unpack(&out_dir)?;
  }

  Ok(())
}
