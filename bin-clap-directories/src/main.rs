use clap::{crate_version, App};
use directories::ProjectDirs;
use serde::Deserialize;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::process::exit;

/// The reverse domain name notation of the application, excluding the
/// organization or application name itself. An empty string can be passed if no
/// qualifier should be used (only affects macOS).
const QUALIFIER: &'static str = "";

/// The name of the organization that develops this application, or for which
/// the application is developed. An empty string can be passed if no
/// organization should be used (only affects macOS and Windows).
const ORGANIZATION: &'static str = "";

/// The name of the application itself.
const APPLICATION: &'static str = env!("CARGO_PKG_NAME");

#[derive(Default, Deserialize)]
struct Config {}

impl Config {
    fn from_path<P: AsRef<Path>>(path: P) -> io::Result<Config> {
        let mut buffer = String::new();
        File::open(path)?.read_to_string(&mut buffer)?;
        match toml::from_str::<Config>(&buffer) {
            Ok(config) => Ok(config),
            Err(e) => Err(io::Error::new(io::ErrorKind::InvalidData, e)),
        }
    }
}

fn main() {
    let _matches = App::new(APPLICATION)
        .version(&crate_version!()[..])
        .get_matches();

    let _config = match ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION) {
        Some(dirs) => {
            let mut path = PathBuf::new();
            path.push(dirs.config_dir());
            path.push("config.toml");
            match Config::from_path(&path) {
                Ok(config) => config,
                Err(e) => {
                    if e.kind() == io::ErrorKind::NotFound {
                        Config::default()
                    } else {
                        eprintln!("cannot load {}: {}", path.display(), e);
                        exit(1);
                    }
                }
            }
        }
        None => Config::default(),
    };
}
