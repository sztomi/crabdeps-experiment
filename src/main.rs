#[macro_use]
extern crate clap;
#[macro_use]
extern crate default_env;
extern crate dirs;

use eyre::Result;

mod downloader;
mod utils;
mod cache;

use downloader::Downloader;
use cache::Cache;

fn main() -> Result<()> {
  let args = clap_app!(app =>
    (version: "1.0")
    (@arg output_dir: -o --output
      default_value("dependencies")
      +takes_value "Where to extract the dependencies. Can be an absolute or relative path.")
    (@arg url: -u --url
      default_value("https://artifacts.plex.tv/conan-bundles")
      "Where to download dependencies from")
    (@arg product: index(1))
    (@arg sha: index(2) "The complete plex-conan SHA (shortened won't work)")
    (@arg config: index(3))
  ).get_matches();

  let out_dir = args.value_of("output_dir").unwrap();
  let url = args.value_of("url").unwrap();
  let product = args.value_of("product").unwrap();
  let sha = args.value_of("sha").unwrap();
  let config = args.value_of("config").unwrap();

  let cache = Cache::new(&product)?;
  let downloader = Downloader::new(url);
  let tarball = downloader.download(&format!("{}/{}/{}.tar.zst", product, sha, config))?;

  utils::untar(&tarball, &out_dir.into())?;

  Ok(())
}
